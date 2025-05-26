//! Часы реального времени

use crate::pac::{Pm, Rtc, WakeUp};

pub use crate::pac::rtc::rrtc_time::DayOfWeek;
pub use crate::pac::wake_up::clocks_bu::RtcClkMux as RtcClkSrc;

/// Абстракция `RTC`
pub struct RealTimeClock<const ENABLED: bool> {
    rtc: Rtc,
}

pub struct Time {
    pub dow: DayOfWeek,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Time {
    pub fn new(dow: DayOfWeek, hours: u8, minutes: u8, seconds: u8) -> Self {
        assert!(hours < 24 && minutes < 60 && seconds < 60);
        Self {
            dow,
            hours,
            minutes,
            seconds,
        }
    }
}

pub struct Date {
    pub centuries: u8,
    pub years: u8,
    pub months: u8,
    pub days: u8,
}

impl Date {
    pub fn new(centuries: u8, years: u8, months: u8, days: u8) -> Self {
        assert!(centuries < 100 && years < 100 && months < 13 && days < 32);
        Self {
            centuries,
            years,
            months,
            days,
        }
    }
}

impl<const ENABLED: bool> RealTimeClock<ENABLED> {
    fn wait_for_flag(&self) {
        while self.rtc.rrtc_ctrl().read().flag().bit_is_set() {}
    }
}

impl RealTimeClock<false> {
    /// Возвращает `RTC` в выключенном состоянии
    pub fn new(rtc: Rtc, clk_src: RtcClkSrc) -> Self {
        let wu = unsafe { WakeUp::steal() };
        wu.clocks_bu()
            .modify(|_, w| w.rtc_clk_mux().variant(clk_src));

        let pm = unsafe { Pm::steal() };
        pm.clk_apb_m_set().modify(|_, w| w.rtc().enable());

        rtc.rrtc_ctrl().modify(|_, w| w.en().disabled());

        let rtc = Self { rtc };
        rtc.wait_for_flag();
        rtc
    }

    pub fn set_time(&mut self, time: Time) {
        let Time {
            dow,
            hours,
            minutes,
            seconds,
        } = time;
        self.rtc.rrtc_time().write(|w| unsafe {
            w.dow().variant(dow);
            w.th().bits(hours / 10);
            w.h().bits(hours % 10);
            w.tm().bits(minutes / 10);
            w.m().bits(minutes % 10);
            w.ts().bits(seconds / 10);
            w.s().bits(seconds % 10)
        });
        self.wait_for_flag();
    }

    pub fn set_date(&mut self, date: Date) {
        let Date {
            centuries,
            years,
            months,
            days,
        } = date;
        self.rtc.rrtc_date().write(|w| unsafe {
            w.tc().bits(centuries / 10);
            w.c().bits(centuries % 10);
            w.ty().bits(years / 10);
            w.y().bits(years % 10);
            w.tm().bit(months == 1);
            w.m().bits(months % 10);
            w.td().bits(days / 10);
            w.d().bits(days % 10)
        });
        self.wait_for_flag();
    }

    pub fn enable(self) -> RealTimeClock<true> {
        self.rtc.rrtc_ctrl().modify(|_, w| w.en().enable());
        self.wait_for_flag();

        RealTimeClock { rtc: self.rtc }
    }
}

impl RealTimeClock<true> {
    pub fn get_time(&self) -> Time {
        let time = self.rtc.rrtc_time().read();
        Time {
            dow: time.dow().variant().unwrap(),
            hours: time.th().bits() * 10 + time.h().bits(),
            minutes: time.tm().bits() * 10 + time.m().bits(),
            seconds: time.ts().bits() * 10 + time.s().bits(),
        }
    }

    pub fn get_date(&self) -> Date {
        let date = self.rtc.rrtc_date().read();
        Date {
            centuries: date.tc().bits() * 10 + date.c().bits(),
            years: date.ty().bits() * 10 + date.y().bits(),
            months: date.tm().bit() as u8 * 10 + date.m().bits(),
            days: date.td().bits() * 10 + date.d().bits(),
        }
    }

    pub fn disable(self) -> RealTimeClock<false> {
        self.rtc.rrtc_ctrl().modify(|_, w| w.en().disabled());
        self.wait_for_flag();

        RealTimeClock { rtc: self.rtc }
    }
}
