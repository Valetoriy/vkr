#![no_main]
#![no_std]

use core::fmt::Write;

use mik32_hal::{clocks::Clocks, crc::Crc32, gpio::GpioExt, pac, prelude::*, serial::Serial};

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pm = p.pm.constrain();
    let wu = p.wake_up.constrain();
    let clocks = Clocks::freeze(pm, wu);

    let gpio0 = p.gpio_0.split();
    let tx = gpio0.p0_6.into_serial(&p.pad_config);
    let mut tx = Serial::tx(p.usart_0, tx, 9600.bps(), &clocks);

    let mut crc = Crc32::new(p.crc, 0, 0x814141AB);
    let vals = [0xABCDABCD, 0xA1B2C3D4];
    for c in vals {
        crc.write(c);
    }
    let value = nb::block!(crc.read()).unwrap();
    writeln!(&mut tx, "CRC32 = 0x{:X}, ожидалось 0x6311BC18", value).unwrap();

    loop {}
}
