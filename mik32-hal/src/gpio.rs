//! Ввод/вывод общего назначения

use core::{convert::Infallible, marker::PhantomData};

/// Расширение для разделения `GPIO` на независимые выводы и регистры
pub trait GpioExt {
    /// Части, на которые разделится `GPIO`
    type Parts;

    /// Разделениe `GPIO` на независимые выводы и регистры
    fn split(self) -> Self::Parts;
}

/// Режим входа (тип-состояние)
#[derive(Default)]
pub struct Input<PULL = Floating>(PhantomData<PULL>);

/// Вход без подтяжки (тип-состояние)
#[derive(Default)]
pub struct Floating;

/// Вход с подтяжкой к земле (тип-состояние)
#[derive(Default)]
pub struct PullDown;

/// Вход с подтяжкой к питанию (тип-состояние)
#[derive(Default)]
pub struct PullUp;

/// Режим выхода (тип-состояние)
#[derive(Default)]
pub struct Output;

/// Последовательный режим (type state)
#[derive(Default)]
pub struct Serial;

/// Таймер или последовательный режим (type state)
#[derive(Default)]
pub struct TimerSerial;

/// Аналоговой режим (type state)
#[derive(Default)]
pub struct Analog;

pub enum Mode {
    Input = 0b100,
    Output = 0b000,
    Serial = 0b001,
    TimerSerial = 0b010,
    Analog = 0b011,
}

mod sealed {
    pub trait PinMode: Default {
        const MODE: super::Mode;
        const PULL: Option<bool> = None;
    }
}

pub(crate) use sealed::PinMode;

impl PinMode for Input<Floating> {
    const MODE: Mode = Mode::Input;
}

impl PinMode for Input<PullUp> {
    const MODE: Mode = Mode::Input;
    const PULL: Option<bool> = Some(true);
}

impl PinMode for Input<PullDown> {
    const MODE: Mode = Mode::Input;
    const PULL: Option<bool> = Some(false);
}

impl PinMode for Output {
    const MODE: Mode = Mode::Output;
}

impl PinMode for Serial {
    const MODE: Mode = Mode::Serial;
}

impl PinMode for TimerSerial {
    const MODE: Mode = Mode::TimerSerial;
}

impl PinMode for Analog {
    const MODE: Mode = Mode::Analog;
}

/// Шаблонный тип вывода
/// - `P` это номер порта `GPIO`: 0, 1 или 2
/// - `N` это номер вывода: от 0 до 15 для `GPIO0 и 1` и от 0 до 7 для `GPIO2`
pub struct Pin<const P: u8, const N: u8, MODE = Input<Floating>>(PhantomData<MODE>);

use crate::pac::PadConfig;
pub use crate::pac::pad_config::pad0_ds::Port0_0 as DS;

const fn gpiox<const P: u8>() -> *const crate::pac::gpio_0::RegisterBlock {
    match P {
        0 => crate::pac::Gpio0::ptr(),
        1 => crate::pac::Gpio1::ptr() as _,
        2 => crate::pac::Gpio2::ptr() as _,
        _ => unreachable!(),
    }
}

impl<const P: u8, const N: u8, M> Pin<P, N, M> {
    /// Настраивает вывод для работы в режиме входа без подтяжки
    #[inline]
    pub fn into_floating_input(self, pad_config: &PadConfig) -> Pin<P, N, Input<Floating>> {
        self.into_mode(pad_config)
    }

    /// Настраивает вывод для работы в режиме входа с подтяжкой к земле
    #[inline]
    pub fn into_pull_down_input(self, pad_config: &PadConfig) -> Pin<P, N, Input<PullDown>> {
        self.into_mode(pad_config)
    }

    /// Настраивает вывод для работы в режиме входа с подтяжкой к питанию
    #[inline]
    pub fn into_pull_up_input(self, pad_config: &PadConfig) -> Pin<P, N, Input<PullUp>> {
        self.into_mode(pad_config)
    }

    /// Настраивает вывод для работы в режиме выхода
    #[inline]
    pub fn into_output(self, pad_config: &PadConfig) -> Pin<P, N, Output> {
        self.into_mode(pad_config)
    }

    /// Настраивает вывод для работы в последовательном режиме
    #[inline]
    pub fn into_serial(self, pad_config: &PadConfig) -> Pin<P, N, Serial> {
        self.into_mode(pad_config)
    }

    /// Настраивает вывод для работы в таймере или последовательном режиме
    #[inline]
    pub fn into_timer_serial(self, pad_config: &PadConfig) -> Pin<P, N, TimerSerial> {
        self.into_mode(pad_config)
    }

    /// Настраивает вывод для работы в аналоговом режиме
    #[inline]
    pub fn into_analog(self, pad_config: &PadConfig) -> Pin<P, N, Analog> {
        self.into_mode(pad_config)
    }

    #[inline]
    pub fn into_mode<MODE: PinMode>(mut self, pad_config: &PadConfig) -> Pin<P, N, MODE> {
        self.mode::<MODE>(pad_config);
        Pin(PhantomData)
    }

    fn mode<MODE: PinMode>(&mut self, pad_config: &PadConfig) {
        let pad_cfg = match P {
            0 => pad_config.pad0_cfg().as_ptr(),
            1 => pad_config.pad1_cfg().as_ptr(),
            2 => pad_config.pad2_cfg().as_ptr(),
            _ => unreachable!(),
        };
        let pad_cfg = unsafe { &mut (*pad_cfg) };
        let mask = 0b11 << (N * 2);
        let value = (MODE::MODE as u32 & 0b11) << (N * 2);
        *pad_cfg = (*pad_cfg & !mask) | value;

        let gpio = unsafe { &(*gpiox::<P>()) };
        match MODE::MODE {
            Mode::Input => {
                gpio.direction_in().modify(|_, w| unsafe { w.bits(1 << N) });
                if let Some(pull) = MODE::PULL {
                    let value: u32 = if pull { 0b01 } else { 0b10 };
                    let pad_pupd = match P {
                        0 => pad_config.pad0_pupd().as_ptr(),
                        1 => pad_config.pad1_pupd().as_ptr(),
                        2 => pad_config.pad2_pupd().as_ptr(),
                        _ => unreachable!(),
                    };
                    let pad_pupd = unsafe { &mut (*pad_pupd) };
                    *pad_pupd = value << (N * 2);
                }
            }
            Mode::Output => {
                gpio.direction_out()
                    .modify(|_, w| unsafe { w.bits(1 << N) });
            }
            _ => (),
        }
    }

    // Устанавливает нагрузочную способность вывода
    pub fn set_ds(&mut self, pad_config: &PadConfig, ds: DS) {
        let pad_ds = match P {
            0 => pad_config.pad0_ds().as_ptr(),
            1 => pad_config.pad1_ds().as_ptr(),
            2 => pad_config.pad2_ds().as_ptr(),
            _ => unreachable!(),
        };
        let pad_ds = unsafe { &mut (*pad_ds) };
        *pad_ds = (ds as u32) << (N * 2);
    }
}

pub use embedded_hal::digital::PinState;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

impl<const P: u8, const N: u8> ErrorType for Pin<P, N, Output> {
    type Error = Infallible;
}
impl<const P: u8, const N: u8, MODE> ErrorType for Pin<P, N, Input<MODE>> {
    type Error = Infallible;
}

impl<const P: u8, const N: u8> OutputPin for Pin<P, N, Output> {
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let gpio = unsafe { &(*gpiox::<P>()) };
        gpio.set().write(|w| unsafe { w.bits(1 << N) });

        Ok(())
    }
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let gpio = unsafe { &(*gpiox::<P>()) };
        gpio.clear().write(|w| unsafe { w.bits(1 << N) });

        Ok(())
    }
}

impl<const P: u8, const N: u8> StatefulOutputPin for Pin<P, N, Output> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        let gpio = unsafe { &(*gpiox::<P>()) };
        Ok(gpio.output().read().bits() & (1 << N) > 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|b| !b)
    }
}

impl<const P: u8, const N: u8, MODE> InputPin for Pin<P, N, Input<MODE>> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let gpio = unsafe { &(*gpiox::<P>()) };
        Ok(gpio.set().read().bits() & (1 << N) > 0)
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_high().map(|b| !b)
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $port_id:expr, [
        $($PXi:ident: ($pxi:ident, $pin_number:expr),)+
    ]) => {
        /// `GPIO`
        pub mod $gpiox {
            use core::marker::PhantomData;
            use crate::pac::{$GPIOX, Pm};
            use super::*;

            /// Части `GPIO`
            pub struct Parts {
                $(
                    /// Вывод
                    pub $pxi: $PXi,
                )+
            }

            $(
                pub type $PXi<MODE = Input<Floating>> = Pin<$port_id, $pin_number, MODE>;
            )+

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    let pm = unsafe { Pm::steal() };
                    pm.clk_apb_p_set().modify(|_, w| w.$gpiox().enable());

                    Parts {
                        $(
                            $pxi: Pin::<$port_id, $pin_number, Input<Floating>>(PhantomData),
                        )+
                    }
                }
            }
        }

        pub use $gpiox::{ $($PXi,)+ };
    }
}

gpio!(Gpio0, gpio_0, 0, [
    P0_0: (p0_0, 0),
    P0_1: (p0_1, 1),
    P0_2: (p0_2, 2),
    P0_3: (p0_3, 3),
    P0_4: (p0_4, 4),
    P0_5: (p0_5, 5),
    P0_6: (p0_6, 6),
    P0_7: (p0_7, 7),
    P0_8: (p0_8, 8),
    P0_9: (p0_9, 9),
    P0_10: (p0_10, 10),
    P0_11: (p0_11, 11),
    P0_12: (p0_12, 12),
    P0_13: (p0_13, 13),
    P0_14: (p0_14, 14),
    P0_15: (p0_15, 15),
]);

gpio!(Gpio1, gpio_1, 1, [
    P1_0: (p1_0, 0),
    P1_1: (p1_1, 1),
    P1_2: (p1_2, 2),
    P1_3: (p1_3, 3),
    P1_4: (p1_4, 4),
    P1_5: (p1_5, 5),
    P1_6: (p1_6, 6),
    P1_7: (p1_7, 7),
    P1_8: (p1_8, 8),
    P1_9: (p1_9, 9),
    P1_10: (p1_10, 10),
    P1_11: (p1_11, 11),
    P1_12: (p1_12, 12),
    P1_13: (p1_13, 13),
    P1_14: (p1_14, 14),
    P1_15: (p1_15, 15),
]);

gpio!(Gpio2, gpio_2, 2, [
    P2_0: (p2_0, 0),
    P2_1: (p2_1, 1),
    P2_2: (p2_2, 2),
    P2_3: (p2_3, 3),
    P2_4: (p2_4, 4),
    P2_5: (p2_5, 5),
    P2_6: (p2_6, 6),
    P2_7: (p2_7, 7),
]);
