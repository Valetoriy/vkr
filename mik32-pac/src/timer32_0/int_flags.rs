#[doc = "Register `INT_FLAGS` reader"]
pub type R = crate::R<IntFlagsSpec>;
#[doc = "Field `OVF_Int` reader - Статус прерывания по переполнению счётчика"]
pub type OvfIntR = crate::BitReader;
#[doc = "Field `UDF_Int` reader - Статус прерывания опустошения счётчика"]
pub type UdfIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Статус прерывания по переполнению счётчика"]
    #[inline(always)]
    pub fn ovf_int(&self) -> OvfIntR {
        OvfIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Статус прерывания опустошения счётчика"]
    #[inline(always)]
    pub fn udf_int(&self) -> UdfIntR {
        UdfIntR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Регистр флагов прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_flags::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntFlagsSpec;
impl crate::RegisterSpec for IntFlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_flags::R`](R) reader structure"]
impl crate::Readable for IntFlagsSpec {}
#[doc = "`reset()` method sets INT_FLAGS to value 0"]
impl crate::Resettable for IntFlagsSpec {}
