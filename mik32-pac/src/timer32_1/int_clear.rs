#[doc = "Register `INT_CLEAR` reader"]
pub type R = crate::R<IntClearSpec>;
#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<IntClearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистр сброса флагов прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clear::R`](R) reader structure"]
impl crate::Readable for IntClearSpec {}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0xffff_ffff"]
impl crate::Resettable for IntClearSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
