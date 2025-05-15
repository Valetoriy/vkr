#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Field `CNT` reader - Значение счётчика. Когда TIMER16 работает с асинхронными тактовыми сигналами, чтение регистра CNT может вернуть недостоверные значения. Поэтому в этом случае необходимо выполнить два последовательных доступа на чтение и убедиться, что два возвращенных значения идентичны"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Значение счётчика. Когда TIMER16 работает с асинхронными тактовыми сигналами, чтение регистра CNT может вернуть недостоверные значения. Поэтому в этом случае необходимо выполнить два последовательных доступа на чтение и убедиться, что два возвращенных значения идентичны"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Регистр счётчика\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {}
