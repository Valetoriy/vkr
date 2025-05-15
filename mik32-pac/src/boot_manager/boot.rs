#[doc = "Register `BOOT` reader"]
pub type R = crate::R<BootSpec>;
#[doc = "Регистр чтения режима загрузки\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BootMode {
    #[doc = "0: Старт из внутреннего ЭСППЗУ"]
    Eeprom = 0,
    #[doc = "1: Старт из системного ОЗУ"]
    Ram = 1,
    #[doc = "2: Старт из внешней памяти с использованием контроллера SPIFI"]
    Spifi = 2,
}
impl From<BootMode> for u8 {
    #[inline(always)]
    fn from(variant: BootMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BootMode {
    type Ux = u8;
}
impl crate::IsEnum for BootMode {}
#[doc = "Field `BOOT_MODE` reader - Регистр чтения режима загрузки"]
pub type BootModeR = crate::FieldReader<BootMode>;
impl BootModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BootMode> {
        match self.bits {
            0 => Some(BootMode::Eeprom),
            1 => Some(BootMode::Ram),
            2 => Some(BootMode::Spifi),
            _ => None,
        }
    }
    #[doc = "Старт из внутреннего ЭСППЗУ"]
    #[inline(always)]
    pub fn is_eeprom(&self) -> bool {
        *self == BootMode::Eeprom
    }
    #[doc = "Старт из системного ОЗУ"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == BootMode::Ram
    }
    #[doc = "Старт из внешней памяти с использованием контроллера SPIFI"]
    #[inline(always)]
    pub fn is_spifi(&self) -> bool {
        *self == BootMode::Spifi
    }
}
impl R {
    #[doc = "Bits 0:1 - Регистр чтения режима загрузки"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new((self.bits & 3) as u8)
    }
}
#[doc = "Регистр режима загрузки\n\nYou can [`read`](crate::Reg::read) this register and get [`boot::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootSpec;
impl crate::RegisterSpec for BootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot::R`](R) reader structure"]
impl crate::Readable for BootSpec {}
#[doc = "`reset()` method sets BOOT to value 0"]
impl crate::Resettable for BootSpec {}
