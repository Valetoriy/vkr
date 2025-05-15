#[doc = "Register `LINE_MUX` writer"]
pub type W = crate::W<LineMuxSpec>;
#[doc = "Выбор порта для линии 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux0 {
    #[doc = "0: MUX_0\\[0\\] = GPIO0_0"]
    Gpio0_0 = 0,
    #[doc = "1: MUX_0\\[1\\] = GPIO0_8"]
    Gpio0_8 = 1,
    #[doc = "2: MUX_0\\[2\\] = GPIO1_0"]
    Gpio1_0 = 2,
    #[doc = "3: MUX_0\\[3\\] = GPIO1_8"]
    Gpio1_8 = 3,
    #[doc = "4: MUX_0\\[4\\] = GPIO2_0"]
    Gpio2_0 = 4,
    #[doc = "5: MUX_0\\[5\\] = GPIO0_4"]
    Gpio0_4 = 5,
    #[doc = "6: MUX_0\\[6\\] = GPIO0_12"]
    Gpio0_12 = 6,
    #[doc = "7: MUX_0\\[7\\] = GPIO1_4"]
    Gpio1_4 = 7,
    #[doc = "8: MUX_0\\[8\\] = GPIO1_12"]
    Gpio1_12 = 8,
    #[doc = "9: MUX_0\\[9\\] = GPIO2_4"]
    Gpio2_4 = 9,
    #[doc = "10: MUX_0\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux0> for u8 {
    #[inline(always)]
    fn from(variant: Mux0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux0 {
    type Ux = u8;
}
impl crate::IsEnum for Mux0 {}
#[doc = "Field `MUX_0` writer - Выбор порта для линии 0"]
pub type Mux0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux0>;
impl<'a, REG> Mux0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_0\\[0\\] = GPIO0_0"]
    #[inline(always)]
    pub fn gpio0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio0_0)
    }
    #[doc = "MUX_0\\[1\\] = GPIO0_8"]
    #[inline(always)]
    pub fn gpio0_8(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio0_8)
    }
    #[doc = "MUX_0\\[2\\] = GPIO1_0"]
    #[inline(always)]
    pub fn gpio1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio1_0)
    }
    #[doc = "MUX_0\\[3\\] = GPIO1_8"]
    #[inline(always)]
    pub fn gpio1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio1_8)
    }
    #[doc = "MUX_0\\[4\\] = GPIO2_0"]
    #[inline(always)]
    pub fn gpio2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio2_0)
    }
    #[doc = "MUX_0\\[5\\] = GPIO0_4"]
    #[inline(always)]
    pub fn gpio0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio0_4)
    }
    #[doc = "MUX_0\\[6\\] = GPIO0_12"]
    #[inline(always)]
    pub fn gpio0_12(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio0_12)
    }
    #[doc = "MUX_0\\[7\\] = GPIO1_4"]
    #[inline(always)]
    pub fn gpio1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio1_4)
    }
    #[doc = "MUX_0\\[8\\] = GPIO1_12"]
    #[inline(always)]
    pub fn gpio1_12(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio1_12)
    }
    #[doc = "MUX_0\\[9\\] = GPIO2_4"]
    #[inline(always)]
    pub fn gpio2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Gpio2_4)
    }
    #[doc = "MUX_0\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux0::Null)
    }
}
#[doc = "Выбор порта для линии 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux1 {
    #[doc = "0: MUX_1\\[0\\] = GPIO0_1"]
    Gpio0_1 = 0,
    #[doc = "1: MUX_1\\[1\\] = GPIO0_9"]
    Gpio0_9 = 1,
    #[doc = "2: MUX_1\\[2\\] = GPIO1_1"]
    Gpio1_1 = 2,
    #[doc = "3: MUX_1\\[3\\] = GPIO1_9"]
    Gpio1_9 = 3,
    #[doc = "4: MUX_1\\[4\\] = GPIO2_1"]
    Gpio2_1 = 4,
    #[doc = "5: MUX_1\\[5\\] = GPIO0_5"]
    Gpio0_5 = 5,
    #[doc = "6: MUX_1\\[6\\] = GPIO0_13"]
    Gpio0_13 = 6,
    #[doc = "7: MUX_1\\[7\\] = GPIO1_5"]
    Gpio1_5 = 7,
    #[doc = "8: MUX_1\\[8\\] = GPIO1_13"]
    Gpio1_13 = 8,
    #[doc = "9: MUX_1\\[9\\] = GPIO2_5"]
    Gpio2_5 = 9,
    #[doc = "10: MUX_1\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux1> for u8 {
    #[inline(always)]
    fn from(variant: Mux1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux1 {
    type Ux = u8;
}
impl crate::IsEnum for Mux1 {}
#[doc = "Field `MUX_1` writer - Выбор порта для линии 1"]
pub type Mux1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux1>;
impl<'a, REG> Mux1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_1\\[0\\] = GPIO0_1"]
    #[inline(always)]
    pub fn gpio0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio0_1)
    }
    #[doc = "MUX_1\\[1\\] = GPIO0_9"]
    #[inline(always)]
    pub fn gpio0_9(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio0_9)
    }
    #[doc = "MUX_1\\[2\\] = GPIO1_1"]
    #[inline(always)]
    pub fn gpio1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio1_1)
    }
    #[doc = "MUX_1\\[3\\] = GPIO1_9"]
    #[inline(always)]
    pub fn gpio1_9(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio1_9)
    }
    #[doc = "MUX_1\\[4\\] = GPIO2_1"]
    #[inline(always)]
    pub fn gpio2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio2_1)
    }
    #[doc = "MUX_1\\[5\\] = GPIO0_5"]
    #[inline(always)]
    pub fn gpio0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio0_5)
    }
    #[doc = "MUX_1\\[6\\] = GPIO0_13"]
    #[inline(always)]
    pub fn gpio0_13(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio0_13)
    }
    #[doc = "MUX_1\\[7\\] = GPIO1_5"]
    #[inline(always)]
    pub fn gpio1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio1_5)
    }
    #[doc = "MUX_1\\[8\\] = GPIO1_13"]
    #[inline(always)]
    pub fn gpio1_13(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio1_13)
    }
    #[doc = "MUX_1\\[9\\] = GPIO2_5"]
    #[inline(always)]
    pub fn gpio2_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Gpio2_5)
    }
    #[doc = "MUX_1\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux1::Null)
    }
}
#[doc = "Выбор порта для линии 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux2 {
    #[doc = "0: MUX_2\\[0\\] = GPIO0_2"]
    Gpio0_2 = 0,
    #[doc = "1: MUX_2\\[1\\] = GPIO0_10"]
    Gpio0_10 = 1,
    #[doc = "2: MUX_2\\[2\\] = GPIO1_2"]
    Gpio1_2 = 2,
    #[doc = "3: MUX_2\\[3\\] = GPIO1_10"]
    Gpio1_10 = 3,
    #[doc = "4: MUX_2\\[4\\] = GPIO2_2"]
    Gpio2_2 = 4,
    #[doc = "5: MUX_2\\[5\\] = GPIO0_6"]
    Gpio0_6 = 5,
    #[doc = "6: MUX_2\\[6\\] = GPIO0_14"]
    Gpio0_14 = 6,
    #[doc = "7: MUX_2\\[7\\] = GPIO1_6"]
    Gpio1_6 = 7,
    #[doc = "8: MUX_2\\[8\\] = GPIO1_14"]
    Gpio1_14 = 8,
    #[doc = "9: MUX_2\\[9\\] = GPIO2_6"]
    Gpio2_6 = 9,
    #[doc = "10: MUX_2\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux2> for u8 {
    #[inline(always)]
    fn from(variant: Mux2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux2 {
    type Ux = u8;
}
impl crate::IsEnum for Mux2 {}
#[doc = "Field `MUX_2` writer - Выбор порта для линии 2"]
pub type Mux2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux2>;
impl<'a, REG> Mux2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_2\\[0\\] = GPIO0_2"]
    #[inline(always)]
    pub fn gpio0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio0_2)
    }
    #[doc = "MUX_2\\[1\\] = GPIO0_10"]
    #[inline(always)]
    pub fn gpio0_10(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio0_10)
    }
    #[doc = "MUX_2\\[2\\] = GPIO1_2"]
    #[inline(always)]
    pub fn gpio1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio1_2)
    }
    #[doc = "MUX_2\\[3\\] = GPIO1_10"]
    #[inline(always)]
    pub fn gpio1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio1_10)
    }
    #[doc = "MUX_2\\[4\\] = GPIO2_2"]
    #[inline(always)]
    pub fn gpio2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio2_2)
    }
    #[doc = "MUX_2\\[5\\] = GPIO0_6"]
    #[inline(always)]
    pub fn gpio0_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio0_6)
    }
    #[doc = "MUX_2\\[6\\] = GPIO0_14"]
    #[inline(always)]
    pub fn gpio0_14(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio0_14)
    }
    #[doc = "MUX_2\\[7\\] = GPIO1_6"]
    #[inline(always)]
    pub fn gpio1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio1_6)
    }
    #[doc = "MUX_2\\[8\\] = GPIO1_14"]
    #[inline(always)]
    pub fn gpio1_14(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio1_14)
    }
    #[doc = "MUX_2\\[9\\] = GPIO2_6"]
    #[inline(always)]
    pub fn gpio2_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Gpio2_6)
    }
    #[doc = "MUX_2\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux2::Null)
    }
}
#[doc = "Выбор порта для линии 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux3 {
    #[doc = "0: MUX_3\\[0\\] = GPIO0_3"]
    Gpio0_3 = 0,
    #[doc = "1: MUX_3\\[1\\] = GPIO0_11"]
    Gpio0_11 = 1,
    #[doc = "2: MUX_3\\[2\\] = GPIO1_3"]
    Gpio1_3 = 2,
    #[doc = "3: MUX_3\\[3\\] = GPIO1_11"]
    Gpio1_11 = 3,
    #[doc = "4: MUX_3\\[4\\] = GPIO2_3"]
    Gpio2_3 = 4,
    #[doc = "5: MUX_3\\[5\\] = GPIO0_7"]
    Gpio0_7 = 5,
    #[doc = "6: MUX_3\\[6\\] = GPIO0_15"]
    Gpio0_15 = 6,
    #[doc = "7: MUX_3\\[7\\] = GPIO1_7"]
    Gpio1_7 = 7,
    #[doc = "8: MUX_3\\[8\\] = GPIO1_15"]
    Gpio1_15 = 8,
    #[doc = "9: MUX_3\\[9\\] = GPIO2_7"]
    Gpio2_7 = 9,
    #[doc = "10: MUX_3\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux3> for u8 {
    #[inline(always)]
    fn from(variant: Mux3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux3 {
    type Ux = u8;
}
impl crate::IsEnum for Mux3 {}
#[doc = "Field `MUX_3` writer - Выбор порта для линии 3"]
pub type Mux3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux3>;
impl<'a, REG> Mux3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_3\\[0\\] = GPIO0_3"]
    #[inline(always)]
    pub fn gpio0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio0_3)
    }
    #[doc = "MUX_3\\[1\\] = GPIO0_11"]
    #[inline(always)]
    pub fn gpio0_11(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio0_11)
    }
    #[doc = "MUX_3\\[2\\] = GPIO1_3"]
    #[inline(always)]
    pub fn gpio1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio1_3)
    }
    #[doc = "MUX_3\\[3\\] = GPIO1_11"]
    #[inline(always)]
    pub fn gpio1_11(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio1_11)
    }
    #[doc = "MUX_3\\[4\\] = GPIO2_3"]
    #[inline(always)]
    pub fn gpio2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio2_3)
    }
    #[doc = "MUX_3\\[5\\] = GPIO0_7"]
    #[inline(always)]
    pub fn gpio0_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio0_7)
    }
    #[doc = "MUX_3\\[6\\] = GPIO0_15"]
    #[inline(always)]
    pub fn gpio0_15(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio0_15)
    }
    #[doc = "MUX_3\\[7\\] = GPIO1_7"]
    #[inline(always)]
    pub fn gpio1_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio1_7)
    }
    #[doc = "MUX_3\\[8\\] = GPIO1_15"]
    #[inline(always)]
    pub fn gpio1_15(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio1_15)
    }
    #[doc = "MUX_3\\[9\\] = GPIO2_7"]
    #[inline(always)]
    pub fn gpio2_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Gpio2_7)
    }
    #[doc = "MUX_3\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux3::Null)
    }
}
#[doc = "Выбор порта для линии 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux4 {
    #[doc = "0: MUX_4\\[0\\] = GPIO0_4"]
    Gpio0_4 = 0,
    #[doc = "1: MUX_4\\[1\\] = GPIO0_12"]
    Gpio0_12 = 1,
    #[doc = "2: MUX_4\\[2\\] = GPIO1_4"]
    Gpio1_4 = 2,
    #[doc = "3: MUX_4\\[3\\] = GPIO1_12"]
    Gpio1_12 = 3,
    #[doc = "4: MUX_4\\[4\\] = GPIO2_4"]
    Gpio2_4 = 4,
    #[doc = "5: MUX_4\\[5\\] = GPIO0_0"]
    Gpio0_0 = 5,
    #[doc = "6: MUX_4\\[6\\] = GPIO0_8"]
    Gpio0_8 = 6,
    #[doc = "7: MUX_4\\[7\\] = GPIO1_0"]
    Gpio1_0 = 7,
    #[doc = "8: MUX_4\\[8\\] = GPIO1_8"]
    Gpio1_8 = 8,
    #[doc = "9: MUX_4\\[9\\] = GPIO2_0"]
    Gpio2_0 = 9,
    #[doc = "10: MUX_4\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux4> for u8 {
    #[inline(always)]
    fn from(variant: Mux4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux4 {
    type Ux = u8;
}
impl crate::IsEnum for Mux4 {}
#[doc = "Field `MUX_4` writer - Выбор порта для линии 4"]
pub type Mux4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux4>;
impl<'a, REG> Mux4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_4\\[0\\] = GPIO0_4"]
    #[inline(always)]
    pub fn gpio0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio0_4)
    }
    #[doc = "MUX_4\\[1\\] = GPIO0_12"]
    #[inline(always)]
    pub fn gpio0_12(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio0_12)
    }
    #[doc = "MUX_4\\[2\\] = GPIO1_4"]
    #[inline(always)]
    pub fn gpio1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio1_4)
    }
    #[doc = "MUX_4\\[3\\] = GPIO1_12"]
    #[inline(always)]
    pub fn gpio1_12(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio1_12)
    }
    #[doc = "MUX_4\\[4\\] = GPIO2_4"]
    #[inline(always)]
    pub fn gpio2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio2_4)
    }
    #[doc = "MUX_4\\[5\\] = GPIO0_0"]
    #[inline(always)]
    pub fn gpio0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio0_0)
    }
    #[doc = "MUX_4\\[6\\] = GPIO0_8"]
    #[inline(always)]
    pub fn gpio0_8(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio0_8)
    }
    #[doc = "MUX_4\\[7\\] = GPIO1_0"]
    #[inline(always)]
    pub fn gpio1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio1_0)
    }
    #[doc = "MUX_4\\[8\\] = GPIO1_8"]
    #[inline(always)]
    pub fn gpio1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio1_8)
    }
    #[doc = "MUX_4\\[9\\] = GPIO2_0"]
    #[inline(always)]
    pub fn gpio2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Gpio2_0)
    }
    #[doc = "MUX_4\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux4::Null)
    }
}
#[doc = "Выбор порта для линии 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux5 {
    #[doc = "0: MUX_5\\[0\\] = GPIO0_5"]
    Gpio0_5 = 0,
    #[doc = "1: MUX_5\\[1\\] = GPIO0_13"]
    Gpio0_13 = 1,
    #[doc = "2: MUX_5\\[2\\] = GPIO1_5"]
    Gpio1_5 = 2,
    #[doc = "3: MUX_5\\[3\\] = GPIO1_13"]
    Gpio1_13 = 3,
    #[doc = "4: MUX_5\\[4\\] = GPIO2_5"]
    Gpio2_5 = 4,
    #[doc = "5: MUX_5\\[5\\] = GPIO0_1"]
    Gpio0_1 = 5,
    #[doc = "6: MUX_5\\[6\\] = GPIO0_9"]
    Gpio0_9 = 6,
    #[doc = "7: MUX_5\\[7\\] = GPIO1_1"]
    Gpio1_1 = 7,
    #[doc = "8: MUX_5\\[8\\] = GPIO1_9"]
    Gpio1_9 = 8,
    #[doc = "9: MUX_5\\[9\\] = GPIO2_1"]
    Gpio2_1 = 9,
    #[doc = "10: MUX_5\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux5> for u8 {
    #[inline(always)]
    fn from(variant: Mux5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux5 {
    type Ux = u8;
}
impl crate::IsEnum for Mux5 {}
#[doc = "Field `MUX_5` writer - Выбор порта для линии 5"]
pub type Mux5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux5>;
impl<'a, REG> Mux5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_5\\[0\\] = GPIO0_5"]
    #[inline(always)]
    pub fn gpio0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio0_5)
    }
    #[doc = "MUX_5\\[1\\] = GPIO0_13"]
    #[inline(always)]
    pub fn gpio0_13(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio0_13)
    }
    #[doc = "MUX_5\\[2\\] = GPIO1_5"]
    #[inline(always)]
    pub fn gpio1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio1_5)
    }
    #[doc = "MUX_5\\[3\\] = GPIO1_13"]
    #[inline(always)]
    pub fn gpio1_13(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio1_13)
    }
    #[doc = "MUX_5\\[4\\] = GPIO2_5"]
    #[inline(always)]
    pub fn gpio2_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio2_5)
    }
    #[doc = "MUX_5\\[5\\] = GPIO0_1"]
    #[inline(always)]
    pub fn gpio0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio0_1)
    }
    #[doc = "MUX_5\\[6\\] = GPIO0_9"]
    #[inline(always)]
    pub fn gpio0_9(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio0_9)
    }
    #[doc = "MUX_5\\[7\\] = GPIO1_1"]
    #[inline(always)]
    pub fn gpio1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio1_1)
    }
    #[doc = "MUX_5\\[8\\] = GPIO1_9"]
    #[inline(always)]
    pub fn gpio1_9(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio1_9)
    }
    #[doc = "MUX_5\\[9\\] = GPIO2_1"]
    #[inline(always)]
    pub fn gpio2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Gpio2_1)
    }
    #[doc = "MUX_5\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux5::Null)
    }
}
#[doc = "Выбор порта для линии 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux6 {
    #[doc = "0: MUX_6\\[0\\] = GPIO0_6"]
    Gpio0_6 = 0,
    #[doc = "1: MUX_6\\[1\\] = GPIO0_14"]
    Gpio0_14 = 1,
    #[doc = "2: MUX_6\\[2\\] = GPIO1_6"]
    Gpio1_6 = 2,
    #[doc = "3: MUX_6\\[3\\] = GPIO1_14"]
    Gpio1_14 = 3,
    #[doc = "4: MUX_6\\[4\\] = GPIO2_6"]
    Gpio2_6 = 4,
    #[doc = "5: MUX_6\\[5\\] = GPIO0_2"]
    Gpio0_2 = 5,
    #[doc = "6: MUX_6\\[6\\] = GPIO0_10"]
    Gpio0_10 = 6,
    #[doc = "7: MUX_6\\[7\\] = GPIO1_2"]
    Gpio1_2 = 7,
    #[doc = "8: MUX_6\\[8\\] = GPIO1_10"]
    Gpio1_10 = 8,
    #[doc = "9: MUX_6\\[9\\] = GPIO2_2"]
    Gpio2_2 = 9,
    #[doc = "10: MUX_6\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux6> for u8 {
    #[inline(always)]
    fn from(variant: Mux6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux6 {
    type Ux = u8;
}
impl crate::IsEnum for Mux6 {}
#[doc = "Field `MUX_6` writer - Выбор порта для линии 6"]
pub type Mux6W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux6>;
impl<'a, REG> Mux6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_6\\[0\\] = GPIO0_6"]
    #[inline(always)]
    pub fn gpio0_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio0_6)
    }
    #[doc = "MUX_6\\[1\\] = GPIO0_14"]
    #[inline(always)]
    pub fn gpio0_14(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio0_14)
    }
    #[doc = "MUX_6\\[2\\] = GPIO1_6"]
    #[inline(always)]
    pub fn gpio1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio1_6)
    }
    #[doc = "MUX_6\\[3\\] = GPIO1_14"]
    #[inline(always)]
    pub fn gpio1_14(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio1_14)
    }
    #[doc = "MUX_6\\[4\\] = GPIO2_6"]
    #[inline(always)]
    pub fn gpio2_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio2_6)
    }
    #[doc = "MUX_6\\[5\\] = GPIO0_2"]
    #[inline(always)]
    pub fn gpio0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio0_2)
    }
    #[doc = "MUX_6\\[6\\] = GPIO0_10"]
    #[inline(always)]
    pub fn gpio0_10(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio0_10)
    }
    #[doc = "MUX_6\\[7\\] = GPIO1_2"]
    #[inline(always)]
    pub fn gpio1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio1_2)
    }
    #[doc = "MUX_6\\[8\\] = GPIO1_10"]
    #[inline(always)]
    pub fn gpio1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio1_10)
    }
    #[doc = "MUX_6\\[9\\] = GPIO2_2"]
    #[inline(always)]
    pub fn gpio2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Gpio2_2)
    }
    #[doc = "MUX_6\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux6::Null)
    }
}
#[doc = "Выбор порта для линии 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux7 {
    #[doc = "0: MUX_7\\[0\\] = GPIO0_7"]
    Gpio0_7 = 0,
    #[doc = "1: MUX_7\\[1\\] = GPIO0_15"]
    Gpio0_15 = 1,
    #[doc = "2: MUX_7\\[2\\] = GPIO1_7"]
    Gpio1_7 = 2,
    #[doc = "3: MUX_7\\[3\\] = GPIO1_15"]
    Gpio1_15 = 3,
    #[doc = "4: MUX_7\\[4\\] = GPIO2_7"]
    Gpio2_7 = 4,
    #[doc = "5: MUX_7\\[5\\] = GPIO0_3"]
    Gpio0_3 = 5,
    #[doc = "6: MUX_7\\[6\\] = GPIO0_11"]
    Gpio0_11 = 6,
    #[doc = "7: MUX_7\\[7\\] = GPIO1_3"]
    Gpio1_3 = 7,
    #[doc = "8: MUX_7\\[8\\] = GPIO1_11"]
    Gpio1_11 = 8,
    #[doc = "9: MUX_7\\[9\\] = GPIO2_3"]
    Gpio2_3 = 9,
    #[doc = "10: MUX_7\\[10\\] = Null"]
    Null = 10,
}
impl From<Mux7> for u8 {
    #[inline(always)]
    fn from(variant: Mux7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux7 {
    type Ux = u8;
}
impl crate::IsEnum for Mux7 {}
#[doc = "Field `MUX_7` writer - Выбор порта для линии 7"]
pub type Mux7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mux7>;
impl<'a, REG> Mux7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUX_7\\[0\\] = GPIO0_7"]
    #[inline(always)]
    pub fn gpio0_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio0_7)
    }
    #[doc = "MUX_7\\[1\\] = GPIO0_15"]
    #[inline(always)]
    pub fn gpio0_15(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio0_15)
    }
    #[doc = "MUX_7\\[2\\] = GPIO1_7"]
    #[inline(always)]
    pub fn gpio1_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio1_7)
    }
    #[doc = "MUX_7\\[3\\] = GPIO1_15"]
    #[inline(always)]
    pub fn gpio1_15(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio1_15)
    }
    #[doc = "MUX_7\\[4\\] = GPIO2_7"]
    #[inline(always)]
    pub fn gpio2_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio2_7)
    }
    #[doc = "MUX_7\\[5\\] = GPIO0_3"]
    #[inline(always)]
    pub fn gpio0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio0_3)
    }
    #[doc = "MUX_7\\[6\\] = GPIO0_11"]
    #[inline(always)]
    pub fn gpio0_11(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio0_11)
    }
    #[doc = "MUX_7\\[7\\] = GPIO1_3"]
    #[inline(always)]
    pub fn gpio1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio1_3)
    }
    #[doc = "MUX_7\\[8\\] = GPIO1_11"]
    #[inline(always)]
    pub fn gpio1_11(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio1_11)
    }
    #[doc = "MUX_7\\[9\\] = GPIO2_3"]
    #[inline(always)]
    pub fn gpio2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Gpio2_3)
    }
    #[doc = "MUX_7\\[10\\] = Null"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Mux7::Null)
    }
}
impl W {
    #[doc = "Bits 0:3 - Выбор порта для линии 0"]
    #[inline(always)]
    pub fn mux_0(&mut self) -> Mux0W<LineMuxSpec> {
        Mux0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Выбор порта для линии 1"]
    #[inline(always)]
    pub fn mux_1(&mut self) -> Mux1W<LineMuxSpec> {
        Mux1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Выбор порта для линии 2"]
    #[inline(always)]
    pub fn mux_2(&mut self) -> Mux2W<LineMuxSpec> {
        Mux2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Выбор порта для линии 3"]
    #[inline(always)]
    pub fn mux_3(&mut self) -> Mux3W<LineMuxSpec> {
        Mux3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Выбор порта для линии 4"]
    #[inline(always)]
    pub fn mux_4(&mut self) -> Mux4W<LineMuxSpec> {
        Mux4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Выбор порта для линии 5"]
    #[inline(always)]
    pub fn mux_5(&mut self) -> Mux5W<LineMuxSpec> {
        Mux5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Выбор порта для линии 6"]
    #[inline(always)]
    pub fn mux_6(&mut self) -> Mux6W<LineMuxSpec> {
        Mux6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Выбор порта для линии 7"]
    #[inline(always)]
    pub fn mux_7(&mut self) -> Mux7W<LineMuxSpec> {
        Mux7W::new(self, 28)
    }
}
#[doc = "Управление мультиплексорами всех 8-ми линий. 4 бита кодируют выбор 1-го из 10-ти доступных портов: MUX_0 – \\[3:0\\]; MUX_1 – \\[7:4\\]; MUX_2 – \\[11:8\\]; MUX_3 – \\[15:12\\]; MUX_4 – \\[19:16\\]; MUX_5 – \\[23:20\\]; MUX_6 – \\[27:24\\]; MUX_7 – \\[31:28\\] Внимание!!! Схемотехническая ошибка в MIK32V2 ограничила чтение регистра LINE_MUX 8-ю младшими битами, а остальные читаются как «0». Запись осуществляется корректно во все 32 бита. Поэтому регистр помечен доступным только на запись. Примечание: На входы неиспользуемых линиий рекомендуется подать «заглушку» в виде логического «0» записью числа 10 в настройках мультиплексоров\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`line_mux::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LineMuxSpec;
impl crate::RegisterSpec for LineMuxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`line_mux::W`](W) writer structure"]
impl crate::Writable for LineMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINE_MUX to value 0"]
impl crate::Resettable for LineMuxSpec {}
