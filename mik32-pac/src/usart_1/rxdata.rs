#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Field `RDR` reader - Принятые данные"]
pub type RdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Принятые данные"]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Регистр принятых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RxdataSpec {}
