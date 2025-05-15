#[doc = "Register `ANY_EDGE_CLEAR` reader"]
pub type R = crate::R<AnyEdgeClearSpec>;
#[doc = "Register `ANY_EDGE_CLEAR` writer"]
pub type W = crate::W<AnyEdgeClearSpec>;
#[doc = "Field `ANY_EDGE_CLEAR` reader - Биты отмены запросов прерывания по любому событию"]
pub type AnyEdgeClearR = crate::FieldReader;
#[doc = "Field `ANY_EDGE_CLEAR` writer - Биты отмены запросов прерывания по любому событию"]
pub type AnyEdgeClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты отмены запросов прерывания по любому событию"]
    #[inline(always)]
    pub fn any_edge_clear(&self) -> AnyEdgeClearR {
        AnyEdgeClearR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты отмены запросов прерывания по любому событию"]
    #[inline(always)]
    pub fn any_edge_clear(&mut self) -> AnyEdgeClearW<AnyEdgeClearSpec> {
        AnyEdgeClearW::new(self, 0)
    }
}
#[doc = "Отмена выбора запросов прерывания по любому событию. Запись «1» – запрос не формируется по любому изменению соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`any_edge_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_edge_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnyEdgeClearSpec;
impl crate::RegisterSpec for AnyEdgeClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`any_edge_clear::R`](R) reader structure"]
impl crate::Readable for AnyEdgeClearSpec {}
#[doc = "`write(|w| ..)` method takes [`any_edge_clear::W`](W) writer structure"]
impl crate::Writable for AnyEdgeClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANY_EDGE_CLEAR to value 0"]
impl crate::Resettable for AnyEdgeClearSpec {}
