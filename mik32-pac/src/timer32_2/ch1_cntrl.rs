#[doc = "Register `CH1_CNTRL` reader"]
pub type R = crate::R<Ch1CntrlSpec>;
#[doc = "Register `CH1_CNTRL` writer"]
pub type W = crate::W<Ch1CntrlSpec>;
#[doc = "Управление фильтрацией входных помех на входе ic_port: 0 – фильтрация выключена; 1 – фильтрация выключена\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noise {
    #[doc = "0: Фильтрация выключена"]
    Enable = 0,
    #[doc = "1: Фильтрация включена"]
    Disable = 1,
}
impl From<Noise> for bool {
    #[inline(always)]
    fn from(variant: Noise) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOISE` reader - Управление фильтрацией входных помех на входе ic_port: 0 – фильтрация выключена; 1 – фильтрация выключена"]
pub type NoiseR = crate::BitReader<Noise>;
impl NoiseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Noise {
        match self.bits {
            false => Noise::Enable,
            true => Noise::Disable,
        }
    }
    #[doc = "Фильтрация выключена"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Noise::Enable
    }
    #[doc = "Фильтрация включена"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Noise::Disable
    }
}
#[doc = "Field `NOISE` writer - Управление фильтрацией входных помех на входе ic_port: 0 – фильтрация выключена; 1 – фильтрация выключена"]
pub type NoiseW<'a, REG> = crate::BitWriter<'a, REG, Noise>;
impl<'a, REG> NoiseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Фильтрация выключена"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Noise::Enable)
    }
    #[doc = "Фильтрация включена"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Noise::Disable)
    }
}
#[doc = "Режим сигнала захвата\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    #[doc = "0: по фронту"]
    Front = 0,
    #[doc = "1: по спаду"]
    Back = 1,
}
impl From<Edge> for bool {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - Режим сигнала захвата"]
pub type EdgeR = crate::BitReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            false => Edge::Front,
            true => Edge::Back,
        }
    }
    #[doc = "по фронту"]
    #[inline(always)]
    pub fn is_front(&self) -> bool {
        *self == Edge::Front
    }
    #[doc = "по спаду"]
    #[inline(always)]
    pub fn is_back(&self) -> bool {
        *self == Edge::Back
    }
}
#[doc = "Field `EDGE` writer - Режим сигнала захвата"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "по фронту"]
    #[inline(always)]
    pub fn front(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Front)
    }
    #[doc = "по спаду"]
    #[inline(always)]
    pub fn back(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Back)
    }
}
#[doc = "Режим работы канала\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "1: Режим сравнения (для timer0)"]
    Compare = 1,
    #[doc = "2: Режим захвата(для timer0)"]
    Capture = 2,
    #[doc = "3: ШИМ"]
    Pwm = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Режим работы канала"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            1 => Some(Mode::Compare),
            2 => Some(Mode::Capture),
            3 => Some(Mode::Pwm),
            _ => None,
        }
    }
    #[doc = "Режим сравнения (для timer0)"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == Mode::Compare
    }
    #[doc = "Режим захвата(для timer0)"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Mode::Capture
    }
    #[doc = "ШИМ"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Mode::Pwm
    }
}
#[doc = "Field `MODE` writer - Режим работы канала"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Режим сравнения (для timer0)"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Compare)
    }
    #[doc = "Режим захвата(для timer0)"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Capture)
    }
    #[doc = "ШИМ"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Pwm)
    }
}
#[doc = "Field `EN` reader - Включение/выключение канала"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Включение/выключение канала"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Тип вывода в режиме ШИМ инвертирование\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwmInv {
    #[doc = "0: Прямой (не инвертированный) выход"]
    Direct = 0,
    #[doc = "1: Инвертированный выход"]
    Inverted = 1,
}
impl From<PwmInv> for bool {
    #[inline(always)]
    fn from(variant: PwmInv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM_INV` reader - Тип вывода в режиме ШИМ инвертирование"]
pub type PwmInvR = crate::BitReader<PwmInv>;
impl PwmInvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwmInv {
        match self.bits {
            false => PwmInv::Direct,
            true => PwmInv::Inverted,
        }
    }
    #[doc = "Прямой (не инвертированный) выход"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == PwmInv::Direct
    }
    #[doc = "Инвертированный выход"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == PwmInv::Inverted
    }
}
#[doc = "Field `PWM_INV` writer - Тип вывода в режиме ШИМ инвертирование"]
pub type PwmInvW<'a, REG> = crate::BitWriter<'a, REG, PwmInv>;
impl<'a, REG> PwmInvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прямой (не инвертированный) выход"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(PwmInv::Direct)
    }
    #[doc = "Инвертированный выход"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(PwmInv::Inverted)
    }
}
#[doc = "Направление передачи данных. Устанавливается автоматически в зависимости от режима работы (1 - выход, 0 - вход)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "1: Выход"]
    Output = 1,
    #[doc = "0: Вход"]
    Input = 0,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Направление передачи данных. Устанавливается автоматически в зависимости от режима работы (1 - выход, 0 - вход)"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            true => Dir::Output,
            false => Dir::Input,
        }
    }
    #[doc = "Выход"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Dir::Output
    }
    #[doc = "Вход"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Dir::Input
    }
}
impl R {
    #[doc = "Bit 0 - Управление фильтрацией входных помех на входе ic_port: 0 – фильтрация выключена; 1 – фильтрация выключена"]
    #[inline(always)]
    pub fn noise(&self) -> NoiseR {
        NoiseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Режим сигнала захвата"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Режим работы канала"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Включение/выключение канала"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Тип вывода в режиме ШИМ инвертирование"]
    #[inline(always)]
    pub fn pwm_inv(&self) -> PwmInvR {
        PwmInvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Направление передачи данных. Устанавливается автоматически в зависимости от режима работы (1 - выход, 0 - вход)"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Управление фильтрацией входных помех на входе ic_port: 0 – фильтрация выключена; 1 – фильтрация выключена"]
    #[inline(always)]
    pub fn noise(&mut self) -> NoiseW<Ch1CntrlSpec> {
        NoiseW::new(self, 0)
    }
    #[doc = "Bit 4 - Режим сигнала захвата"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<Ch1CntrlSpec> {
        EdgeW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Режим работы канала"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<Ch1CntrlSpec> {
        ModeW::new(self, 5)
    }
    #[doc = "Bit 7 - Включение/выключение канала"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Ch1CntrlSpec> {
        EnW::new(self, 7)
    }
    #[doc = "Bit 8 - Тип вывода в режиме ШИМ инвертирование"]
    #[inline(always)]
    pub fn pwm_inv(&mut self) -> PwmInvW<Ch1CntrlSpec> {
        PwmInvW::new(self, 8)
    }
}
#[doc = "Конфигурационный регистр 1 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_cntrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_cntrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1CntrlSpec;
impl crate::RegisterSpec for Ch1CntrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_cntrl::R`](R) reader structure"]
impl crate::Readable for Ch1CntrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1_cntrl::W`](W) writer structure"]
impl crate::Writable for Ch1CntrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1_CNTRL to value 0"]
impl crate::Resettable for Ch1CntrlSpec {}
