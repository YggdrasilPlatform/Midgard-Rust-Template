///Reader of register MACA1HR
pub type R = crate::R<u32, super::MACA1HR>;
///Writer for register MACA1HR
pub type W = crate::W<u32, super::MACA1HR>;
///Register MACA1HR `reset()`'s with value 0xffff
impl crate::ResetValue for super::MACA1HR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
///Reader of field `MACA1H`
pub type MACA1H_R = crate::R<u16, u16>;
///Write proxy for field `MACA1H`
pub struct MACA1H_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA1H_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `MBC`
pub type MBC_R = crate::R<u8, u8>;
///Write proxy for field `MBC`
pub struct MBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MBC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
///SA
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_A {
    ///0: This address is used for comparison with DA fields of the received frame
    DESTINATION = 0,
    ///1: This address is used for comparison with SA fields of received frames
    SOURCE = 1,
}
impl From<SA_A> for bool {
    #[inline(always)]
    fn from(variant: SA_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SA`
pub type SA_R = crate::R<bool, SA_A>;
impl SA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SA_A {
        match self.bits {
            false => SA_A::DESTINATION,
            true => SA_A::SOURCE,
        }
    }
    ///Checks if the value of the field is `DESTINATION`
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        *self == SA_A::DESTINATION
    }
    ///Checks if the value of the field is `SOURCE`
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == SA_A::SOURCE
    }
}
///Write proxy for field `SA`
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This address is used for comparison with DA fields of the received frame
    #[inline(always)]
    pub fn destination(self) -> &'a mut W {
        self.variant(SA_A::DESTINATION)
    }
    ///This address is used for comparison with SA fields of received frames
    #[inline(always)]
    pub fn source(self) -> &'a mut W {
        self.variant(SA_A::SOURCE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///AE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AE_A {
    ///0: Address filters ignore this address
    DISABLED = 0,
    ///1: Address filters use this address
    ENABLED = 1,
}
impl From<AE_A> for bool {
    #[inline(always)]
    fn from(variant: AE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AE`
pub type AE_R = crate::R<bool, AE_A>;
impl AE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AE_A {
        match self.bits {
            false => AE_A::DISABLED,
            true => AE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AE_A::ENABLED
    }
}
///Write proxy for field `AE`
pub struct AE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Address filters ignore this address
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AE_A::DISABLED)
    }
    ///Address filters use this address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:15 - MACA1H
    #[inline(always)]
    pub fn maca1h(&self) -> MACA1H_R {
        MACA1H_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - MACA1H
    #[inline(always)]
    pub fn maca1h(&mut self) -> MACA1H_W {
        MACA1H_W { w: self }
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W {
        MBC_W { w: self }
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
}
