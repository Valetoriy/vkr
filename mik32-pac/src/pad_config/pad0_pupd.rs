#[doc = "Register `PAD0_PUPD` reader"]
pub type R = crate::R<Pad0PupdSpec>;
#[doc = "Register `PAD0_PUPD` writer"]
pub type W = crate::W<Pad0PupdSpec>;
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_0 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_0> for u8 {
    #[inline(always)]
    fn from(variant: Port0_0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_0 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_0 {}
#[doc = "Field `Port_0_0` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_0R = crate::FieldReader<Port0_0>;
impl Port0_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_0> {
        match self.bits {
            0 => Some(Port0_0::PullNone),
            1 => Some(Port0_0::PullUp),
            2 => Some(Port0_0::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_0::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_0::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_0::PullDown
    }
}
#[doc = "Field `Port_0_0` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_0>;
impl<'a, REG> Port0_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_0::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_0::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_0::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_1 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_1> for u8 {
    #[inline(always)]
    fn from(variant: Port0_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_1 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_1 {}
#[doc = "Field `Port_0_1` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_1R = crate::FieldReader<Port0_1>;
impl Port0_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_1> {
        match self.bits {
            0 => Some(Port0_1::PullNone),
            1 => Some(Port0_1::PullUp),
            2 => Some(Port0_1::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_1::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_1::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_1::PullDown
    }
}
#[doc = "Field `Port_0_1` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_1>;
impl<'a, REG> Port0_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_1::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_1::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_1::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_2 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_2> for u8 {
    #[inline(always)]
    fn from(variant: Port0_2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_2 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_2 {}
#[doc = "Field `Port_0_2` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_2R = crate::FieldReader<Port0_2>;
impl Port0_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_2> {
        match self.bits {
            0 => Some(Port0_2::PullNone),
            1 => Some(Port0_2::PullUp),
            2 => Some(Port0_2::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_2::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_2::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_2::PullDown
    }
}
#[doc = "Field `Port_0_2` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_2>;
impl<'a, REG> Port0_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_2::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_2::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_2::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_3 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_3> for u8 {
    #[inline(always)]
    fn from(variant: Port0_3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_3 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_3 {}
#[doc = "Field `Port_0_3` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_3R = crate::FieldReader<Port0_3>;
impl Port0_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_3> {
        match self.bits {
            0 => Some(Port0_3::PullNone),
            1 => Some(Port0_3::PullUp),
            2 => Some(Port0_3::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_3::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_3::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_3::PullDown
    }
}
#[doc = "Field `Port_0_3` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_3>;
impl<'a, REG> Port0_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_3::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_3::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_3::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_4 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_4> for u8 {
    #[inline(always)]
    fn from(variant: Port0_4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_4 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_4 {}
#[doc = "Field `Port_0_4` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_4R = crate::FieldReader<Port0_4>;
impl Port0_4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_4> {
        match self.bits {
            0 => Some(Port0_4::PullNone),
            1 => Some(Port0_4::PullUp),
            2 => Some(Port0_4::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_4::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_4::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_4::PullDown
    }
}
#[doc = "Field `Port_0_4` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_4>;
impl<'a, REG> Port0_4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_4::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_4::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_4::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_5 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_5> for u8 {
    #[inline(always)]
    fn from(variant: Port0_5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_5 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_5 {}
#[doc = "Field `Port_0_5` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_5R = crate::FieldReader<Port0_5>;
impl Port0_5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_5> {
        match self.bits {
            0 => Some(Port0_5::PullNone),
            1 => Some(Port0_5::PullUp),
            2 => Some(Port0_5::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_5::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_5::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_5::PullDown
    }
}
#[doc = "Field `Port_0_5` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_5>;
impl<'a, REG> Port0_5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_5::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_5::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_5::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_6 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_6> for u8 {
    #[inline(always)]
    fn from(variant: Port0_6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_6 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_6 {}
#[doc = "Field `Port_0_6` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_6R = crate::FieldReader<Port0_6>;
impl Port0_6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_6> {
        match self.bits {
            0 => Some(Port0_6::PullNone),
            1 => Some(Port0_6::PullUp),
            2 => Some(Port0_6::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_6::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_6::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_6::PullDown
    }
}
#[doc = "Field `Port_0_6` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_6>;
impl<'a, REG> Port0_6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_6::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_6::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_6::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_7 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_7> for u8 {
    #[inline(always)]
    fn from(variant: Port0_7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_7 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_7 {}
#[doc = "Field `Port_0_7` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_7R = crate::FieldReader<Port0_7>;
impl Port0_7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_7> {
        match self.bits {
            0 => Some(Port0_7::PullNone),
            1 => Some(Port0_7::PullUp),
            2 => Some(Port0_7::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_7::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_7::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_7::PullDown
    }
}
#[doc = "Field `Port_0_7` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_7>;
impl<'a, REG> Port0_7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_7::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_7::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_7::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_8 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_8> for u8 {
    #[inline(always)]
    fn from(variant: Port0_8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_8 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_8 {}
#[doc = "Field `Port_0_8` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_8R = crate::FieldReader<Port0_8>;
impl Port0_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_8> {
        match self.bits {
            0 => Some(Port0_8::PullNone),
            1 => Some(Port0_8::PullUp),
            2 => Some(Port0_8::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_8::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_8::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_8::PullDown
    }
}
#[doc = "Field `Port_0_8` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_8>;
impl<'a, REG> Port0_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_8::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_8::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_8::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_9 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_9> for u8 {
    #[inline(always)]
    fn from(variant: Port0_9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_9 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_9 {}
#[doc = "Field `Port_0_9` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_9R = crate::FieldReader<Port0_9>;
impl Port0_9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_9> {
        match self.bits {
            0 => Some(Port0_9::PullNone),
            1 => Some(Port0_9::PullUp),
            2 => Some(Port0_9::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_9::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_9::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_9::PullDown
    }
}
#[doc = "Field `Port_0_9` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_9>;
impl<'a, REG> Port0_9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_9::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_9::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_9::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_10 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_10> for u8 {
    #[inline(always)]
    fn from(variant: Port0_10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_10 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_10 {}
#[doc = "Field `Port_0_10` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_10R = crate::FieldReader<Port0_10>;
impl Port0_10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_10> {
        match self.bits {
            0 => Some(Port0_10::PullNone),
            1 => Some(Port0_10::PullUp),
            2 => Some(Port0_10::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_10::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_10::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_10::PullDown
    }
}
#[doc = "Field `Port_0_10` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_10>;
impl<'a, REG> Port0_10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_10::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_10::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_10::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_11 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_11> for u8 {
    #[inline(always)]
    fn from(variant: Port0_11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_11 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_11 {}
#[doc = "Field `Port_0_11` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_11R = crate::FieldReader<Port0_11>;
impl Port0_11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_11> {
        match self.bits {
            0 => Some(Port0_11::PullNone),
            1 => Some(Port0_11::PullUp),
            2 => Some(Port0_11::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_11::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_11::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_11::PullDown
    }
}
#[doc = "Field `Port_0_11` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_11>;
impl<'a, REG> Port0_11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_11::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_11::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_11::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_12 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_12> for u8 {
    #[inline(always)]
    fn from(variant: Port0_12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_12 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_12 {}
#[doc = "Field `Port_0_12` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_12R = crate::FieldReader<Port0_12>;
impl Port0_12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_12> {
        match self.bits {
            0 => Some(Port0_12::PullNone),
            1 => Some(Port0_12::PullUp),
            2 => Some(Port0_12::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_12::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_12::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_12::PullDown
    }
}
#[doc = "Field `Port_0_12` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_12>;
impl<'a, REG> Port0_12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_12::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_12::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_12::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_13 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_13> for u8 {
    #[inline(always)]
    fn from(variant: Port0_13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_13 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_13 {}
#[doc = "Field `Port_0_13` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_13R = crate::FieldReader<Port0_13>;
impl Port0_13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_13> {
        match self.bits {
            0 => Some(Port0_13::PullNone),
            1 => Some(Port0_13::PullUp),
            2 => Some(Port0_13::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_13::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_13::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_13::PullDown
    }
}
#[doc = "Field `Port_0_13` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_13>;
impl<'a, REG> Port0_13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_13::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_13::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_13::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_14 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_14> for u8 {
    #[inline(always)]
    fn from(variant: Port0_14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_14 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_14 {}
#[doc = "Field `Port_0_14` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_14R = crate::FieldReader<Port0_14>;
impl Port0_14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_14> {
        match self.bits {
            0 => Some(Port0_14::PullNone),
            1 => Some(Port0_14::PullUp),
            2 => Some(Port0_14::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_14::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_14::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_14::PullDown
    }
}
#[doc = "Field `Port_0_14` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_14>;
impl<'a, REG> Port0_14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_14::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_14::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_14::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0_15 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port0_15> for u8 {
    #[inline(always)]
    fn from(variant: Port0_15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0_15 {
    type Ux = u8;
}
impl crate::IsEnum for Port0_15 {}
#[doc = "Field `Port_0_15` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_15R = crate::FieldReader<Port0_15>;
impl Port0_15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port0_15> {
        match self.bits {
            0 => Some(Port0_15::PullNone),
            1 => Some(Port0_15::PullUp),
            2 => Some(Port0_15::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port0_15::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port0_15::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port0_15::PullDown
    }
}
#[doc = "Field `Port_0_15` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port0_15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port0_15>;
impl<'a, REG> Port0_15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_15::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_15::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port0_15::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_0(&self) -> Port0_0R {
        Port0_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_1(&self) -> Port0_1R {
        Port0_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_2(&self) -> Port0_2R {
        Port0_2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_3(&self) -> Port0_3R {
        Port0_3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_4(&self) -> Port0_4R {
        Port0_4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_5(&self) -> Port0_5R {
        Port0_5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_6(&self) -> Port0_6R {
        Port0_6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_7(&self) -> Port0_7R {
        Port0_7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_8(&self) -> Port0_8R {
        Port0_8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_9(&self) -> Port0_9R {
        Port0_9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_10(&self) -> Port0_10R {
        Port0_10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_11(&self) -> Port0_11R {
        Port0_11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_12(&self) -> Port0_12R {
        Port0_12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_13(&self) -> Port0_13R {
        Port0_13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_14(&self) -> Port0_14R {
        Port0_14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_15(&self) -> Port0_15R {
        Port0_15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_0(&mut self) -> Port0_0W<Pad0PupdSpec> {
        Port0_0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_1(&mut self) -> Port0_1W<Pad0PupdSpec> {
        Port0_1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_2(&mut self) -> Port0_2W<Pad0PupdSpec> {
        Port0_2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_3(&mut self) -> Port0_3W<Pad0PupdSpec> {
        Port0_3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_4(&mut self) -> Port0_4W<Pad0PupdSpec> {
        Port0_4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_5(&mut self) -> Port0_5W<Pad0PupdSpec> {
        Port0_5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_6(&mut self) -> Port0_6W<Pad0PupdSpec> {
        Port0_6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_7(&mut self) -> Port0_7W<Pad0PupdSpec> {
        Port0_7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_8(&mut self) -> Port0_8W<Pad0PupdSpec> {
        Port0_8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_9(&mut self) -> Port0_9W<Pad0PupdSpec> {
        Port0_9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_10(&mut self) -> Port0_10W<Pad0PupdSpec> {
        Port0_10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_11(&mut self) -> Port0_11W<Pad0PupdSpec> {
        Port0_11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_12(&mut self) -> Port0_12W<Pad0PupdSpec> {
        Port0_12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_13(&mut self) -> Port0_13W<Pad0PupdSpec> {
        Port0_13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_14(&mut self) -> Port0_14W<Pad0PupdSpec> {
        Port0_14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_0_15(&mut self) -> Port0_15W<Pad0PupdSpec> {
        Port0_15W::new(self, 30)
    }
}
#[doc = "Управление резисторами подтяжки выводов PORT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0_pupd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0_pupd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad0PupdSpec;
impl crate::RegisterSpec for Pad0PupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad0_pupd::R`](R) reader structure"]
impl crate::Readable for Pad0PupdSpec {}
#[doc = "`write(|w| ..)` method takes [`pad0_pupd::W`](W) writer structure"]
impl crate::Writable for Pad0PupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD0_PUPD to value 0"]
impl crate::Resettable for Pad0PupdSpec {}
