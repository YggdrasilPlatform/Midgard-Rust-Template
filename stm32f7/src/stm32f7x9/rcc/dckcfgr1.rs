///Reader of register DCKCFGR1
pub type R = crate::R<u32, super::DCKCFGR1>;
///Writer for register DCKCFGR1
pub type W = crate::W<u32, super::DCKCFGR1>;
///Register DCKCFGR1 `reset()`'s with value 0x2000_3000
impl crate::ResetValue for super::DCKCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_3000
    }
}
///PLLI2S division factor for SAI1 clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLI2SDIVQ_A {
    ///0: PLLI2SDIVQ = /1
    DIV1 = 0,
    ///1: PLLI2SDIVQ = /2
    DIV2 = 1,
    ///2: PLLI2SDIVQ = /3
    DIV3 = 2,
    ///3: PLLI2SDIVQ = /4
    DIV4 = 3,
    ///4: PLLI2SDIVQ = /5
    DIV5 = 4,
    ///5: PLLI2SDIVQ = /6
    DIV6 = 5,
    ///6: PLLI2SDIVQ = /7
    DIV7 = 6,
    ///7: PLLI2SDIVQ = /8
    DIV8 = 7,
    ///8: PLLI2SDIVQ = /9
    DIV9 = 8,
    ///9: PLLI2SDIVQ = /10
    DIV10 = 9,
    ///10: PLLI2SDIVQ = /11
    DIV11 = 10,
    ///11: PLLI2SDIVQ = /12
    DIV12 = 11,
    ///12: PLLI2SDIVQ = /13
    DIV13 = 12,
    ///13: PLLI2SDIVQ = /14
    DIV14 = 13,
    ///14: PLLI2SDIVQ = /15
    DIV15 = 14,
    ///15: PLLI2SDIVQ = /16
    DIV16 = 15,
    ///16: PLLI2SDIVQ = /17
    DIV17 = 16,
    ///17: PLLI2SDIVQ = /18
    DIV18 = 17,
    ///18: PLLI2SDIVQ = /19
    DIV19 = 18,
    ///19: PLLI2SDIVQ = /20
    DIV20 = 19,
    ///20: PLLI2SDIVQ = /21
    DIV21 = 20,
    ///21: PLLI2SDIVQ = /22
    DIV22 = 21,
    ///22: PLLI2SDIVQ = /23
    DIV23 = 22,
    ///23: PLLI2SDIVQ = /24
    DIV24 = 23,
    ///24: PLLI2SDIVQ = /25
    DIV25 = 24,
    ///25: PLLI2SDIVQ = /26
    DIV26 = 25,
    ///26: PLLI2SDIVQ = /27
    DIV27 = 26,
    ///27: PLLI2SDIVQ = /28
    DIV28 = 27,
    ///28: PLLI2SDIVQ = /29
    DIV29 = 28,
    ///29: PLLI2SDIVQ = /30
    DIV30 = 29,
    ///30: PLLI2SDIVQ = /31
    DIV31 = 30,
    ///31: PLLI2SDIVQ = /32
    DIV32 = 31,
}
impl From<PLLI2SDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SDIVQ_A) -> Self {
        variant as _
    }
}
///Reader of field `PLLI2SDIVQ`
pub type PLLI2SDIVQ_R = crate::R<u8, PLLI2SDIVQ_A>;
impl PLLI2SDIVQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SDIVQ_A {
        match self.bits {
            0 => PLLI2SDIVQ_A::DIV1,
            1 => PLLI2SDIVQ_A::DIV2,
            2 => PLLI2SDIVQ_A::DIV3,
            3 => PLLI2SDIVQ_A::DIV4,
            4 => PLLI2SDIVQ_A::DIV5,
            5 => PLLI2SDIVQ_A::DIV6,
            6 => PLLI2SDIVQ_A::DIV7,
            7 => PLLI2SDIVQ_A::DIV8,
            8 => PLLI2SDIVQ_A::DIV9,
            9 => PLLI2SDIVQ_A::DIV10,
            10 => PLLI2SDIVQ_A::DIV11,
            11 => PLLI2SDIVQ_A::DIV12,
            12 => PLLI2SDIVQ_A::DIV13,
            13 => PLLI2SDIVQ_A::DIV14,
            14 => PLLI2SDIVQ_A::DIV15,
            15 => PLLI2SDIVQ_A::DIV16,
            16 => PLLI2SDIVQ_A::DIV17,
            17 => PLLI2SDIVQ_A::DIV18,
            18 => PLLI2SDIVQ_A::DIV19,
            19 => PLLI2SDIVQ_A::DIV20,
            20 => PLLI2SDIVQ_A::DIV21,
            21 => PLLI2SDIVQ_A::DIV22,
            22 => PLLI2SDIVQ_A::DIV23,
            23 => PLLI2SDIVQ_A::DIV24,
            24 => PLLI2SDIVQ_A::DIV25,
            25 => PLLI2SDIVQ_A::DIV26,
            26 => PLLI2SDIVQ_A::DIV27,
            27 => PLLI2SDIVQ_A::DIV28,
            28 => PLLI2SDIVQ_A::DIV29,
            29 => PLLI2SDIVQ_A::DIV30,
            30 => PLLI2SDIVQ_A::DIV31,
            31 => PLLI2SDIVQ_A::DIV32,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV2
    }
    ///Checks if the value of the field is `DIV3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV3
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV4
    }
    ///Checks if the value of the field is `DIV5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV5
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV6
    }
    ///Checks if the value of the field is `DIV7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV7
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV8
    }
    ///Checks if the value of the field is `DIV9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV9
    }
    ///Checks if the value of the field is `DIV10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV10
    }
    ///Checks if the value of the field is `DIV11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV11
    }
    ///Checks if the value of the field is `DIV12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV12
    }
    ///Checks if the value of the field is `DIV13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV13
    }
    ///Checks if the value of the field is `DIV14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV14
    }
    ///Checks if the value of the field is `DIV15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV15
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV16
    }
    ///Checks if the value of the field is `DIV17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV17
    }
    ///Checks if the value of the field is `DIV18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV18
    }
    ///Checks if the value of the field is `DIV19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV19
    }
    ///Checks if the value of the field is `DIV20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV20
    }
    ///Checks if the value of the field is `DIV21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV21
    }
    ///Checks if the value of the field is `DIV22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV22
    }
    ///Checks if the value of the field is `DIV23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV23
    }
    ///Checks if the value of the field is `DIV24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV24
    }
    ///Checks if the value of the field is `DIV25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV25
    }
    ///Checks if the value of the field is `DIV26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV26
    }
    ///Checks if the value of the field is `DIV27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV27
    }
    ///Checks if the value of the field is `DIV28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV28
    }
    ///Checks if the value of the field is `DIV29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV29
    }
    ///Checks if the value of the field is `DIV30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV30
    }
    ///Checks if the value of the field is `DIV31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV31
    }
    ///Checks if the value of the field is `DIV32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV32
    }
}
///Write proxy for field `PLLI2SDIVQ`
pub struct PLLI2SDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SDIVQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SDIVQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///PLLI2SDIVQ = /1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV1)
    }
    ///PLLI2SDIVQ = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV2)
    }
    ///PLLI2SDIVQ = /3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV3)
    }
    ///PLLI2SDIVQ = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV4)
    }
    ///PLLI2SDIVQ = /5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV5)
    }
    ///PLLI2SDIVQ = /6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV6)
    }
    ///PLLI2SDIVQ = /7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV7)
    }
    ///PLLI2SDIVQ = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV8)
    }
    ///PLLI2SDIVQ = /9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV9)
    }
    ///PLLI2SDIVQ = /10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV10)
    }
    ///PLLI2SDIVQ = /11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV11)
    }
    ///PLLI2SDIVQ = /12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV12)
    }
    ///PLLI2SDIVQ = /13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV13)
    }
    ///PLLI2SDIVQ = /14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV14)
    }
    ///PLLI2SDIVQ = /15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV15)
    }
    ///PLLI2SDIVQ = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV16)
    }
    ///PLLI2SDIVQ = /17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV17)
    }
    ///PLLI2SDIVQ = /18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV18)
    }
    ///PLLI2SDIVQ = /19
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV19)
    }
    ///PLLI2SDIVQ = /20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV20)
    }
    ///PLLI2SDIVQ = /21
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV21)
    }
    ///PLLI2SDIVQ = /22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV22)
    }
    ///PLLI2SDIVQ = /23
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV23)
    }
    ///PLLI2SDIVQ = /24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV24)
    }
    ///PLLI2SDIVQ = /25
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV25)
    }
    ///PLLI2SDIVQ = /26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV26)
    }
    ///PLLI2SDIVQ = /27
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV27)
    }
    ///PLLI2SDIVQ = /28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV28)
    }
    ///PLLI2SDIVQ = /29
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV29)
    }
    ///PLLI2SDIVQ = /30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV30)
    }
    ///PLLI2SDIVQ = /31
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV31)
    }
    ///PLLI2SDIVQ = /32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV32)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
///PLLSAI division factor for SAI1 clock
///
///Value on reset: 16
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAIDIVQ_A {
    ///0: PLLSAIDIVQ = /1
    DIV1 = 0,
    ///1: PLLSAIDIVQ = /2
    DIV2 = 1,
    ///2: PLLSAIDIVQ = /3
    DIV3 = 2,
    ///3: PLLSAIDIVQ = /4
    DIV4 = 3,
    ///4: PLLSAIDIVQ = /5
    DIV5 = 4,
    ///5: PLLSAIDIVQ = /6
    DIV6 = 5,
    ///6: PLLSAIDIVQ = /7
    DIV7 = 6,
    ///7: PLLSAIDIVQ = /8
    DIV8 = 7,
    ///8: PLLSAIDIVQ = /9
    DIV9 = 8,
    ///9: PLLSAIDIVQ = /10
    DIV10 = 9,
    ///10: PLLSAIDIVQ = /11
    DIV11 = 10,
    ///11: PLLSAIDIVQ = /12
    DIV12 = 11,
    ///12: PLLSAIDIVQ = /13
    DIV13 = 12,
    ///13: PLLSAIDIVQ = /14
    DIV14 = 13,
    ///14: PLLSAIDIVQ = /15
    DIV15 = 14,
    ///15: PLLSAIDIVQ = /16
    DIV16 = 15,
    ///16: PLLSAIDIVQ = /17
    DIV17 = 16,
    ///17: PLLSAIDIVQ = /18
    DIV18 = 17,
    ///18: PLLSAIDIVQ = /19
    DIV19 = 18,
    ///19: PLLSAIDIVQ = /20
    DIV20 = 19,
    ///20: PLLSAIDIVQ = /21
    DIV21 = 20,
    ///21: PLLSAIDIVQ = /22
    DIV22 = 21,
    ///22: PLLSAIDIVQ = /23
    DIV23 = 22,
    ///23: PLLSAIDIVQ = /24
    DIV24 = 23,
    ///24: PLLSAIDIVQ = /25
    DIV25 = 24,
    ///25: PLLSAIDIVQ = /26
    DIV26 = 25,
    ///26: PLLSAIDIVQ = /27
    DIV27 = 26,
    ///27: PLLSAIDIVQ = /28
    DIV28 = 27,
    ///28: PLLSAIDIVQ = /29
    DIV29 = 28,
    ///29: PLLSAIDIVQ = /30
    DIV30 = 29,
    ///30: PLLSAIDIVQ = /31
    DIV31 = 30,
    ///31: PLLSAIDIVQ = /32
    DIV32 = 31,
}
impl From<PLLSAIDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVQ_A) -> Self {
        variant as _
    }
}
///Reader of field `PLLSAIDIVQ`
pub type PLLSAIDIVQ_R = crate::R<u8, PLLSAIDIVQ_A>;
impl PLLSAIDIVQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVQ_A {
        match self.bits {
            0 => PLLSAIDIVQ_A::DIV1,
            1 => PLLSAIDIVQ_A::DIV2,
            2 => PLLSAIDIVQ_A::DIV3,
            3 => PLLSAIDIVQ_A::DIV4,
            4 => PLLSAIDIVQ_A::DIV5,
            5 => PLLSAIDIVQ_A::DIV6,
            6 => PLLSAIDIVQ_A::DIV7,
            7 => PLLSAIDIVQ_A::DIV8,
            8 => PLLSAIDIVQ_A::DIV9,
            9 => PLLSAIDIVQ_A::DIV10,
            10 => PLLSAIDIVQ_A::DIV11,
            11 => PLLSAIDIVQ_A::DIV12,
            12 => PLLSAIDIVQ_A::DIV13,
            13 => PLLSAIDIVQ_A::DIV14,
            14 => PLLSAIDIVQ_A::DIV15,
            15 => PLLSAIDIVQ_A::DIV16,
            16 => PLLSAIDIVQ_A::DIV17,
            17 => PLLSAIDIVQ_A::DIV18,
            18 => PLLSAIDIVQ_A::DIV19,
            19 => PLLSAIDIVQ_A::DIV20,
            20 => PLLSAIDIVQ_A::DIV21,
            21 => PLLSAIDIVQ_A::DIV22,
            22 => PLLSAIDIVQ_A::DIV23,
            23 => PLLSAIDIVQ_A::DIV24,
            24 => PLLSAIDIVQ_A::DIV25,
            25 => PLLSAIDIVQ_A::DIV26,
            26 => PLLSAIDIVQ_A::DIV27,
            27 => PLLSAIDIVQ_A::DIV28,
            28 => PLLSAIDIVQ_A::DIV29,
            29 => PLLSAIDIVQ_A::DIV30,
            30 => PLLSAIDIVQ_A::DIV31,
            31 => PLLSAIDIVQ_A::DIV32,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV2
    }
    ///Checks if the value of the field is `DIV3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV3
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV4
    }
    ///Checks if the value of the field is `DIV5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV5
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV6
    }
    ///Checks if the value of the field is `DIV7`
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV7
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV8
    }
    ///Checks if the value of the field is `DIV9`
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV9
    }
    ///Checks if the value of the field is `DIV10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV10
    }
    ///Checks if the value of the field is `DIV11`
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV11
    }
    ///Checks if the value of the field is `DIV12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV12
    }
    ///Checks if the value of the field is `DIV13`
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV13
    }
    ///Checks if the value of the field is `DIV14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV14
    }
    ///Checks if the value of the field is `DIV15`
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV15
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV16
    }
    ///Checks if the value of the field is `DIV17`
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV17
    }
    ///Checks if the value of the field is `DIV18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV18
    }
    ///Checks if the value of the field is `DIV19`
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV19
    }
    ///Checks if the value of the field is `DIV20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV20
    }
    ///Checks if the value of the field is `DIV21`
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV21
    }
    ///Checks if the value of the field is `DIV22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV22
    }
    ///Checks if the value of the field is `DIV23`
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV23
    }
    ///Checks if the value of the field is `DIV24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV24
    }
    ///Checks if the value of the field is `DIV25`
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV25
    }
    ///Checks if the value of the field is `DIV26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV26
    }
    ///Checks if the value of the field is `DIV27`
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV27
    }
    ///Checks if the value of the field is `DIV28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV28
    }
    ///Checks if the value of the field is `DIV29`
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV29
    }
    ///Checks if the value of the field is `DIV30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV30
    }
    ///Checks if the value of the field is `DIV31`
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV31
    }
    ///Checks if the value of the field is `DIV32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV32
    }
}
///Write proxy for field `PLLSAIDIVQ`
pub struct PLLSAIDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLSAIDIVQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///PLLSAIDIVQ = /1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV1)
    }
    ///PLLSAIDIVQ = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV2)
    }
    ///PLLSAIDIVQ = /3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV3)
    }
    ///PLLSAIDIVQ = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV4)
    }
    ///PLLSAIDIVQ = /5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV5)
    }
    ///PLLSAIDIVQ = /6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV6)
    }
    ///PLLSAIDIVQ = /7
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV7)
    }
    ///PLLSAIDIVQ = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV8)
    }
    ///PLLSAIDIVQ = /9
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV9)
    }
    ///PLLSAIDIVQ = /10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV10)
    }
    ///PLLSAIDIVQ = /11
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV11)
    }
    ///PLLSAIDIVQ = /12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV12)
    }
    ///PLLSAIDIVQ = /13
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV13)
    }
    ///PLLSAIDIVQ = /14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV14)
    }
    ///PLLSAIDIVQ = /15
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV15)
    }
    ///PLLSAIDIVQ = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV16)
    }
    ///PLLSAIDIVQ = /17
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV17)
    }
    ///PLLSAIDIVQ = /18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV18)
    }
    ///PLLSAIDIVQ = /19
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV19)
    }
    ///PLLSAIDIVQ = /20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV20)
    }
    ///PLLSAIDIVQ = /21
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV21)
    }
    ///PLLSAIDIVQ = /22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV22)
    }
    ///PLLSAIDIVQ = /23
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV23)
    }
    ///PLLSAIDIVQ = /24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV24)
    }
    ///PLLSAIDIVQ = /25
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV25)
    }
    ///PLLSAIDIVQ = /26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV26)
    }
    ///PLLSAIDIVQ = /27
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV27)
    }
    ///PLLSAIDIVQ = /28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV28)
    }
    ///PLLSAIDIVQ = /29
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV29)
    }
    ///PLLSAIDIVQ = /30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV30)
    }
    ///PLLSAIDIVQ = /31
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV31)
    }
    ///PLLSAIDIVQ = /32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV32)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
///division factor for LCD_CLK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAIDIVR_A {
    ///0: PLLSAIDIVR = /2
    DIV2 = 0,
    ///1: PLLSAIDIVR = /4
    DIV4 = 1,
    ///2: PLLSAIDIVR = /8
    DIV8 = 2,
    ///3: PLLSAIDIVR = /16
    DIV16 = 3,
}
impl From<PLLSAIDIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVR_A) -> Self {
        variant as _
    }
}
///Reader of field `PLLSAIDIVR`
pub type PLLSAIDIVR_R = crate::R<u8, PLLSAIDIVR_A>;
impl PLLSAIDIVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVR_A {
        match self.bits {
            0 => PLLSAIDIVR_A::DIV2,
            1 => PLLSAIDIVR_A::DIV4,
            2 => PLLSAIDIVR_A::DIV8,
            3 => PLLSAIDIVR_A::DIV16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV16
    }
}
///Write proxy for field `PLLSAIDIVR`
pub struct PLLSAIDIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLSAIDIVR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///PLLSAIDIVR = /2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV2)
    }
    ///PLLSAIDIVR = /4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV4)
    }
    ///PLLSAIDIVR = /8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV8)
    }
    ///PLLSAIDIVR = /16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///SAI1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    ///0: SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    PLLSAI = 0,
    ///1: SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    PLLI2S = 1,
    ///2: SAI1 clock frequency = Alternate function input frequency
    AFIF = 2,
    ///3: SAI1 clock frequency = HSI or HSE
    HSI_HSE = 3,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
///Reader of field `SAI1SEL`
pub type SAI1SEL_R = crate::R<u8, SAI1SEL_A>;
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAI1SEL_A {
        match self.bits {
            0 => SAI1SEL_A::PLLSAI,
            1 => SAI1SEL_A::PLLI2S,
            2 => SAI1SEL_A::AFIF,
            3 => SAI1SEL_A::HSI_HSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PLLSAI`
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1SEL_A::PLLSAI
    }
    ///Checks if the value of the field is `PLLI2S`
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1SEL_A::PLLI2S
    }
    ///Checks if the value of the field is `AFIF`
    #[inline(always)]
    pub fn is_afif(&self) -> bool {
        *self == SAI1SEL_A::AFIF
    }
    ///Checks if the value of the field is `HSI_HSE`
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == SAI1SEL_A::HSI_HSE
    }
}
///Write proxy for field `SAI1SEL`
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI1SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLLSAI)
    }
    ///SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLLI2S)
    }
    ///SAI1 clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn afif(self) -> &'a mut W {
        self.variant(SAI1SEL_A::AFIF)
    }
    ///SAI1 clock frequency = HSI or HSE
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(SAI1SEL_A::HSI_HSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
///SAI2 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI2SEL_A {
    ///0: SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    PLLSAI = 0,
    ///1: SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    PLLI2S = 1,
    ///2: SAI2 clock frequency = Alternate function input frequency
    AFIF = 2,
    ///3: SAI2 clock frequency = HSI or HSE
    HSI_HSE = 3,
}
impl From<SAI2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2SEL_A) -> Self {
        variant as _
    }
}
///Reader of field `SAI2SEL`
pub type SAI2SEL_R = crate::R<u8, SAI2SEL_A>;
impl SAI2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAI2SEL_A {
        match self.bits {
            0 => SAI2SEL_A::PLLSAI,
            1 => SAI2SEL_A::PLLI2S,
            2 => SAI2SEL_A::AFIF,
            3 => SAI2SEL_A::HSI_HSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PLLSAI`
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI2SEL_A::PLLSAI
    }
    ///Checks if the value of the field is `PLLI2S`
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI2SEL_A::PLLI2S
    }
    ///Checks if the value of the field is `AFIF`
    #[inline(always)]
    pub fn is_afif(&self) -> bool {
        *self == SAI2SEL_A::AFIF
    }
    ///Checks if the value of the field is `HSI_HSE`
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == SAI2SEL_A::HSI_HSE
    }
}
///Write proxy for field `SAI2SEL`
pub struct SAI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI2SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI2SEL_A::PLLSAI)
    }
    ///SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI2SEL_A::PLLI2S)
    }
    ///SAI2 clock frequency = Alternate function input frequency
    #[inline(always)]
    pub fn afif(self) -> &'a mut W {
        self.variant(SAI2SEL_A::AFIF)
    }
    ///SAI2 clock frequency = HSI or HSE
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(SAI2SEL_A::HSI_HSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
///Timers clocks prescalers selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPRE_A {
    ///0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    MUL2 = 0,
    ///1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    MUL4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TIMPRE`
pub type TIMPRE_R = crate::R<bool, TIMPRE_A>;
impl TIMPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::MUL2,
            true => TIMPRE_A::MUL4,
        }
    }
    ///Checks if the value of the field is `MUL2`
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == TIMPRE_A::MUL2
    }
    ///Checks if the value of the field is `MUL4`
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == TIMPRE_A::MUL4
    }
}
///Write proxy for field `TIMPRE`
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIMPRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL2)
    }
    ///If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL4)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///DFSDM1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDM1SEL_A {
    ///0: APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source
    APB2 = 0,
    ///1: System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source
    SYSCLK = 1,
}
impl From<DFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DFSDM1SEL`
pub type DFSDM1SEL_R = crate::R<bool, DFSDM1SEL_A>;
impl DFSDM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1SEL_A {
        match self.bits {
            false => DFSDM1SEL_A::APB2,
            true => DFSDM1SEL_A::SYSCLK,
        }
    }
    ///Checks if the value of the field is `APB2`
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == DFSDM1SEL_A::APB2
    }
    ///Checks if the value of the field is `SYSCLK`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == DFSDM1SEL_A::SYSCLK
    }
}
///Write proxy for field `DFSDM1SEL`
pub struct DFSDM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DFSDM1SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::APB2)
    }
    ///System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::SYSCLK)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///DFSDM1 AUDIO clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADFSDM1SEL_A {
    ///0: SAI1 clock selected as DFSDM1 Audio clock source
    SAI1 = 0,
    ///1: SAI2 clock selected as DFSDM1 Audio clock source
    SAI2 = 1,
}
impl From<ADFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADFSDM1SEL`
pub type ADFSDM1SEL_R = crate::R<bool, ADFSDM1SEL_A>;
impl ADFSDM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADFSDM1SEL_A {
        match self.bits {
            false => ADFSDM1SEL_A::SAI1,
            true => ADFSDM1SEL_A::SAI2,
        }
    }
    ///Checks if the value of the field is `SAI1`
    #[inline(always)]
    pub fn is_sai1(&self) -> bool {
        *self == ADFSDM1SEL_A::SAI1
    }
    ///Checks if the value of the field is `SAI2`
    #[inline(always)]
    pub fn is_sai2(&self) -> bool {
        *self == ADFSDM1SEL_A::SAI2
    }
}
///Write proxy for field `ADFSDM1SEL`
pub struct ADFSDM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFSDM1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADFSDM1SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///SAI1 clock selected as DFSDM1 Audio clock source
    #[inline(always)]
    pub fn sai1(self) -> &'a mut W {
        self.variant(ADFSDM1SEL_A::SAI1)
    }
    ///SAI2 clock selected as DFSDM1 Audio clock source
    #[inline(always)]
    pub fn sai2(self) -> &'a mut W {
        self.variant(ADFSDM1SEL_A::SAI2)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    ///Bits 0:4 - PLLI2S division factor for SAI1 clock
    #[inline(always)]
    pub fn plli2sdivq(&self) -> PLLI2SDIVQ_R {
        PLLI2SDIVQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - PLLSAI division factor for SAI1 clock
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:17 - division factor for LCD_CLK
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PLLSAIDIVR_R {
        PLLSAIDIVR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 20:21 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 22:23 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - DFSDM1 clock source selection
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - DFSDM1 AUDIO clock source selection
    #[inline(always)]
    pub fn adfsdm1sel(&self) -> ADFSDM1SEL_R {
        ADFSDM1SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:4 - PLLI2S division factor for SAI1 clock
    #[inline(always)]
    pub fn plli2sdivq(&mut self) -> PLLI2SDIVQ_W {
        PLLI2SDIVQ_W { w: self }
    }
    ///Bits 8:12 - PLLSAI division factor for SAI1 clock
    #[inline(always)]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W {
        PLLSAIDIVQ_W { w: self }
    }
    ///Bits 16:17 - division factor for LCD_CLK
    #[inline(always)]
    pub fn pllsaidivr(&mut self) -> PLLSAIDIVR_W {
        PLLSAIDIVR_W { w: self }
    }
    ///Bits 20:21 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    ///Bits 22:23 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W {
        SAI2SEL_W { w: self }
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    ///Bit 25 - DFSDM1 clock source selection
    #[inline(always)]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W {
        DFSDM1SEL_W { w: self }
    }
    ///Bit 26 - DFSDM1 AUDIO clock source selection
    #[inline(always)]
    pub fn adfsdm1sel(&mut self) -> ADFSDM1SEL_W {
        ADFSDM1SEL_W { w: self }
    }
}
