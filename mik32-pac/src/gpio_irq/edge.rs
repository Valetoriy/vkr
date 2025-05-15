#[doc = "Register `EDGE` reader"]
pub type R = crate::R<EdgeSpec>;
#[doc = "Register `EDGE` writer"]
pub type W = crate::W<EdgeSpec>;
#[doc = "Field `EDGE` reader - Биты выбора типа запросов прерывания по событию"]
pub type EdgeR = crate::FieldReader;
#[doc = "Field `EDGE` writer - Биты выбора типа запросов прерывания по событию"]
pub type EdgeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты выбора типа запросов прерывания по событию"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты выбора типа запросов прерывания по событию"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<EdgeSpec> {
        EdgeW::new(self, 0)
    }
}
#[doc = "Выбор типа запросов прерывания по событию. При чтении – текущий тип запросов: 0: по уровню; 1: по событию (фронт или спад). Запись «1» – запрос формируется по событию для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`edge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdgeSpec;
impl crate::RegisterSpec for EdgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edge::R`](R) reader structure"]
impl crate::Readable for EdgeSpec {}
#[doc = "`write(|w| ..)` method takes [`edge::W`](W) writer structure"]
impl crate::Writable for EdgeSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets EDGE to value 0"]
impl crate::Resettable for EdgeSpec {}
