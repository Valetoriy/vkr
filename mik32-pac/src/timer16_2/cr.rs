#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ENABLE` reader - Разрешение TIMER16. Бит ENABLE устанавливается и очищается программно"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Разрешение TIMER16. Бит ENABLE устанавливается и очищается программно"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNGSTRT` reader - Запуск TIMER16 в одиночном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса, как только обнаруживается внешний триггер. Если этот бит установлен, когда TIMER16 находится в режиме непрерывного счёта, то TIMER16 остановится при следующем совпадении регистров ARR и CNT. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
pub type SngstrtR = crate::BitReader;
#[doc = "Field `SNGSTRT` writer - Запуск TIMER16 в одиночном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса, как только обнаруживается внешний триггер. Если этот бит установлен, когда TIMER16 находится в режиме непрерывного счёта, то TIMER16 остановится при следующем совпадении регистров ARR и CNT. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
pub type SngstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTRT` reader - Запуск TIMER16 в непрерывном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в непрерывном режиме. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает таймер в непрерывном режиме, как только будет обнаружен внешний триггер. Если этот бит установлен, когда идёт счёт в режиме одиночного импульса, то таймер не остановится при очередном совпадении регистров ARR и CNT, и счётчик TIMER16 продолжит счёт в непрерывном режиме. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
pub type CntstrtR = crate::BitReader;
#[doc = "Field `CNTSTRT` writer - Запуск TIMER16 в непрерывном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в непрерывном режиме. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает таймер в непрерывном режиме, как только будет обнаружен внешний триггер. Если этот бит установлен, когда идёт счёт в режиме одиночного импульса, то таймер не остановится при очередном совпадении регистров ARR и CNT, и счётчик TIMER16 продолжит счёт в непрерывном режиме. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
pub type CntstrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Разрешение TIMER16. Бит ENABLE устанавливается и очищается программно"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Запуск TIMER16 в одиночном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса, как только обнаруживается внешний триггер. Если этот бит установлен, когда TIMER16 находится в режиме непрерывного счёта, то TIMER16 остановится при следующем совпадении регистров ARR и CNT. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
    #[inline(always)]
    pub fn sngstrt(&self) -> SngstrtR {
        SngstrtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Запуск TIMER16 в непрерывном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в непрерывном режиме. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает таймер в непрерывном режиме, как только будет обнаружен внешний триггер. Если этот бит установлен, когда идёт счёт в режиме одиночного импульса, то таймер не остановится при очередном совпадении регистров ARR и CNT, и счётчик TIMER16 продолжит счёт в непрерывном режиме. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CntstrtR {
        CntstrtR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Разрешение TIMER16. Бит ENABLE устанавливается и очищается программно"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Запуск TIMER16 в одиночном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает TIMER16 в режиме одиночного импульса, как только обнаруживается внешний триггер. Если этот бит установлен, когда TIMER16 находится в режиме непрерывного счёта, то TIMER16 остановится при следующем совпадении регистров ARR и CNT. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SngstrtW<CrSpec> {
        SngstrtW::new(self, 1)
    }
    #[doc = "Bit 2 - Запуск TIMER16 в непрерывном режиме. Этот бит устанавливается программно и очищается аппаратно. В случае программного запуска (TRIGEN\\[1:0\\] = 0b00), установка этого бита запускает TIMER16 в непрерывном режиме. Если программный запуск отключён (TRIGEN\\[1:0\\] отличен от 0b00), установка этого бита запускает таймер в непрерывном режиме, как только будет обнаружен внешний триггер. Если этот бит установлен, когда идёт счёт в режиме одиночного импульса, то таймер не остановится при очередном совпадении регистров ARR и CNT, и счётчик TIMER16 продолжит счёт в непрерывном режиме. Этот бит может быть установлен только при включённом TIMER16. Он будет автоматически сброшен аппаратными средствами"]
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CntstrtW<CrSpec> {
        CntstrtW::new(self, 2)
    }
}
#[doc = "Регистр управления\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
