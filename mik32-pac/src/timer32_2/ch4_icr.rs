#[doc = "Register `CH4_ICR` reader"]
pub type R = crate::R<Ch4IcrSpec>;
#[doc = "Register `CH4_ICR` writer"]
pub type W = crate::W<Ch4IcrSpec>;
#[doc = "Field `ICR` reader - Значение таймера в режиме захвата"]
pub type IcrR = crate::FieldReader<u32>;
#[doc = "Field `ICR` writer - Значение таймера в режиме захвата"]
pub type IcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Значение таймера в режиме захвата"]
    #[inline(always)]
    pub fn icr(&self) -> IcrR {
        IcrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Значение таймера в режиме захвата"]
    #[inline(always)]
    pub fn icr(&mut self) -> IcrW<Ch4IcrSpec> {
        IcrW::new(self, 0)
    }
}
#[doc = "Значение захвата 4 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4IcrSpec;
impl crate::RegisterSpec for Ch4IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_icr::R`](R) reader structure"]
impl crate::Readable for Ch4IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4_icr::W`](W) writer structure"]
impl crate::Writable for Ch4IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4_ICR to value 0"]
impl crate::Resettable for Ch4IcrSpec {}
