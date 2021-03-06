///Reader of register ITCMCR
pub type R = crate::R<u32, super::ITCMCR>;
///Writer for register ITCMCR
pub type W = crate::W<u32, super::ITCMCR>;
///Register ITCMCR `reset()`'s with value 0
impl crate::ResetValue for super::ITCMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EN`
pub type EN_R = crate::R<bool, bool>;
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
///Reader of field `RMW`
pub type RMW_R = crate::R<bool, bool>;
///Write proxy for field `RMW`
pub struct RMW_W<'a> {
    w: &'a mut W,
}
impl<'a> RMW_W<'a> {
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
///Reader of field `RETEN`
pub type RETEN_R = crate::R<bool, bool>;
///Write proxy for field `RETEN`
pub struct RETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETEN_W<'a> {
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
///Reader of field `SZ`
pub type SZ_R = crate::R<u8, u8>;
///Write proxy for field `SZ`
pub struct SZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - RMW
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - RETEN
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 3:6 - SZ
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    ///Bit 1 - RMW
    #[inline(always)]
    pub fn rmw(&mut self) -> RMW_W {
        RMW_W { w: self }
    }
    ///Bit 2 - RETEN
    #[inline(always)]
    pub fn reten(&mut self) -> RETEN_W {
        RETEN_W { w: self }
    }
    ///Bits 3:6 - SZ
    #[inline(always)]
    pub fn sz(&mut self) -> SZ_W {
        SZ_W { w: self }
    }
}
