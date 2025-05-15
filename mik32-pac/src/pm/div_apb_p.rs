#[doc = "Register `DIV_APB_P` reader"]
pub type R = crate::R<DivApbPSpec>;
#[doc = "Register `DIV_APB_P` writer"]
pub type W = crate::W<DivApbPSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Задаёт значение делителя частоты шины APB_P. Частота шины APB_P (APB_M_CLK) рассчитывается, как AHB_CLK/(DIV_APB_P+1)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_apb_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_apb_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivApbPSpec;
impl crate::RegisterSpec for DivApbPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_apb_p::R`](R) reader structure"]
impl crate::Readable for DivApbPSpec {}
#[doc = "`write(|w| ..)` method takes [`div_apb_p::W`](W) writer structure"]
impl crate::Writable for DivApbPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV_APB_P to value 0"]
impl crate::Resettable for DivApbPSpec {}
