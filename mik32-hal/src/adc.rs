//! Аналого–цифровой преобразователь

use core::marker::PhantomData;

use crate::{
    gpio::{Analog, Pin},
    pac::Adc as ADC,
    pac::Pm,
};

/// Абстракция `ADC`
pub struct Adc<'a> {
    adc: ADC,
    _marker: PhantomData<&'a ()>,
}

pub trait AdcChannel {
    const CHANNEL_NUMBER: u8;
}

impl<'a> Adc<'a> {
    /// Настраивает `ADC` и возвращает абстракцию
    pub fn new(adc: ADC) -> Self {
        let pm = unsafe { Pm::steal() };
        pm.clk_apb_p_set().modify(|_, w| w.analog_regs().enable());

        adc.adc_config().modify(|_, w| {
            w.en().enable();
            w.resetn().set_bit()
        });
        adc.adc_continuous().write(|w| w.continuous().set_bit());

        Self {
            adc,
            _marker: PhantomData,
        }
    }

    pub fn set_sah_time(&self, sah_time: u8) {
        self.adc
            .adc_config()
            .modify(|_, w| unsafe { w.sah_time().bits(sah_time) });
    }

    pub fn read(&self) -> u16 {
        self.adc.adc_value().read().value().bits()
    }

    pub fn select_channel<C: AdcChannel>(&mut self, _pin: &'a C) {
        self.adc
            .adc_config()
            .modify(|_, w| unsafe { w.sel().bits(C::CHANNEL_NUMBER) });
    }
}

macro_rules! adc_channels {
    ($($adc_channel:expr => ($port_id:expr, $pin_number:expr),)+) => {
        $(
        impl AdcChannel for Pin<$port_id, $pin_number, Analog> {
            const CHANNEL_NUMBER: u8 = $adc_channel;
        }
        )+
    };
}

adc_channels!(
    0 => (1, 5),
    1 => (1, 7),
    2 => (0, 2),
    3 => (0, 4),
    4 => (0, 7),
    5 => (0, 9),
    6 => (0, 11),
    7 => (0, 13),
);
