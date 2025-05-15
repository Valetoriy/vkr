#[doc = "Register `LEVEL_SET` reader"]
pub type R = crate::R<LevelSetSpec>;
#[doc = "Register `LEVEL_SET` writer"]
pub type W = crate::W<LevelSetSpec>;
#[doc = "Field `LEVEL_SET` reader - Биты выбора фронта и высокого уровня для запросов прерывания"]
pub type LevelSetR = crate::FieldReader;
#[doc = "Field `LEVEL_SET` writer - Биты выбора фронта и высокого уровня для запросов прерывания"]
pub type LevelSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты выбора фронта и высокого уровня для запросов прерывания"]
    #[inline(always)]
    pub fn level_set(&self) -> LevelSetR {
        LevelSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты выбора фронта и высокого уровня для запросов прерывания"]
    #[inline(always)]
    pub fn level_set(&mut self) -> LevelSetW<LevelSetSpec> {
        LevelSetW::new(self, 0)
    }
}
#[doc = "Выбор фронта и высокого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по фронту или уровню логической «1» для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`level_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LevelSetSpec;
impl crate::RegisterSpec for LevelSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`level_set::R`](R) reader structure"]
impl crate::Readable for LevelSetSpec {}
#[doc = "`write(|w| ..)` method takes [`level_set::W`](W) writer structure"]
impl crate::Writable for LevelSetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets LEVEL_SET to value 0"]
impl crate::Resettable for LevelSetSpec {}
