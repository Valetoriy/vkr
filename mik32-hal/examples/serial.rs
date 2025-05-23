#![no_main]
#![no_std]

use embedded_hal_nb::serial::{Read, Write};
use mik32_hal::{clocks::Clocks, gpio::GpioExt, pac, prelude::*, serial::Serial};
use nb::block;

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pm = p.pm.constrain();
    let wu = p.wake_up.constrain();
    let clocks = Clocks::freeze(pm, wu);

    let gpio0 = p.gpio_0.split();
    let tx = gpio0.p0_6.into_serial(&p.pad_config);
    let rx = gpio0.p0_5.into_serial(&p.pad_config);

    let (mut tx, mut rx) = Serial::new(p.usart_0, (tx, rx), 9600.bps(), &clocks).split();

    loop {
        let b: u8 = nb::block!(rx.read()).unwrap();
        block!(tx.write(b)).unwrap();
    }
}
