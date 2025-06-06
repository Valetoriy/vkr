#[doc = "Register `OUTPUT` reader"]
pub type R = crate::R<OutputSpec>;
#[doc = "Register `OUTPUT` writer"]
pub type W = crate::W<OutputSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Выходной регистр Чтение возвращает содержимое выходного регистра независимо от текущего направления выводов Запись устанавливает значения всех битов выходного регистра\n\nYou can [`read`](crate::Reg::read) this register and get [`output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputSpec;
impl crate::RegisterSpec for OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`output::R`](R) reader structure"]
impl crate::Readable for OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`output::W`](W) writer structure"]
impl crate::Writable for OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTPUT to value 0"]
impl crate::Resettable for OutputSpec {}
