#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `TIM_VAL` reader - Текущее значение счётчика"]
pub type TimValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Текущее значение счётчика"]
    #[inline(always)]
    pub fn tim_val(&self) -> TimValR {
        TimValR::new(self.bits)
    }
}
#[doc = "Текущее значение основного таймера\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
