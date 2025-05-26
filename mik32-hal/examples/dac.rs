#![no_main]
#![no_std]

use mik32_hal::{clocks::Clocks, dac::Dac, gpio::GpioExt, pac, prelude::*};

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pm = p.pm.constrain();
    let wu = p.wake_up.constrain();
    let clocks = Clocks::freeze(pm, wu);

    let gpio1 = p.gpio_1.split();
    let dac_pin = gpio1.p1_12.into_analog(&p.pad_config);
    let mut dac0 = Dac::new(p.dac0, dac_pin, 1.MHz(), &clocks);
    loop {
        for i in 0..4095 {
            dac0.write(i);
        }
        for i in (0..4095).rev() {
            dac0.write(i);
        }
    }
}
