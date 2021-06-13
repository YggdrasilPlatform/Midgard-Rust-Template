///Reader of register IER
pub type R = crate::R<u32, super::IER>;
///Writer for register IER
pub type W = crate::W<u32, super::IER>;
///Register IER `reset()`'s with value 0
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Register Reload interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRIE_A {
    ///0: Register reload interrupt disabled
    DISABLED = 0,
    ///1: Register reload interrupt enabled
    ENABLED = 1,
}
impl From<RRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RRIE`
pub type RRIE_R = crate::R<bool, RRIE_A>;
impl RRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RRIE_A {
        match self.bits {
            false => RRIE_A::DISABLED,
            true => RRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRIE_A::ENABLED
    }
}
///Write proxy for field `RRIE`
pub struct RRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Register reload interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RRIE_A::DISABLED)
    }
    ///Register reload interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RRIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Transfer Error Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERRIE_A {
    ///0: Transfer error interrupt disabled
    DISABLED = 0,
    ///1: Transfer error interrupt enabled
    ENABLED = 1,
}
impl From<TERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TERRIE`
pub type TERRIE_R = crate::R<bool, TERRIE_A>;
impl TERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TERRIE_A {
        match self.bits {
            false => TERRIE_A::DISABLED,
            true => TERRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TERRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TERRIE_A::ENABLED
    }
}
///Write proxy for field `TERRIE`
pub struct TERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Transfer error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TERRIE_A::DISABLED)
    }
    ///Transfer error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TERRIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///FIFO Underrun Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUIE_A {
    ///0: FIFO underrun interrupt disabled
    DISABLED = 0,
    ///1: FIFO underrun interrupt enabled
    ENABLED = 1,
}
impl From<FUIE_A> for bool {
    #[inline(always)]
    fn from(variant: FUIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FUIE`
pub type FUIE_R = crate::R<bool, FUIE_A>;
impl FUIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FUIE_A {
        match self.bits {
            false => FUIE_A::DISABLED,
            true => FUIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FUIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FUIE_A::ENABLED
    }
}
///Write proxy for field `FUIE`
pub struct FUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FUIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///FIFO underrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FUIE_A::DISABLED)
    }
    ///FIFO underrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FUIE_A::ENABLED)
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
///Line Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIE_A {
    ///0: Line interrupt disabled
    DISABLED = 0,
    ///1: Line interrupt enabled
    ENABLED = 1,
}
impl From<LIE_A> for bool {
    #[inline(always)]
    fn from(variant: LIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LIE`
pub type LIE_R = crate::R<bool, LIE_A>;
impl LIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LIE_A {
        match self.bits {
            false => LIE_A::DISABLED,
            true => LIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LIE_A::ENABLED
    }
}
///Write proxy for field `LIE`
pub struct LIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Line interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LIE_A::DISABLED)
    }
    ///Line interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LIE_A::ENABLED)
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
    ///Bit 3 - Register Reload interrupt enable
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Transfer Error Interrupt Enable
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt Enable
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Line Interrupt Enable
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 3 - Register Reload interrupt enable
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W {
        RRIE_W { w: self }
    }
    ///Bit 2 - Transfer Error Interrupt Enable
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W {
        TERRIE_W { w: self }
    }
    ///Bit 1 - FIFO Underrun Interrupt Enable
    #[inline(always)]
    pub fn fuie(&mut self) -> FUIE_W {
        FUIE_W { w: self }
    }
    ///Bit 0 - Line Interrupt Enable
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W {
        LIE_W { w: self }
    }
}
