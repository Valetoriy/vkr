#![no_main]
#![no_std]

use embedded_hal::digital::{InputPin, StatefulOutputPin};
use mik32_hal::{
    clocks::Clocks,
    gpio::{DS, GpioExt},
    pac,
    prelude::*,
};
use panic_halt as _;
use riscv::asm::delay;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pm = p.pm.constrain();
    let wu = p.wake_up.constrain();
    let _clocks = Clocks::freeze(pm, wu);

    let gpio2 = p.gpio_2.split();
    let mut button = gpio2.p2_6.into_pull_up_input(&p.pad_config);
    let mut led = gpio2.p2_7.into_output(&p.pad_config);
    led.set_ds(&p.pad_config, DS::_8mA);

    loop {
        if button.is_high().unwrap() {
            led.toggle().unwrap();
        }
        delay(1000000);
    }
}
