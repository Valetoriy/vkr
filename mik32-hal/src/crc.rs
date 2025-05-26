//! Вычисление циклической контрольной суммы

use core::convert::Infallible;

pub use crate::pac::crc::ctrl::{Fxor, Tot, Totr};
use crate::pac::{Crc, Pm};

/// Абстракция `CRC32`
pub struct Crc32 {
    crc: Crc,
}

impl Crc32 {
    /// Настраивает `CRC32` и возвращает абстракцию
    ///
    /// По умолчанию включена перестановка байтов входных данных
    pub fn new(crc: Crc, init_val: u32, poly: u32) -> Self {
        let pm = unsafe { Pm::steal() };
        pm.clk_ahb_set().modify(|_, w| w.crc32().enable());

        let mut crc = Self { crc };
        crc.set_poly(poly);
        crc.reset(init_val);
        crc.set_input_reverse(Tot::Bytes); // в противовес Little Endian
        crc
    }

    pub fn read(&self) -> nb::Result<u32, Infallible> {
        match self.is_busy() {
            true => Err(nb::Error::WouldBlock),
            false => Ok(self.crc.data().read().bits()),
        }
    }

    pub fn is_busy(&self) -> bool {
        self.crc.ctrl().read().busy().is_busy()
    }

    pub fn write(&mut self, val: u32) {
        self.crc.data().write(|w| unsafe { w.bits(val) });
    }

    /// Установка полинома
    pub fn set_poly(&mut self, val: u32) {
        self.crc.poly().write(|w| unsafe { w.bits(val) });
    }

    /// Включение перестановки битов/байтов входных данных
    pub fn set_input_reverse(&mut self, tot: Tot) {
        self.crc.ctrl().modify(|_, w| w.tot().variant(tot));
    }

    /// Включение перестановки битов/байтов выходных данных
    pub fn set_output_reverse(&mut self, totr: Totr) {
        self.crc.ctrl().modify(|_, w| w.totr().variant(totr));
    }

    /// Инверсия контрольной суммы
    pub fn set_output_inverse(&mut self, fxor: Fxor) {
        self.crc.ctrl().modify(|_, w| w.fxor().variant(fxor));
    }

    pub fn reset(&mut self, init_val: u32) {
        self.crc.ctrl().modify(|_, w| w.was().set_bit());
        self.crc.data().write(|w| unsafe { w.bits(init_val) });
        self.crc.ctrl().modify(|_, w| w.was().clear_bit());
    }
}
