#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `CMPM` reader - Совпадение сравнения. Бит CMPM устанавливается аппаратными средствами, чтобы сообщить приложению, что значение регистра CNT достигло значения регистра CMP"]
pub type CmpmR = crate::BitReader;
#[doc = "Field `ARRM` reader - Cоответствие автозагрузки. ARRM устанавливается аппаратурой, чтобы сообщить приложению, что значение регистра CNT достигло значения регистра ARR"]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - Cобытие фронта внешнего триггера. EXTTRIG устанавливается аппаратно, чтобы сообщить приложению, что на выбранном входе внешнего триггера возник достоверный фронт импульса. Если триггер игнорируется, так как таймер уже запущен, то этот флаг не устанавливается"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `CMPOK` reader - Обновление регистра сравнения OK. CMPOK устанавливается аппаратными средствами, чтобы сообщить приложению, что операция записи в регистр CMP шины APB_P успешно завершена"]
pub type CmpokR = crate::BitReader;
#[doc = "Field `ARROK` reader - Изменение направления счётчика с «вниз» на «вверх». В режиме энкодера бит UP устанавливается аппаратно, чтобы сообщить приложению, что направление счётчика изменилось с «вниз» на «вверх»"]
pub type ArrokR = crate::BitReader;
#[doc = "Field `UP` reader - Изменение направления счётчика с «вниз» на «вверх». В режиме энкодера бит UP устанавливается аппаратно, чтобы сообщить приложению, что направление счётчика изменилось с «вниз» на «вверх»"]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - Изменение направления счётчика с «вверх» на «вниз». В режиме энкодера бит DOWN устанавливается аппаратно, чтобы сообщить приложению, что направление счётчика изменилось с «вверх» на «вниз»"]
pub type DownR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Совпадение сравнения. Бит CMPM устанавливается аппаратными средствами, чтобы сообщить приложению, что значение регистра CNT достигло значения регистра CMP"]
    #[inline(always)]
    pub fn cmpm(&self) -> CmpmR {
        CmpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cоответствие автозагрузки. ARRM устанавливается аппаратурой, чтобы сообщить приложению, что значение регистра CNT достигло значения регистра ARR"]
    #[inline(always)]
    pub fn arrm(&self) -> ArrmR {
        ArrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cобытие фронта внешнего триггера. EXTTRIG устанавливается аппаратно, чтобы сообщить приложению, что на выбранном входе внешнего триггера возник достоверный фронт импульса. Если триггер игнорируется, так как таймер уже запущен, то этот флаг не устанавливается"]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Обновление регистра сравнения OK. CMPOK устанавливается аппаратными средствами, чтобы сообщить приложению, что операция записи в регистр CMP шины APB_P успешно завершена"]
    #[inline(always)]
    pub fn cmpok(&self) -> CmpokR {
        CmpokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Изменение направления счётчика с «вниз» на «вверх». В режиме энкодера бит UP устанавливается аппаратно, чтобы сообщить приложению, что направление счётчика изменилось с «вниз» на «вверх»"]
    #[inline(always)]
    pub fn arrok(&self) -> ArrokR {
        ArrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Изменение направления счётчика с «вниз» на «вверх». В режиме энкодера бит UP устанавливается аппаратно, чтобы сообщить приложению, что направление счётчика изменилось с «вниз» на «вверх»"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Изменение направления счётчика с «вверх» на «вниз». В режиме энкодера бит DOWN устанавливается аппаратно, чтобы сообщить приложению, что направление счётчика изменилось с «вверх» на «вниз»"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Регистр флагов прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
