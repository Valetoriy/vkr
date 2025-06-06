#[doc = "Register `RRTC_REG11` reader"]
pub type R = crate::R<RrtcReg11Spec>;
#[doc = "Register `RRTC_REG11` writer"]
pub type W = crate::W<RrtcReg11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистры общего назначения REG11\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcReg11Spec;
impl crate::RegisterSpec for RrtcReg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_reg11::R`](R) reader structure"]
impl crate::Readable for RrtcReg11Spec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_reg11::W`](W) writer structure"]
impl crate::Writable for RrtcReg11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRTC_REG11 to value 0"]
impl crate::Resettable for RrtcReg11Spec {}
