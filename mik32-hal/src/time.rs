//! Единицы времени

#![allow(non_snake_case)]

pub use fugit::{
    HertzU32 as Hertz, KilohertzU32 as KiloHertz, MegahertzU32 as MegaHertz,
    MicrosDurationU32 as MicroSeconds, MillisDurationU32 as MilliSeconds,
};

/// Биты в секунду
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug)]
pub struct Bps(pub u32);

use core::ops;

pub trait U32Ext {
    /// Обернуть в биты в секунду
    fn bps(self) -> Bps;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }
}

impl ops::Mul<u32> for Bps {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::MulAssign<u32> for Bps {
    fn mul_assign(&mut self, rhs: u32) {
        self.0 *= rhs;
    }
}

impl ops::Div<u32> for Bps {
    type Output = Self;
    fn div(self, rhs: u32) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::Div<Bps> for Bps {
    type Output = u32;
    fn div(self, rhs: Bps) -> u32 {
        self.0 / rhs.0
    }
}

impl ops::DivAssign<u32> for Bps {
    fn div_assign(&mut self, rhs: u32) {
        self.0 /= rhs;
    }
}
