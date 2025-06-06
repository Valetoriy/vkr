#[doc = "Register `DIV_APB_M` reader"]
pub type R = crate::R<DivApbMSpec>;
#[doc = "Register `DIV_APB_M` writer"]
pub type W = crate::W<DivApbMSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Задаёт значение делителя частоты шины APB_M. Частота шины APB_M (APB_M_CLK) рассчитывается, как AHB_CLK/(DIV_APB_M+1)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_apb_m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_apb_m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivApbMSpec;
impl crate::RegisterSpec for DivApbMSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_apb_m::R`](R) reader structure"]
impl crate::Readable for DivApbMSpec {}
#[doc = "`write(|w| ..)` method takes [`div_apb_m::W`](W) writer structure"]
impl crate::Writable for DivApbMSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV_APB_M to value 0"]
impl crate::Resettable for DivApbMSpec {}
