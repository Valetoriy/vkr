#[doc = "Register `SYS_CLK_MUX` reader"]
pub type R = crate::R<SysClkMuxSpec>;
#[doc = "Register `SYS_CLK_MUX` writer"]
pub type W = crate::W<SysClkMuxSpec>;
#[doc = "Выбор источника тактирования системы\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux {
    #[doc = "0: Внешний OSC32M"]
    Osc32m = 0,
    #[doc = "1: Внутренний HSI32M"]
    Hsi32m = 1,
    #[doc = "2: Внешний OSC32K"]
    Osc32k = 2,
    #[doc = "3: Внутренний LSI32К"]
    Lsi32k = 3,
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(variant: Mux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux {
    type Ux = u8;
}
impl crate::IsEnum for Mux {}
#[doc = "Field `MUX` reader - Выбор источника тактирования системы"]
pub type MuxR = crate::FieldReader<Mux>;
impl MuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mux {
        match self.bits {
            0 => Mux::Osc32m,
            1 => Mux::Hsi32m,
            2 => Mux::Osc32k,
            3 => Mux::Lsi32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Внешний OSC32M"]
    #[inline(always)]
    pub fn is_osc32m(&self) -> bool {
        *self == Mux::Osc32m
    }
    #[doc = "Внутренний HSI32M"]
    #[inline(always)]
    pub fn is_hsi32m(&self) -> bool {
        *self == Mux::Hsi32m
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
#[doc = "Field `MUX` writer - Выбор источника тактирования системы"]
pub type MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mux, crate::Safe>;
impl<'a, REG> MuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Внешний OSC32M"]
    #[inline(always)]
    pub fn osc32m(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Osc32m)
    }
    #[doc = "Внутренний HSI32M"]
    #[inline(always)]
    pub fn hsi32m(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Hsi32m)
    }
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
#[doc = "Запрет на принудительное переключение с выбранного источника тактирования при пропадании тактирования\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceMux {
    #[doc = "0: Разрешение автоматической смены частоты"]
    Unfixed = 0,
    #[doc = "1: Запрет автоматической смены частоты"]
    Fixed = 1,
}
impl From<ForceMux> for bool {
    #[inline(always)]
    fn from(variant: ForceMux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_MUX` reader - Запрет на принудительное переключение с выбранного источника тактирования при пропадании тактирования"]
pub type ForceMuxR = crate::BitReader<ForceMux>;
impl ForceMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceMux {
        match self.bits {
            false => ForceMux::Unfixed,
            true => ForceMux::Fixed,
        }
    }
    #[doc = "Разрешение автоматической смены частоты"]
    #[inline(always)]
    pub fn is_unfixed(&self) -> bool {
        *self == ForceMux::Unfixed
    }
    #[doc = "Запрет автоматической смены частоты"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == ForceMux::Fixed
    }
}
#[doc = "Field `FORCE_MUX` writer - Запрет на принудительное переключение с выбранного источника тактирования при пропадании тактирования"]
pub type ForceMuxW<'a, REG> = crate::BitWriter<'a, REG, ForceMux>;
impl<'a, REG> ForceMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Разрешение автоматической смены частоты"]
    #[inline(always)]
    pub fn unfixed(self) -> &'a mut crate::W<REG> {
        self.variant(ForceMux::Unfixed)
    }
    #[doc = "Запрет автоматической смены частоты"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(ForceMux::Fixed)
    }
}
impl R {
    #[doc = "Bits 0:1 - Выбор источника тактирования системы"]
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Запрет на принудительное переключение с выбранного источника тактирования при пропадании тактирования"]
    #[inline(always)]
    pub fn force_mux(&self) -> ForceMuxR {
        ForceMuxR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Выбор источника тактирования системы"]
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<SysClkMuxSpec> {
        MuxW::new(self, 0)
    }
    #[doc = "Bit 2 - Запрет на принудительное переключение с выбранного источника тактирования при пропадании тактирования"]
    #[inline(always)]
    pub fn force_mux(&mut self) -> ForceMuxW<SysClkMuxSpec> {
        ForceMuxW::new(self, 2)
    }
}
#[doc = "Настройка источника тактирования системы\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysClkMuxSpec;
impl crate::RegisterSpec for SysClkMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_clk_mux::R`](R) reader structure"]
impl crate::Readable for SysClkMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_clk_mux::W`](W) writer structure"]
impl crate::Writable for SysClkMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CLK_MUX to value 0"]
impl crate::Resettable for SysClkMuxSpec {}
