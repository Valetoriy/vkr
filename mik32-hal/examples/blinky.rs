#![no_main]
#![no_std]

use mik32_hal::{clocks::Clocks, pac, prelude::*};
use mik32_pac::Pm;
use panic_halt as _;
use riscv::asm::delay;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pm = p.pm.constrain();
    let wu = p.wake_up.constrain();
    let _clocks = Clocks::freeze(pm, wu);

    let pm = unsafe { Pm::steal() };
    pm.clk_apb_p_set().write(|w| w.gpio_2().set_bit());

    let pc = p.pad_config;
    pc.pad2_cfg().write(|w| w.port_2_7().func1_gpio());
    let gpio2 = p.gpio_2;
    gpio2.direction_out().write(|w| unsafe { w.bits(1 << 7) });

    loop {
        gpio2.output().modify(|r, w| {
            let bits = r.bits() ^ (1 << 7);
            unsafe { w.bits(bits) }
        });
        delay(1000000);
    }
}
