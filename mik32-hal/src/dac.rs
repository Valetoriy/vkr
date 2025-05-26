//! Цифро-аналоговый преобразователь

use crate::{
    clocks::Clocks,
    gpio::{Analog, Pin},
    pac::{Dac0, Dac1, Pm},
    time::Hertz,
};

mod sealed {
    use crate::pac::dac0::RegisterBlock;

    pub trait Instance {
        type DacPin;
        fn ptr() -> *const RegisterBlock;
    }
}
pub(crate) use sealed::Instance;

impl Instance for Dac0 {
    type DacPin = Pin<1, 12, Analog>;

    fn ptr() -> *const mik32_pac::dac0::RegisterBlock {
        Dac0::ptr()
    }
}
impl Instance for Dac1 {
    type DacPin = Pin<1, 13, Analog>;

    fn ptr() -> *const mik32_pac::dac0::RegisterBlock {
        Dac1::ptr() as _
    }
}

pub struct Dac<DAC: Instance> {
    dac: DAC,
    dac_pin: DAC::DacPin,
}

impl<DAC: Instance> Dac<DAC> {
    /// Настраивает цифро-аналоговый преобразователь и возвращает абстракцию
    pub fn new(dac: DAC, dac_pin: DAC::DacPin, clock: Hertz, clocks: &Clocks) -> Self {
        let pm = unsafe { Pm::steal() };
        pm.clk_apb_p_set().modify(|_, w| w.analog_regs().enable());

        let mut div = clocks.apb_p_clk().raw() / clock.raw();
        if div > 0 {
            div -= 1;
        }
        assert!(div < 256, "невозможная тактовая частота");
        let d = unsafe { &*DAC::ptr() };
        d.dac0_cfg().modify(|_, w| {
            w.en().enable();
            w.rn().set_bit();
            unsafe { w.div().bits(div as u8) }
        });

        Self { dac, dac_pin }
    }

    pub fn write(&mut self, value: u16) {
        let d = unsafe { &*DAC::ptr() };
        d.dac0_value().write(|w| unsafe { w.value().bits(value) });
    }

    /// Возвращает заимствованные ресурсы
    pub fn release(self) -> (DAC, DAC::DacPin) {
        (self.dac, self.dac_pin)
    }
}
