#![no_main]
#![no_std]

use embedded_hal::digital::{InputPin, OutputPin};
use mik32_hal::{clocks::Clocks, gpio::GpioExt, pac, prelude::*};
use panic_halt as _;
use riscv::asm::delay;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pm = p.pm.constrain();
    let wu = p.wake_up.constrain();
    let _clocks = Clocks::freeze(pm, wu);

    let gpio2 = p.gpio_2.split();
    let mut button = gpio2.p2_6.into_floating_input(&p.pad_config);
    let mut led = gpio2.p2_7.into_output(&p.pad_config);

    loop {
        let pressed = button.is_high().unwrap();
        led.set_state(pressed.into()).unwrap();
        delay(1000000);
    }
}
