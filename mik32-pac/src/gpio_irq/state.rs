#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Field `STATE` reader - Биты состояния линий после мультиплексоров"]
pub type StateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Биты состояния линий после мультиплексоров"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Текущее состояние линий после мультиплексоров. Номер бита соответствует линии запроса прерывания\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for StateSpec {}
