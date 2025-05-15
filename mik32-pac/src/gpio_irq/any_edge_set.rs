#[doc = "Register `ANY_EDGE_SET` reader"]
pub type R = crate::R<AnyEdgeSetSpec>;
#[doc = "Register `ANY_EDGE_SET` writer"]
pub type W = crate::W<AnyEdgeSetSpec>;
#[doc = "Field `ANY_EDGE_SET` reader - Биты выбора запросов прерывания по любому событию"]
pub type AnyEdgeSetR = crate::FieldReader;
#[doc = "Field `ANY_EDGE_SET` writer - Биты выбора запросов прерывания по любому событию"]
pub type AnyEdgeSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты выбора запросов прерывания по любому событию"]
    #[inline(always)]
    pub fn any_edge_set(&self) -> AnyEdgeSetR {
        AnyEdgeSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты выбора запросов прерывания по любому событию"]
    #[inline(always)]
    pub fn any_edge_set(&mut self) -> AnyEdgeSetW<AnyEdgeSetSpec> {
        AnyEdgeSetW::new(self, 0)
    }
}
#[doc = "Выбор запросов прерывания по любому событию. Запись «1» – запрос формируется по любому изменению соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`any_edge_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_edge_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnyEdgeSetSpec;
impl crate::RegisterSpec for AnyEdgeSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`any_edge_set::R`](R) reader structure"]
impl crate::Readable for AnyEdgeSetSpec {}
#[doc = "`write(|w| ..)` method takes [`any_edge_set::W`](W) writer structure"]
impl crate::Writable for AnyEdgeSetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANY_EDGE_SET to value 0"]
impl crate::Resettable for AnyEdgeSetSpec {}
