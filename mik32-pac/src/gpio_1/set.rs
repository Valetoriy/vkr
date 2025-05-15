#[doc = "Register `SET` reader"]
pub type R = crate::R<SetSpec>;
#[doc = "Register `SET` writer"]
pub type W = crate::W<SetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Текущее состояние выводов (STATE) / Установка в «1» (SET) При чтении – текущее состояние выводов. При записи «1» соответствующий биту вывод устанавливается в «1»\n\nYou can [`read`](crate::Reg::read) this register and get [`set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetSpec;
impl crate::RegisterSpec for SetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set::R`](R) reader structure"]
impl crate::Readable for SetSpec {}
#[doc = "`write(|w| ..)` method takes [`set::W`](W) writer structure"]
impl crate::Writable for SetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET to value 0"]
impl crate::Resettable for SetSpec {}
