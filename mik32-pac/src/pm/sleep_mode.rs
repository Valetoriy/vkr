#[doc = "Register `SLEEP_MODE` reader"]
pub type R = crate::R<SleepModeSpec>;
#[doc = "Register `SLEEP_MODE` writer"]
pub type W = crate::W<SleepModeSpec>;
#[doc = "Field `EEPROM` reader - Отключение тактирования ЭСППЗУ"]
pub type EepromR = crate::BitReader;
#[doc = "Field `EEPROM` writer - Отключение тактирования ЭСППЗУ"]
pub type EepromW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM` reader - Отключение тактирования ОЗУ"]
pub type RamR = crate::BitReader;
#[doc = "Field `RAM` writer - Отключение тактирования ОЗУ"]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIFI` reader - Отключение тактирования контроллера SPIFI"]
pub type SpifiR = crate::BitReader;
#[doc = "Field `SPIFI` writer - Отключение тактирования контроллера SPIFI"]
pub type SpifiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Отключение тактирования ЭСППЗУ"]
    #[inline(always)]
    pub fn eeprom(&self) -> EepromR {
        EepromR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Отключение тактирования ОЗУ"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Отключение тактирования контроллера SPIFI"]
    #[inline(always)]
    pub fn spifi(&self) -> SpifiR {
        SpifiR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Отключение тактирования ЭСППЗУ"]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EepromW<SleepModeSpec> {
        EepromW::new(self, 0)
    }
    #[doc = "Bit 1 - Отключение тактирования ОЗУ"]
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<SleepModeSpec> {
        RamW::new(self, 1)
    }
    #[doc = "Bit 2 - Отключение тактирования контроллера SPIFI"]
    #[inline(always)]
    pub fn spifi(&mut self) -> SpifiW<SleepModeSpec> {
        SpifiW::new(self, 2)
    }
}
#[doc = "Переход в спящий режим осуществляется записью «1» в три младшие бита данного регистра. При записи отключается тактирование ядра. В зависимости от записываемого значения отключается тактирование модулей\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepModeSpec;
impl crate::RegisterSpec for SleepModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_mode::R`](R) reader structure"]
impl crate::Readable for SleepModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sleep_mode::W`](W) writer structure"]
impl crate::Writable for SleepModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_MODE to value 0"]
impl crate::Resettable for SleepModeSpec {}
