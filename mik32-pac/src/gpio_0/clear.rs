#[doc = "Register `CLEAR` reader"]
pub type R = crate::R<ClearSpec>;
#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<ClearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Текущее состояние запросов прерываний (IRQ_STATE) / Установка в «0» (CLEAR) При чтении – текущее состояние запросов прерываний. При записи «1» соответствующий биту вывод устанавливается в «0»\n\nYou can [`read`](crate::Reg::read) this register and get [`clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear::R`](R) reader structure"]
impl crate::Readable for ClearSpec {}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for ClearSpec {}
