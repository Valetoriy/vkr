#[doc = "Register `LEVEL` reader"]
pub type R = crate::R<LevelSpec>;
#[doc = "Register `LEVEL` writer"]
pub type W = crate::W<LevelSpec>;
#[doc = "Field `LEVEL` reader - Биты выбора типа запросов прерывания по уровню"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Биты выбора типа запросов прерывания по уровню"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты выбора типа запросов прерывания по уровню"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты выбора типа запросов прерывания по уровню"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<LevelSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Выбор типа запросов прерывания по уровню. При чтении – текущий тип запроса (инвертированный): 0: по событию (фронт или спад); 1: по уровню. Запись «1» – запрос формируется по уровню для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LevelSpec;
impl crate::RegisterSpec for LevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`level::R`](R) reader structure"]
impl crate::Readable for LevelSpec {}
#[doc = "`write(|w| ..)` method takes [`level::W`](W) writer structure"]
impl crate::Writable for LevelSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets LEVEL to value 0xff"]
impl crate::Resettable for LevelSpec {
    const RESET_VALUE: u32 = 0xff;
}
