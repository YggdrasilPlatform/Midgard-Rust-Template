///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Color Look-Up Table Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLUTEN_A {
    ///0: Color look-up table disabled
    DISABLED = 0,
    ///1: Color look-up table enabled
    ENABLED = 1,
}
impl From<CLUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLUTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CLUTEN`
pub type CLUTEN_R = crate::R<bool, CLUTEN_A>;
impl CLUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLUTEN_A {
        match self.bits {
            false => CLUTEN_A::DISABLED,
            true => CLUTEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLUTEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLUTEN_A::ENABLED
    }
}
///Write proxy for field `CLUTEN`
pub struct CLUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLUTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CLUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Color look-up table disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLUTEN_A::DISABLED)
    }
    ///Color look-up table enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLUTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Color Keying Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLKEN_A {
    ///0: Color keying disabled
    DISABLED = 0,
    ///1: Color keying enabled
    ENABLED = 1,
}
impl From<COLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: COLKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `COLKEN`
pub type COLKEN_R = crate::R<bool, COLKEN_A>;
impl COLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COLKEN_A {
        match self.bits {
            false => COLKEN_A::DISABLED,
            true => COLKEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COLKEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COLKEN_A::ENABLED
    }
}
///Write proxy for field `COLKEN`
pub struct COLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COLKEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Color keying disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COLKEN_A::DISABLED)
    }
    ///Color keying enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COLKEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Layer Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEN_A {
    ///0: Layer disabled
    DISABLED = 0,
    ///1: Layer enabled
    ENABLED = 1,
}
impl From<LEN_A> for bool {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LEN`
pub type LEN_R = crate::R<bool, LEN_A>;
impl LEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LEN_A {
        match self.bits {
            false => LEN_A::DISABLED,
            true => LEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LEN_A::ENABLED
    }
}
///Write proxy for field `LEN`
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Layer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LEN_A::DISABLED)
    }
    ///Layer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W {
        CLUTEN_W { w: self }
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W {
        COLKEN_W { w: self }
    }
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
}
