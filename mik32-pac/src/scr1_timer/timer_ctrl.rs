#[doc = "Register `TIMER_CTRL` reader"]
pub type R = crate::R<TimerCtrlSpec>;
#[doc = "Register `TIMER_CTRL` writer"]
pub type W = crate::W<TimerCtrlSpec>;
#[doc = "Включение таймера\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Таймер выключен"]
    Diasable = 0,
    #[doc = "1: Таймер включён"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Включение таймера"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Diasable,
            true => Enable::Enable,
        }
    }
    #[doc = "Таймер выключен"]
    #[inline(always)]
    pub fn is_diasable(&self) -> bool {
        *self == Enable::Diasable
    }
    #[doc = "Таймер включён"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Включение таймера"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Таймер выключен"]
    #[inline(always)]
    pub fn diasable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Diasable)
    }
    #[doc = "Таймер включён"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
#[doc = "Источник тактирования\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksrc {
    #[doc = "0: (до 32 МГц) AHB_CLK ядра процессора"]
    CpuAhbClk = 0,
    #[doc = "1: (32768 Гц) OSC32K или LSI32K, выбирается в PM.CPU_RTC_CLK_MUX"]
    CpuRtcClk = 1,
}
impl From<Clksrc> for bool {
    #[inline(always)]
    fn from(variant: Clksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - Источник тактирования"]
pub type ClksrcR = crate::BitReader<Clksrc>;
impl ClksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksrc {
        match self.bits {
            false => Clksrc::CpuAhbClk,
            true => Clksrc::CpuRtcClk,
        }
    }
    #[doc = "(до 32 МГц) AHB_CLK ядра процессора"]
    #[inline(always)]
    pub fn is_cpu_ahb_clk(&self) -> bool {
        *self == Clksrc::CpuAhbClk
    }
    #[doc = "(32768 Гц) OSC32K или LSI32K, выбирается в PM.CPU_RTC_CLK_MUX"]
    #[inline(always)]
    pub fn is_cpu_rtc_clk(&self) -> bool {
        *self == Clksrc::CpuRtcClk
    }
}
#[doc = "Field `CLKSRC` writer - Источник тактирования"]
pub type ClksrcW<'a, REG> = crate::BitWriter<'a, REG, Clksrc>;
impl<'a, REG> ClksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(до 32 МГц) AHB_CLK ядра процессора"]
    #[inline(always)]
    pub fn cpu_ahb_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::CpuAhbClk)
    }
    #[doc = "(32768 Гц) OSC32K или LSI32K, выбирается в PM.CPU_RTC_CLK_MUX"]
    #[inline(always)]
    pub fn cpu_rtc_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::CpuRtcClk)
    }
}
impl R {
    #[doc = "Bit 0 - Включение таймера"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Источник тактирования"]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Включение таймера"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<TimerCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Источник тактирования"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<TimerCtrlSpec> {
        ClksrcW::new(self, 1)
    }
}
#[doc = "Регистр конфигурации\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerCtrlSpec;
impl crate::RegisterSpec for TimerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ctrl::R`](R) reader structure"]
impl crate::Readable for TimerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_ctrl::W`](W) writer structure"]
impl crate::Writable for TimerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_CTRL to value 0x01"]
impl crate::Resettable for TimerCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
