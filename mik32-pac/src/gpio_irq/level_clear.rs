#[doc = "Register `LEVEL_CLEAR` reader"]
pub type R = crate::R<LevelClearSpec>;
#[doc = "Register `LEVEL_CLEAR` writer"]
pub type W = crate::W<LevelClearSpec>;
#[doc = "Field `LEVEL_CLEAR` reader - Биты выбора спада и низкого уровня для запросов прерывания"]
pub type LevelClearR = crate::FieldReader;
#[doc = "Field `LEVEL_CLEAR` writer - Биты выбора спада и низкого уровня для запросов прерывания"]
pub type LevelClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Биты выбора спада и низкого уровня для запросов прерывания"]
    #[inline(always)]
    pub fn level_clear(&self) -> LevelClearR {
        LevelClearR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Биты выбора спада и низкого уровня для запросов прерывания"]
    #[inline(always)]
    pub fn level_clear(&mut self) -> LevelClearW<LevelClearSpec> {
        LevelClearW::new(self, 0)
    }
}
#[doc = "Выбор спада и низкого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по спаду или уровню логического «0» для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`level_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LevelClearSpec;
impl crate::RegisterSpec for LevelClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`level_clear::R`](R) reader structure"]
impl crate::Readable for LevelClearSpec {}
#[doc = "`write(|w| ..)` method takes [`level_clear::W`](W) writer structure"]
impl crate::Writable for LevelClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets LEVEL_CLEAR to value 0"]
impl crate::Resettable for LevelClearSpec {}
