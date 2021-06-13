///Reader of register AHB3RSTR
pub type R = crate::R<u32, super::AHB3RSTR>;
///Writer for register AHB3RSTR
pub type W = crate::W<u32, super::AHB3RSTR>;
///Register AHB3RSTR `reset()`'s with value 0
impl crate::ResetValue for super::AHB3RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Flexible memory controller module reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCRST_A {
    ///1: Reset the selected module
    RESET = 1,
}
impl From<FMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FMCRST`
pub type FMCRST_R = crate::R<bool, FMCRST_A>;
impl FMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FMCRST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FMCRST_A::RESET),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FMCRST_A::RESET
    }
}
///Write proxy for field `FMCRST`
pub struct FMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FMCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMCRST_A::RESET)
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
///Quad SPI memory controller reset
pub type QSPIRST_A = FMCRST_A;
///Reader of field `QSPIRST`
pub type QSPIRST_R = crate::R<bool, FMCRST_A>;
///Write proxy for field `QSPIRST`
pub struct QSPIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: QSPIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMCRST_A::RESET)
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
impl R {
    ///Bit 0 - Flexible memory controller module reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Quad SPI memory controller reset
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller module reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
    ///Bit 1 - Quad SPI memory controller reset
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W {
        QSPIRST_W { w: self }
    }
}
