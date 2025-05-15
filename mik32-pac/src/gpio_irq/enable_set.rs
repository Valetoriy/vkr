#[doc = "Register `ENABLE_SET` reader"]
pub type R = crate::R<EnableSetSpec>;
#[doc = "Register `ENABLE_SET` writer"]
pub type W = crate::W<EnableSetSpec>;
#[doc = "Field `ENABLE_SET` reader - Биты разрешения запросов прерывания"]
pub type EnableSetR = crate::FieldReader;
#[doc = "Field `ENABLE_SET` writer - Биты разрешения запросов прерывания"]
pub type EnableSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты разрешения запросов прерывания"]
    #[inline(always)]
    pub fn enable_set(&self) -> EnableSetR {
        EnableSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты разрешения запросов прерывания"]
    #[inline(always)]
    pub fn enable_set(&mut self) -> EnableSetW<EnableSetSpec> {
        EnableSetW::new(self, 0)
    }
}
#[doc = "Разрешение запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» разрешает запрос от соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSetSpec;
impl crate::RegisterSpec for EnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_set::R`](R) reader structure"]
impl crate::Readable for EnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_set::W`](W) writer structure"]
impl crate::Writable for EnableSetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ENABLE_SET to value 0"]
impl crate::Resettable for EnableSetSpec {}
