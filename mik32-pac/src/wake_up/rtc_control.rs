#[doc = "Register `RTC_CONTROL` reader"]
pub type R = crate::R<RtcControlSpec>;
#[doc = "Register `RTC_CONTROL` writer"]
pub type W = crate::W<RtcControlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Сброс RTC происходит при записи «1»\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcControlSpec;
impl crate::RegisterSpec for RtcControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_control::R`](R) reader structure"]
impl crate::Readable for RtcControlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_control::W`](W) writer structure"]
impl crate::Writable for RtcControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CONTROL to value 0"]
impl crate::Resettable for RtcControlSpec {}
