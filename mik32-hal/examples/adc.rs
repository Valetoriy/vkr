#![no_main]
#![no_std]

use embedded_hal::digital::OutputPin;
use mik32_hal::{adc::Adc, clocks::Clocks, gpio::GpioExt, pac, prelude::*};

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pm = p.pm.constrain();
    let wu = p.wake_up.constrain();
    let _clocks = Clocks::freeze(pm, wu);

    let gpio2 = p.gpio_2.split();
    let mut led = gpio2.p2_7.into_output(&p.pad_config);

    let gpio1 = p.gpio_1.split();
    let adc_pin = gpio1.p1_5.into_analog(&p.pad_config);
    let mut adc = Adc::new(p.adc);
    adc.select_channel(&adc_pin);

    loop {
        let value = adc.read();
        let state = value > 2048;
        led.set_state(state.into()).unwrap();
    }
}
