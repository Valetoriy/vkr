#[doc = "Register `TOP` reader"]
pub type R = crate::R<TopSpec>;
#[doc = "Register `TOP` writer"]
pub type W = crate::W<TopSpec>;
#[doc = "Field `TIM_TOP` reader - максимальное значение счётчика (ограничивает счётную последовательность сверху)"]
pub type TimTopR = crate::FieldReader<u32>;
#[doc = "Field `TIM_TOP` writer - максимальное значение счётчика (ограничивает счётную последовательность сверху)"]
pub type TimTopW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - максимальное значение счётчика (ограничивает счётную последовательность сверху)"]
    #[inline(always)]
    pub fn tim_top(&self) -> TimTopR {
        TimTopR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - максимальное значение счётчика (ограничивает счётную последовательность сверху)"]
    #[inline(always)]
    pub fn tim_top(&mut self) -> TimTopW<TopSpec> {
        TimTopW::new(self, 0)
    }
}
#[doc = "максимальное значение счётной последовательности\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopSpec;
impl crate::RegisterSpec for TopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TopSpec {}
#[doc = "`write(|w| ..)` method takes [`top::W`](W) writer structure"]
impl crate::Writable for TopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOP to value 0xffff_ffff"]
impl crate::Resettable for TopSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
