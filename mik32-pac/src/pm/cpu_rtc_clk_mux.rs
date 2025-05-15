#[doc = "Register `CPU_RTC_CLK_MUX` reader"]
pub type R = crate::R<CpuRtcClkMuxSpec>;
#[doc = "Register `CPU_RTC_CLK_MUX` writer"]
pub type W = crate::W<CpuRtcClkMuxSpec>;
#[doc = "Выбор источника тактирования сторожевого таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mux {
    #[doc = "0: Внешний OSC32K"]
    Osc32k = 0,
    #[doc = "1: Внутренний LSI32К"]
    Lsi32k = 1,
}
impl From<Mux> for bool {
    #[inline(always)]
    fn from(variant: Mux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX` reader - Выбор источника тактирования сторожевого таймера"]
pub type MuxR = crate::BitReader<Mux>;
impl MuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mux {
        match self.bits {
            false => Mux::Osc32k,
            true => Mux::Lsi32k,
        }
    }
    #[doc = "Внешний OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == Mux::Osc32k
    }
    #[doc = "Внутренний LSI32К"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == Mux::Lsi32k
    }
}
#[doc = "Field `MUX` writer - Выбор источника тактирования сторожевого таймера"]
pub type MuxW<'a, REG> = crate::BitWriter<'a, REG, Mux>;
impl<'a, REG> MuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Внешний OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Osc32k)
    }
    #[doc = "Внутренний LSI32К"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Lsi32k)
    }
}
impl R {
    #[doc = "Bit 0 - Выбор источника тактирования сторожевого таймера"]
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Выбор источника тактирования сторожевого таймера"]
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<CpuRtcClkMuxSpec> {
        MuxW::new(self, 0)
    }
}
#[doc = "Выбор источника тактирования RTC для системного таймера в составе ядра\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_rtc_clk_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_rtc_clk_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuRtcClkMuxSpec;
impl crate::RegisterSpec for CpuRtcClkMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_rtc_clk_mux::R`](R) reader structure"]
impl crate::Readable for CpuRtcClkMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_rtc_clk_mux::W`](W) writer structure"]
impl crate::Writable for CpuRtcClkMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_RTC_CLK_MUX to value 0"]
impl crate::Resettable for CpuRtcClkMuxSpec {}
