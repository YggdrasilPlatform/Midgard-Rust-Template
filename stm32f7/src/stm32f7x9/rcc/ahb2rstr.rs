///Reader of register AHB2RSTR
pub type R = crate::R<u32, super::AHB2RSTR>;
///Writer for register AHB2RSTR
pub type W = crate::W<u32, super::AHB2RSTR>;
///Register AHB2RSTR `reset()`'s with value 0
impl crate::ResetValue for super::AHB2RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///USB OTG FS module reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSRST_A {
    ///1: Reset the selected module
    RESET = 1,
}
impl From<OTGFSRST_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OTGFSRST`
pub type OTGFSRST_R = crate::R<bool, OTGFSRST_A>;
impl OTGFSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, OTGFSRST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(OTGFSRST_A::RESET),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRST_A::RESET
    }
}
///Write proxy for field `OTGFSRST`
pub struct OTGFSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OTGFSRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::RESET)
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
///Random number generator module reset
pub type RNGRST_A = OTGFSRST_A;
///Reader of field `RNGRST`
pub type RNGRST_R = crate::R<bool, OTGFSRST_A>;
///Write proxy for field `RNGRST`
pub struct RNGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RNGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Hash module reset
pub type HSAHRST_A = OTGFSRST_A;
///Reader of field `HSAHRST`
pub type HSAHRST_R = crate::R<bool, OTGFSRST_A>;
///Write proxy for field `HSAHRST`
pub struct HSAHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSAHRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSAHRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Cryptographic module reset
pub type CRYPRST_A = OTGFSRST_A;
///Reader of field `CRYPRST`
pub type CRYPRST_R = crate::R<bool, OTGFSRST_A>;
///Write proxy for field `CRYPRST`
pub struct CRYPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRYPRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::RESET)
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
///Camera interface reset
pub type DCMIRST_A = OTGFSRST_A;
///Reader of field `DCMIRST`
pub type DCMIRST_R = crate::R<bool, OTGFSRST_A>;
///Write proxy for field `DCMIRST`
pub struct DCMIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DCMIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::RESET)
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
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Random number generator module reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Hash module reset
    #[inline(always)]
    pub fn hsahrst(&self) -> HSAHRST_R {
        HSAHRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Cryptographic module reset
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 0 - Camera interface reset
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W {
        OTGFSRST_W { w: self }
    }
    ///Bit 6 - Random number generator module reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W {
        RNGRST_W { w: self }
    }
    ///Bit 5 - Hash module reset
    #[inline(always)]
    pub fn hsahrst(&mut self) -> HSAHRST_W {
        HSAHRST_W { w: self }
    }
    ///Bit 4 - Cryptographic module reset
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W {
        CRYPRST_W { w: self }
    }
    ///Bit 0 - Camera interface reset
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W {
        DCMIRST_W { w: self }
    }
}
