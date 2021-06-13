///Reader of register MMCTIMR
pub type R = crate::R<u32, super::MMCTIMR>;
///Writer for register MMCTIMR
pub type W = crate::W<u32, super::MMCTIMR>;
///Register MMCTIMR `reset()`'s with value 0
impl crate::ResetValue for super::MMCTIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Transmitted good frames single collision mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFSCM_A {
    ///0: Transmitted-good-single-collision half-full interrupt enabled
    UNMASKED = 0,
    ///1: Transmitted-good-single-collision half-full interrupt disabled
    MASKED = 1,
}
impl From<TGFSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFSCM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TGFSCM`
pub type TGFSCM_R = crate::R<bool, TGFSCM_A>;
impl TGFSCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TGFSCM_A {
        match self.bits {
            false => TGFSCM_A::UNMASKED,
            true => TGFSCM_A::MASKED,
        }
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFSCM_A::UNMASKED
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFSCM_A::MASKED
    }
}
///Write proxy for field `TGFSCM`
pub struct TGFSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFSCM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TGFSCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Transmitted-good-single-collision half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFSCM_A::UNMASKED)
    }
    ///Transmitted-good-single-collision half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFSCM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Transmitted good frames more than single collision mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFMSCM_A {
    ///0: Transmitted-good-multiple-collision half-full interrupt enabled
    UNMASKED = 0,
    ///1: Transmitted-good-multiple-collision half-full interrupt disabled
    MASKED = 1,
}
impl From<TGFMSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFMSCM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TGFMSCM`
pub type TGFMSCM_R = crate::R<bool, TGFMSCM_A>;
impl TGFMSCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TGFMSCM_A {
        match self.bits {
            false => TGFMSCM_A::UNMASKED,
            true => TGFMSCM_A::MASKED,
        }
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFMSCM_A::UNMASKED
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFMSCM_A::MASKED
    }
}
///Write proxy for field `TGFMSCM`
pub struct TGFMSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFMSCM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TGFMSCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Transmitted-good-multiple-collision half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::UNMASKED)
    }
    ///Transmitted-good-multiple-collision half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Transmitted good frames mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFM_A {
    ///0: Transmitted-good counter half-full interrupt enabled
    UNMASKED = 0,
    ///1: Transmitted-good counter half-full interrupt disabled
    MASKED = 1,
}
impl From<TGFM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TGFM`
pub type TGFM_R = crate::R<bool, TGFM_A>;
impl TGFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TGFM_A {
        match self.bits {
            false => TGFM_A::UNMASKED,
            true => TGFM_A::MASKED,
        }
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFM_A::UNMASKED
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFM_A::MASKED
    }
}
///Write proxy for field `TGFM`
pub struct TGFM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TGFM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Transmitted-good counter half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFM_A::UNMASKED)
    }
    ///Transmitted-good counter half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    ///Bit 14 - Transmitted good frames single collision mask
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Transmitted good frames more than single collision mask
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Transmitted good frames mask
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 14 - Transmitted good frames single collision mask
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TGFSCM_W {
        TGFSCM_W { w: self }
    }
    ///Bit 15 - Transmitted good frames more than single collision mask
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W {
        TGFMSCM_W { w: self }
    }
    ///Bit 16 - Transmitted good frames mask
    #[inline(always)]
    pub fn tgfm(&mut self) -> TGFM_W {
        TGFM_W { w: self }
    }
}
