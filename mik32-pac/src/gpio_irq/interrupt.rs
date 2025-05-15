#[doc = "Register `INTERRUPT` reader"]
pub type R = crate::R<InterruptSpec>;
#[doc = "Field `INTERRUPT` reader - Биты состояния флагов запросов прерывания"]
pub type InterruptR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Биты состояния флагов запросов прерывания"]
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Текущее состояние флагов запросов прерывания. Номер бита соответствует номеру линии\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptSpec;
impl crate::RegisterSpec for InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for InterruptSpec {}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for InterruptSpec {}
