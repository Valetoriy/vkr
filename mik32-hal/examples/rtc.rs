#![no_main]
#![no_std]

use core::fmt::Write;

use mik32_hal::{
    clocks::Clocks,
    gpio::GpioExt,
    pac,
    prelude::*,
    rtc::DayOfWeek,
    rtc::{RealTimeClock, RtcClkSrc, Time},
    serial::Serial,
};

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

    let mut rtc = RealTimeClock::new(p.rtc, RtcClkSrc::Automatic);
    rtc.set_time(Time::new(DayOfWeek::Monday, 0, 51, 12));
    let rtc = rtc.enable();

    let mut time = rtc.get_time();
    loop {
        let current_time = rtc.get_time();
        if time.seconds != current_time.seconds {
            writeln!(
                &mut tx,
                "Текущее вермя: {:?}, {:02}:{:02}:{:02}",
                time.dow, time.hours, time.minutes, time.seconds
            )
            .unwrap();
            time = current_time;
        }
    }
}
