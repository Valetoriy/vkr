//! Универсальный синхронный асинхронный приемник-передатчик (`USART`)

use core::marker::PhantomData;

use crate::clocks::Clocks;
use crate::gpio::Pin;
use crate::pac::usart_0::RegisterBlock;
use crate::pac::{Usart0, Usart1};
use crate::time::{Bps, U32Ext};

mod sealed {
    use crate::pac::usart_0::RegisterBlock;

    pub trait Instance {
        type TxPin;
        type RxPin;
        fn ptr() -> *const RegisterBlock;
        fn enable();
    }
}
use mik32_pac::Pm;
pub(crate) use sealed::Instance;

impl Instance for Usart0 {
    type TxPin = Pin<0, 6, crate::gpio::Serial>;
    type RxPin = Pin<0, 5, crate::gpio::Serial>;
    fn ptr() -> *const RegisterBlock {
        Usart0::ptr()
    }
    fn enable() {
        let pm = unsafe { Pm::steal() };
        pm.clk_apb_p_set().modify(|_, w| w.uart_0().enable());
    }
}
impl Instance for Usart1 {
    type TxPin = Pin<1, 9, crate::gpio::Serial>;
    type RxPin = Pin<1, 8, crate::gpio::Serial>;
    fn ptr() -> *const RegisterBlock {
        Usart1::ptr() as _
    }
    fn enable() {
        let pm = unsafe { Pm::steal() };
        pm.clk_apb_p_set().modify(|_, w| w.uart_1().enable());
    }
}

/// Ошибка `USART`
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Перезапись буфера `RXDATA`
    Overrun,
    /// Полученные данные не соответствуют конфигурации `USART`
    FrameFormat,
    /// Ошибочный бит чётности
    Parity,
    /// Ложные переключения на линии `RX`
    Noise,
    /// Другая ошибка
    Other,
}

use crate::pac::usart_0::control1::M as WordLength;

pub enum Parity {
    ParityNone,
    ParityEven,
    ParityOdd,
}
use crate::pac::usart_0::control2::Stop1 as StopBits;

pub struct Config {
    baudrate: Bps,
    wordlength: WordLength,
    parity: Parity,
    stopbits: StopBits,
}

impl Config {
    pub fn baudrate(mut self, baudrate: Bps) -> Self {
        self.baudrate = baudrate;
        self
    }

    pub fn wordlength(mut self, wordlength: WordLength) -> Self {
        self.wordlength = wordlength;
        self
    }

    pub fn wordlength_8bits(mut self) -> Self {
        self.wordlength = WordLength::_8bits;
        self
    }

    pub fn wordlength_9bits(mut self) -> Self {
        self.wordlength = WordLength::_8bits;
        self
    }

    pub fn wordlength_7bits(mut self) -> Self {
        self.wordlength = WordLength::_7bits;
        self
    }

    pub fn parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }

    pub fn parity_none(mut self) -> Self {
        self.parity = Parity::ParityNone;
        self
    }

    pub fn parity_even(mut self) -> Self {
        self.parity = Parity::ParityEven;
        self
    }

    pub fn parity_odd(mut self) -> Self {
        self.parity = Parity::ParityOdd;
        self
    }

    pub fn stopbits(mut self, stopbits: StopBits) -> Self {
        self.stopbits = stopbits;
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            baudrate: 115_200.bps(),
            wordlength: WordLength::_8bits,
            parity: Parity::ParityNone,
            stopbits: StopBits::_1bit,
        }
    }
}

impl From<Bps> for Config {
    fn from(baud: Bps) -> Self {
        Config::default().baudrate(baud)
    }
}

/// Абстракция последовательного интерфейса
pub struct Serial<USART: Instance> {
    pub tx: Tx<USART>,
    pub rx: Rx<USART>,
    #[allow(clippy::type_complexity)]
    pub token: ReleaseToken<USART, (Option<USART::TxPin>, Option<USART::RxPin>)>,
}

/// Передатчик
pub struct Tx<USART>(PhantomData<USART>);

/// Приёмник
pub struct Rx<USART>(PhantomData<USART>);

/// Хранит данные для высвобождения
pub struct ReleaseToken<USART, PINS> {
    usart: USART,
    pins: PINS,
}

impl<USART: Instance> Serial<USART> {
    /// Настраивает последовательный интерфейс и возвращает абстракцию
    pub fn new(
        usart: USART,
        pins: (USART::TxPin, USART::RxPin),
        config: impl Into<Config>,
        clocks: &Clocks,
    ) -> Self {
        Serial::_new(usart, (Some(pins.0), Some(pins.1)), config.into(), clocks)
    }

    /// Настраивает последовательный интерфейс и возвращает передатчик
    pub fn tx(
        usart: USART,
        tx_pin: USART::TxPin,
        config: impl Into<Config>,
        clocks: &Clocks,
    ) -> Tx<USART> {
        Serial::_new(usart, (Some(tx_pin), None), config.into(), clocks)
            .split()
            .0
    }

    /// Настраивает последовательный интерфейс и возвращает приёмник
    pub fn rx(
        usart: USART,
        rx_pin: USART::RxPin,
        config: impl Into<Config>,
        clocks: &Clocks,
    ) -> Rx<USART> {
        Serial::_new(usart, (None, Some(rx_pin)), config.into(), clocks)
            .split()
            .1
    }

    fn _new(
        usart: USART,
        pins: (Option<USART::TxPin>, Option<USART::RxPin>),
        config: Config,
        clocks: &Clocks,
    ) -> Self {
        let u = unsafe { &*USART::ptr() };
        u.control1().modify(|_, w| w.ue().disable());
        USART::enable();

        let div = clocks.apb_p_clk().raw() / config.baudrate.0;
        assert!(div >= 16, "невозможная скорость передачи данных");
        u.divider().write(|w| unsafe { w.brr().bits(div as u16) });
        u.control1().modify(|_, w| w.m().variant(config.wordlength));
        u.control2()
            .modify(|_, w| w.stop_1().variant(config.stopbits));

        let pce = !matches!(config.parity, Parity::ParityNone);
        if pce {
            let ps = matches!(config.parity, Parity::ParityOdd);
            u.control1().modify(|_, w| {
                w.pce().bit(pce);
                w.ps().bit(ps)
            });
        }

        u.control1().modify(|_, w| {
            w.te().enable();
            w.re().enable();
            w.ue().enable()
        });
        while !u.flags().read().teack().bit() {}
        while !u.flags().read().reack().bit() {}

        Serial {
            tx: Tx(PhantomData),
            rx: Rx(PhantomData),
            token: ReleaseToken { usart, pins },
        }
    }

    /// Возвращает заимствованные ресурсы
    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (USART, (Option<USART::TxPin>, Option<USART::RxPin>)) {
        (self.token.usart, self.token.pins)
    }

    /// Разделяет абстракцию последовательного интерфейса на отдельные каналы
    pub fn split(self) -> (Tx<USART>, Rx<USART>) {
        (self.tx, self.rx)
    }
}

impl<USART: Instance> Tx<USART> {
    /// Записывает 16-ти битное слово в `USART`
    pub fn write_u16(&mut self, word: u16) -> nb::Result<(), Error> {
        let usart = unsafe { &*USART::ptr() };

        if usart.flags().read().txe().bit_is_set() {
            usart.txdata().write(|w| unsafe { w.tdr().bits(word) });
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn write_u8(&mut self, word: u8) -> nb::Result<(), Error> {
        self.write_u16(word as u16)
    }

    pub fn bwrite_all_u16(&mut self, buffer: &[u16]) -> Result<(), Error> {
        for &w in buffer {
            nb::block!(self.write_u16(w))?;
        }
        Ok(())
    }

    pub fn bwrite_all_u8(&mut self, buffer: &[u8]) -> Result<(), Error> {
        for &w in buffer {
            nb::block!(self.write_u8(w))?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> nb::Result<(), Error> {
        let usart = unsafe { &*USART::ptr() };

        if usart.flags().read().tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn bflush(&mut self) -> Result<(), Error> {
        nb::block!(self.flush())
    }

    /// Возвращает `true` если регистр `TXDATA` пуст
    pub fn is_tx_empty(&self) -> bool {
        unsafe { (*USART::ptr()).flags().read().txe().bit_is_set() }
    }

    pub fn is_tx_complete(&self) -> bool {
        unsafe { (*USART::ptr()).flags().read().tc().bit_is_set() }
    }
}

impl<USART: Instance> core::fmt::Write for Tx<USART> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.bytes()
            .try_for_each(|c| nb::block!(self.write_u8(c)))
            .map_err(|_| core::fmt::Error)
    }
}

impl<USART: Instance> Rx<USART> {
    /// Считывает 16-ти битное слово из USART
    pub fn read_u16(&mut self) -> nb::Result<u16, Error> {
        let usart = unsafe { &*USART::ptr() };
        let flags = usart.flags().read();

        // Проверяем на наличие ошибок
        let err = if flags.pe().bit_is_set() {
            usart.flags().modify(|_, w| w.pe()._1());
            Some(Error::Parity)
        } else if flags.fe().bit_is_set() {
            usart.flags().modify(|_, w| w.fe()._1());
            Some(Error::FrameFormat)
        } else if flags.nf().bit_is_set() {
            usart.flags().modify(|_, w| w.nf()._1());
            Some(Error::Noise)
        } else if flags.ore().bit_is_set() {
            usart.flags().modify(|_, w| w.ore()._1());
            Some(Error::Overrun)
        } else {
            None
        };

        if let Some(err) = err {
            Err(nb::Error::Other(err))
        } else if flags.rxne().bit_is_set() {
            Ok(usart.rxdata().read().rdr().bits())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn read_u8(&mut self) -> nb::Result<u8, Error> {
        self.read_u16().map(|word16| word16 as u8)
    }

    /// Возвращает `true` если возведён флаг отсутствия активности на линии `RX`
    pub fn is_idle(&self) -> bool {
        unsafe { (*USART::ptr()).flags().read().idle().bit_is_set() }
    }

    /// Возвращает `true` если регистр `RXDATA` не пуст
    pub fn is_rx_not_empty(&self) -> bool {
        unsafe { (*USART::ptr()).flags().read().rxne().bit_is_set() }
    }
}

impl<USART: Instance> Serial<USART> {
    /// Возвращает `true` если возведён флаг отсутствия активности на линии `RX`
    pub fn is_idle(&self) -> bool {
        self.rx.is_idle()
    }

    /// Возвращает `true` если регистр `TXDATA` пуст
    pub fn is_tx_empty(&self) -> bool {
        self.tx.is_tx_empty()
    }

    /// Возвращает `true` если регистр `RXDATA` не пуст
    pub fn is_rx_not_empty(&self) -> bool {
        self.rx.is_rx_not_empty()
    }
}

impl<USART: Instance> core::fmt::Write for Serial<USART> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.tx.write_str(s)
    }
}

// embedded_hal_nb
use embedded_hal_nb::serial::ErrorKind;
use embedded_hal_nb::{serial, serial::ErrorType};

impl embedded_hal_nb::serial::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Overrun => ErrorKind::Overrun,
            Error::FrameFormat => ErrorKind::FrameFormat,
            Error::Parity => ErrorKind::Parity,
            Error::Noise => ErrorKind::Noise,
            Error::Other => ErrorKind::Other,
        }
    }
}

impl<USART: Instance> ErrorType for Tx<USART> {
    type Error = Error;
}

impl<USART: Instance> ErrorType for Rx<USART> {
    type Error = Error;
}

impl<USART: Instance> ErrorType for Serial<USART> {
    type Error = Error;
}

impl<USART: Instance> serial::Write<u8> for Tx<USART> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.write_u8(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.flush()
    }
}

impl<USART: Instance> serial::Write<u16> for Tx<USART> {
    fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
        self.write_u16(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.flush()
    }
}

impl<USART: Instance> serial::Read<u8> for Rx<USART> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.read_u8()
    }
}

impl<USART: Instance> serial::Read<u16> for Rx<USART> {
    fn read(&mut self) -> nb::Result<u16, Self::Error> {
        self.read_u16()
    }
}

impl<USART: Instance> serial::Write<u8> for Serial<USART> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.tx.write_u8(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush()
    }
}

impl<USART: Instance> serial::Write<u16> for Serial<USART> {
    fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
        self.tx.write_u16(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush()
    }
}

impl<USART: Instance> serial::Read<u8> for Serial<USART> {
    fn read(&mut self) -> nb::Result<u8, Error> {
        self.rx.read_u8()
    }
}

impl<USART: Instance> serial::Read<u16> for Serial<USART> {
    fn read(&mut self) -> nb::Result<u16, Error> {
        self.rx.read_u16()
    }
}

// embedded_io
impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<USART: Instance> embedded_io::ErrorType for Serial<USART> {
    type Error = Error;
}

impl<USART: Instance> embedded_io::ErrorType for Tx<USART> {
    type Error = Error;
}

impl<USART: Instance> embedded_io::ErrorType for Rx<USART> {
    type Error = Error;
}

impl<USART: Instance> embedded_io::Write for Tx<USART> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        nb::block!(serial::Write::write(self, buf[0])).unwrap();
        let mut count = 1;
        for byte in buf.iter().skip(1) {
            match serial::Write::write(self, *byte) {
                Ok(()) => count += 1,
                Err(nb::Error::WouldBlock) => break,
                Err(nb::Error::Other(o)) => return Err(o),
            }
        }
        Ok(count)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.bflush()
    }
}

impl<UART: Instance> embedded_io::Read for Rx<UART> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        buf[0] = nb::block!(serial::Read::read(self)).unwrap();
        let mut count = 1;
        for byte in buf.iter_mut().skip(1) {
            match serial::Read::read(self) {
                Ok(b) => {
                    *byte = b;
                    count += 1
                }
                Err(nb::Error::WouldBlock) => break,
                Err(nb::Error::Other(o)) => return Err(o),
            }
        }
        Ok(count)
    }
}

impl<USART: Instance> embedded_io::Write for Serial<USART> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.bflush()
    }
}

impl<UART: Instance> embedded_io::Read for Serial<UART> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.read(buf)
    }
}
