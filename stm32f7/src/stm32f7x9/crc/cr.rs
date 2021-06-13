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
///RESET bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    ///1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    RESET = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `RESET`
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESET_AW::RESET)
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
///Reverse output data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_OUT_AW {
    ///0: Bit order not affected
    NORMAL = 0,
    ///1: Bit reversed output
    REVERSED = 1,
}
impl From<REV_OUT_AW> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `REV_OUT`
pub struct REV_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_OUT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REV_OUT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_OUT_AW::NORMAL)
    }
    ///Bit reversed output
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(REV_OUT_AW::REVERSED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Reverse input data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REV_IN_AW {
    ///0: Bit order not affected
    NORMAL = 0,
    ///1: Bit reversal done by byte
    BYTE = 1,
    ///2: Bit reversal done by half-word
    HALFWORD = 2,
    ///3: Bit reversal done by word
    WORD = 3,
}
impl From<REV_IN_AW> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_AW) -> Self {
        variant as _
    }
}
///Write proxy for field `REV_IN`
pub struct REV_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_IN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REV_IN_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_IN_AW::NORMAL)
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(REV_IN_AW::BYTE)
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(REV_IN_AW::HALFWORD)
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(REV_IN_AW::WORD)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
///Polynomial size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLYSIZE_AW {
    ///0: 32-bit polynomial
    POLYSIZE32 = 0,
    ///1: 16-bit polynomial
    POLYSIZE16 = 1,
    ///2: 8-bit polynomial
    POLYSIZE8 = 2,
    ///3: 7-bit polynomial
    POLYSIZE7 = 3,
}
impl From<POLYSIZE_AW> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_AW) -> Self {
        variant as _
    }
}
///Write proxy for field `POLYSIZE`
pub struct POLYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSIZE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: POLYSIZE_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///32-bit polynomial
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::POLYSIZE32)
    }
    ///16-bit polynomial
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::POLYSIZE16)
    }
    ///8-bit polynomial
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::POLYSIZE8)
    }
    ///7-bit polynomial
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::POLYSIZE7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl W {
    ///Bit 0 - RESET bit
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    ///Bit 7 - Reverse output data
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W {
        REV_OUT_W { w: self }
    }
    ///Bits 5:6 - Reverse input data
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W {
        REV_IN_W { w: self }
    }
    ///Bits 3:4 - Polynomial size
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W {
        POLYSIZE_W { w: self }
    }
}
