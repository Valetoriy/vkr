#[doc = "Register `RRTC_REG9` reader"]
pub type R = crate::R<RrtcReg9Spec>;
#[doc = "Register `RRTC_REG9` writer"]
pub type W = crate::W<RrtcReg9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистры общего назначения REG9\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcReg9Spec;
impl crate::RegisterSpec for RrtcReg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_reg9::R`](R) reader structure"]
impl crate::Readable for RrtcReg9Spec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_reg9::W`](W) writer structure"]
impl crate::Writable for RrtcReg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRTC_REG9 to value 0"]
impl crate::Resettable for RrtcReg9Spec {}
