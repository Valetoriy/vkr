#[doc = "Register `INT_FLAGS` reader"]
pub type R = crate::R<IntFlagsSpec>;
#[doc = "Field `OVF_Int` reader - Статус прерывания по переполнению счётчика"]
pub type OvfIntR = crate::BitReader;
#[doc = "Field `UDF_Int` reader - Статус прерывания опустошения счётчика"]
pub type UdfIntR = crate::BitReader;
#[doc = "Field `IC_Int_CH1` reader - Статус прерывания захвата 1 канала таймера"]
pub type IcIntCh1R = crate::BitReader;
#[doc = "Field `IC_Int_CH2` reader - Статус прерывания захвата 2 канала таймера"]
pub type IcIntCh2R = crate::BitReader;
#[doc = "Field `IC_Int_CH3` reader - Статус прерывания захвата 3 канала таймера"]
pub type IcIntCh3R = crate::BitReader;
#[doc = "Field `IC_Int_CH4` reader - Статус прерывания захвата 4 канала таймера"]
pub type IcIntCh4R = crate::BitReader;
#[doc = "Field `OC_Int_CH1` reader - Статус прерывания совпадения 1 канала таймера"]
pub type OcIntCh1R = crate::BitReader;
#[doc = "Field `OC_Int_CH2` reader - Статус прерывания совпадения 2 канала таймера"]
pub type OcIntCh2R = crate::BitReader;
#[doc = "Field `OC_Int_CH3` reader - Статус прерывания совпадения 3 канала таймера"]
pub type OcIntCh3R = crate::BitReader;
#[doc = "Field `OC_Int_CH4` reader - Статус прерывания совпадения 4 канала таймера"]
pub type OcIntCh4R = crate::BitReader;
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
    #[doc = "Bit 2 - Статус прерывания захвата 1 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch1(&self) -> IcIntCh1R {
        IcIntCh1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Статус прерывания захвата 2 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch2(&self) -> IcIntCh2R {
        IcIntCh2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Статус прерывания захвата 3 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch3(&self) -> IcIntCh3R {
        IcIntCh3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Статус прерывания захвата 4 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch4(&self) -> IcIntCh4R {
        IcIntCh4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Статус прерывания совпадения 1 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch1(&self) -> OcIntCh1R {
        OcIntCh1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Статус прерывания совпадения 2 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch2(&self) -> OcIntCh2R {
        OcIntCh2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Статус прерывания совпадения 3 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch3(&self) -> OcIntCh3R {
        OcIntCh3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Статус прерывания совпадения 4 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch4(&self) -> OcIntCh4R {
        OcIntCh4R::new(((self.bits >> 9) & 1) != 0)
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
