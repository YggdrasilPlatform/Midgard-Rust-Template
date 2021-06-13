///Reader of register OTG_HS_HCSPLT12
pub type R = crate::R<u32, super::OTG_HS_HCSPLT12>;
///Writer for register OTG_HS_HCSPLT12
pub type W = crate::W<u32, super::OTG_HS_HCSPLT12>;
///Register OTG_HS_HCSPLT12 `reset()`'s with value 0
impl crate::ResetValue for super::OTG_HS_HCSPLT12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PRTADDR`
pub type PRTADDR_R = crate::R<u8, u8>;
///Write proxy for field `PRTADDR`
pub struct PRTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTADDR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Reader of field `HUBADDR`
pub type HUBADDR_R = crate::R<u8, u8>;
///Write proxy for field `HUBADDR`
pub struct HUBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HUBADDR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | (((value as u32) & 0x7f) << 7);
        self.w
    }
}
///Reader of field `XACTPOS`
pub type XACTPOS_R = crate::R<u8, u8>;
///Write proxy for field `XACTPOS`
pub struct XACTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> XACTPOS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Reader of field `COMPLSPLT`
pub type COMPLSPLT_R = crate::R<bool, bool>;
///Write proxy for field `COMPLSPLT`
pub struct COMPLSPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLSPLT_W<'a> {
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
///Reader of field `SPLITEN`
pub type SPLITEN_R = crate::R<bool, bool>;
///Write proxy for field `SPLITEN`
pub struct SPLITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLITEN_W<'a> {
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
    ///Bits 0:6 - Port address
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - Hub address
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    ///Bits 14:15 - XACTPOS
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bit 16 - Do complete split
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 31 - Split enable
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:6 - Port address
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W {
        PRTADDR_W { w: self }
    }
    ///Bits 7:13 - Hub address
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W {
        HUBADDR_W { w: self }
    }
    ///Bits 14:15 - XACTPOS
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W {
        XACTPOS_W { w: self }
    }
    ///Bit 16 - Do complete split
    #[inline(always)]
    pub fn complsplt(&mut self) -> COMPLSPLT_W {
        COMPLSPLT_W { w: self }
    }
    ///Bit 31 - Split enable
    #[inline(always)]
    pub fn spliten(&mut self) -> SPLITEN_W {
        SPLITEN_W { w: self }
    }
}
