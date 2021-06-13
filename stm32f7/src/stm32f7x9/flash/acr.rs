///Reader of register ACR
pub type R = crate::R<u32, super::ACR>;
///Writer for register ACR
pub type W = crate::W<u32, super::ACR>;
///Register ACR `reset()`'s with value 0
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Latency
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATENCY_A {
    ///0: 0 wait states
    WS0 = 0,
    ///1: 1 wait states
    WS1 = 1,
    ///2: 2 wait states
    WS2 = 2,
    ///3: 3 wait states
    WS3 = 3,
    ///4: 4 wait states
    WS4 = 4,
    ///5: 5 wait states
    WS5 = 5,
    ///6: 6 wait states
    WS6 = 6,
    ///7: 7 wait states
    WS7 = 7,
    ///8: 8 wait states
    WS8 = 8,
    ///9: 9 wait states
    WS9 = 9,
    ///10: 10 wait states
    WS10 = 10,
    ///11: 11 wait states
    WS11 = 11,
    ///12: 12 wait states
    WS12 = 12,
    ///13: 13 wait states
    WS13 = 13,
    ///14: 14 wait states
    WS14 = 14,
    ///15: 15 wait states
    WS15 = 15,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
///Reader of field `LATENCY`
pub type LATENCY_R = crate::R<u8, LATENCY_A>;
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            0 => LATENCY_A::WS0,
            1 => LATENCY_A::WS1,
            2 => LATENCY_A::WS2,
            3 => LATENCY_A::WS3,
            4 => LATENCY_A::WS4,
            5 => LATENCY_A::WS5,
            6 => LATENCY_A::WS6,
            7 => LATENCY_A::WS7,
            8 => LATENCY_A::WS8,
            9 => LATENCY_A::WS9,
            10 => LATENCY_A::WS10,
            11 => LATENCY_A::WS11,
            12 => LATENCY_A::WS12,
            13 => LATENCY_A::WS13,
            14 => LATENCY_A::WS14,
            15 => LATENCY_A::WS15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `WS0`
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::WS0
    }
    ///Checks if the value of the field is `WS1`
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::WS1
    }
    ///Checks if the value of the field is `WS2`
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::WS2
    }
    ///Checks if the value of the field is `WS3`
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY_A::WS3
    }
    ///Checks if the value of the field is `WS4`
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY_A::WS4
    }
    ///Checks if the value of the field is `WS5`
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY_A::WS5
    }
    ///Checks if the value of the field is `WS6`
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY_A::WS6
    }
    ///Checks if the value of the field is `WS7`
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY_A::WS7
    }
    ///Checks if the value of the field is `WS8`
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY_A::WS8
    }
    ///Checks if the value of the field is `WS9`
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY_A::WS9
    }
    ///Checks if the value of the field is `WS10`
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY_A::WS10
    }
    ///Checks if the value of the field is `WS11`
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY_A::WS11
    }
    ///Checks if the value of the field is `WS12`
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY_A::WS12
    }
    ///Checks if the value of the field is `WS13`
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY_A::WS13
    }
    ///Checks if the value of the field is `WS14`
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY_A::WS14
    }
    ///Checks if the value of the field is `WS15`
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY_A::WS15
    }
}
///Write proxy for field `LATENCY`
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LATENCY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///0 wait states
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::WS0)
    }
    ///1 wait states
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::WS1)
    }
    ///2 wait states
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::WS2)
    }
    ///3 wait states
    #[inline(always)]
    pub fn ws3(self) -> &'a mut W {
        self.variant(LATENCY_A::WS3)
    }
    ///4 wait states
    #[inline(always)]
    pub fn ws4(self) -> &'a mut W {
        self.variant(LATENCY_A::WS4)
    }
    ///5 wait states
    #[inline(always)]
    pub fn ws5(self) -> &'a mut W {
        self.variant(LATENCY_A::WS5)
    }
    ///6 wait states
    #[inline(always)]
    pub fn ws6(self) -> &'a mut W {
        self.variant(LATENCY_A::WS6)
    }
    ///7 wait states
    #[inline(always)]
    pub fn ws7(self) -> &'a mut W {
        self.variant(LATENCY_A::WS7)
    }
    ///8 wait states
    #[inline(always)]
    pub fn ws8(self) -> &'a mut W {
        self.variant(LATENCY_A::WS8)
    }
    ///9 wait states
    #[inline(always)]
    pub fn ws9(self) -> &'a mut W {
        self.variant(LATENCY_A::WS9)
    }
    ///10 wait states
    #[inline(always)]
    pub fn ws10(self) -> &'a mut W {
        self.variant(LATENCY_A::WS10)
    }
    ///11 wait states
    #[inline(always)]
    pub fn ws11(self) -> &'a mut W {
        self.variant(LATENCY_A::WS11)
    }
    ///12 wait states
    #[inline(always)]
    pub fn ws12(self) -> &'a mut W {
        self.variant(LATENCY_A::WS12)
    }
    ///13 wait states
    #[inline(always)]
    pub fn ws13(self) -> &'a mut W {
        self.variant(LATENCY_A::WS13)
    }
    ///14 wait states
    #[inline(always)]
    pub fn ws14(self) -> &'a mut W {
        self.variant(LATENCY_A::WS14)
    }
    ///15 wait states
    #[inline(always)]
    pub fn ws15(self) -> &'a mut W {
        self.variant(LATENCY_A::WS15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
///Prefetch enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTEN_A {
    ///0: Prefetch is disabled
    DISABLED = 0,
    ///1: Prefetch is enabled
    ENABLED = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PRFTEN`
pub type PRFTEN_R = crate::R<bool, PRFTEN_A>;
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::DISABLED,
            true => PRFTEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN_A::ENABLED
    }
}
///Write proxy for field `PRFTEN`
pub struct PRFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PRFTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Prefetch is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::DISABLED)
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///ART Accelerator Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARTEN_A {
    ///0: ART Accelerator is disabled
    DISABLED = 0,
    ///1: ART Accelerator is enabled
    ENABLED = 1,
}
impl From<ARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ARTEN`
pub type ARTEN_R = crate::R<bool, ARTEN_A>;
impl ARTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARTEN_A {
        match self.bits {
            false => ARTEN_A::DISABLED,
            true => ARTEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARTEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARTEN_A::ENABLED
    }
}
///Write proxy for field `ARTEN`
pub struct ARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///ART Accelerator is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARTEN_A::DISABLED)
    }
    ///ART Accelerator is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///ART Accelerator reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARTRST_A {
    ///0: Accelerator is not reset
    NOTRESET = 0,
    ///1: Accelerator is reset
    RESET = 1,
}
impl From<ARTRST_A> for bool {
    #[inline(always)]
    fn from(variant: ARTRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ARTRST`
pub type ARTRST_R = crate::R<bool, ARTRST_A>;
impl ARTRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARTRST_A {
        match self.bits {
            false => ARTRST_A::NOTRESET,
            true => ARTRST_A::RESET,
        }
    }
    ///Checks if the value of the field is `NOTRESET`
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ARTRST_A::NOTRESET
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ARTRST_A::RESET
    }
}
///Write proxy for field `ARTRST`
pub struct ARTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARTRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Accelerator is not reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ARTRST_A::NOTRESET)
    }
    ///Accelerator is reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ARTRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - ART Accelerator Enable
    #[inline(always)]
    pub fn arten(&self) -> ARTEN_R {
        ARTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 11 - ART Accelerator reset
    #[inline(always)]
    pub fn artrst(&self) -> ARTRST_R {
        ARTRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W {
        PRFTEN_W { w: self }
    }
    ///Bit 9 - ART Accelerator Enable
    #[inline(always)]
    pub fn arten(&mut self) -> ARTEN_W {
        ARTEN_W { w: self }
    }
    ///Bit 11 - ART Accelerator reset
    #[inline(always)]
    pub fn artrst(&mut self) -> ARTRST_W {
        ARTRST_W { w: self }
    }
}
