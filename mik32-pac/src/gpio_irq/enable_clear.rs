#[doc = "Register `ENABLE_CLEAR` reader"]
pub type R = crate::R<EnableClearSpec>;
#[doc = "Register `ENABLE_CLEAR` writer"]
pub type W = crate::W<EnableClearSpec>;
#[doc = "Field `ENABLE_CLEAR` reader - Биты запрета запросов прерывания"]
pub type EnableClearR = crate::FieldReader;
#[doc = "Field `ENABLE_CLEAR` writer - Биты запрета запросов прерывания"]
pub type EnableClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты запрета запросов прерывания"]
    #[inline(always)]
    pub fn enable_clear(&self) -> EnableClearR {
        EnableClearR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты запрета запросов прерывания"]
    #[inline(always)]
    pub fn enable_clear(&mut self) -> EnableClearW<EnableClearSpec> {
        EnableClearW::new(self, 0)
    }
}
#[doc = "Запрет запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» запрещает запрос от соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableClearSpec;
impl crate::RegisterSpec for EnableClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_clear::R`](R) reader structure"]
impl crate::Readable for EnableClearSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_clear::W`](W) writer structure"]
impl crate::Writable for EnableClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ENABLE_CLEAR to value 0"]
impl crate::Resettable for EnableClearSpec {}
