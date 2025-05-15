#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "Register `INT_MASK` writer"]
pub type W = crate::W<IntMaskSpec>;
#[doc = "Field `OVF_Int` reader - Маска прерывания по переполнению счётчика"]
pub type OvfIntR = crate::BitReader;
#[doc = "Field `OVF_Int` writer - Маска прерывания по переполнению счётчика"]
pub type OvfIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDF_Int` reader - Маска прерывания опустошения счётчика"]
pub type UdfIntR = crate::BitReader;
#[doc = "Field `UDF_Int` writer - Маска прерывания опустошения счётчика"]
pub type UdfIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Маска прерывания по переполнению счётчика"]
    #[inline(always)]
    pub fn ovf_int(&self) -> OvfIntR {
        OvfIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Маска прерывания опустошения счётчика"]
    #[inline(always)]
    pub fn udf_int(&self) -> UdfIntR {
        UdfIntR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Маска прерывания по переполнению счётчика"]
    #[inline(always)]
    pub fn ovf_int(&mut self) -> OvfIntW<IntMaskSpec> {
        OvfIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Маска прерывания опустошения счётчика"]
    #[inline(always)]
    pub fn udf_int(&mut self) -> UdfIntW<IntMaskSpec> {
        UdfIntW::new(self, 1)
    }
}
#[doc = "Регистр маски прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_mask::W`](W) writer structure"]
impl crate::Writable for IntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for IntMaskSpec {}
