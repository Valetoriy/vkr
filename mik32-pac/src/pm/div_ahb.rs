#[doc = "Register `DIV_AHB` reader"]
pub type R = crate::R<DivAhbSpec>;
#[doc = "Register `DIV_AHB` writer"]
pub type W = crate::W<DivAhbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Задаёт значение делителя шины AHB. Частота шины AHB (AHB_CLK) рассчитывается, как SYS_CLK/(DIV_AHB+1)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_ahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_ahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivAhbSpec;
impl crate::RegisterSpec for DivAhbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_ahb::R`](R) reader structure"]
impl crate::Readable for DivAhbSpec {}
#[doc = "`write(|w| ..)` method takes [`div_ahb::W`](W) writer structure"]
impl crate::Writable for DivAhbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV_AHB to value 0"]
impl crate::Resettable for DivAhbSpec {}
