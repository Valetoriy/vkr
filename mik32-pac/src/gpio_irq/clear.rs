#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<ClearSpec>;
#[doc = "Field `CLEAR` writer - Биты сброса флагов запросов прерывания по событию"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Биты сброса флагов запросов прерывания по событию"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<ClearSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "Сброс флагов запросов прерывания по событию. Запись «1» – очищает флаг запроса прерывания для соответствующей линии\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for ClearSpec {}
