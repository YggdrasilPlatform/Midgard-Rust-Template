///Reader of register OTG_HS_DTHRCTL
pub type R = crate::R<u32, super::OTG_HS_DTHRCTL>;
///Writer for register OTG_HS_DTHRCTL
pub type W = crate::W<u32, super::OTG_HS_DTHRCTL>;
///Register OTG_HS_DTHRCTL `reset()`'s with value 0
impl crate::ResetValue for super::OTG_HS_DTHRCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `NONISOTHREN`
pub type NONISOTHREN_R = crate::R<bool, bool>;
///Write proxy for field `NONISOTHREN`
pub struct NONISOTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> NONISOTHREN_W<'a> {
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
///Reader of field `ISOTHREN`
pub type ISOTHREN_R = crate::R<bool, bool>;
///Write proxy for field `ISOTHREN`
pub struct ISOTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOTHREN_W<'a> {
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
///Reader of field `TXTHRLEN`
pub type TXTHRLEN_R = crate::R<u16, u16>;
///Write proxy for field `TXTHRLEN`
pub struct TXTHRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTHRLEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | (((value as u32) & 0x01ff) << 2);
        self.w
    }
}
///Reader of field `RXTHREN`
pub type RXTHREN_R = crate::R<bool, bool>;
///Write proxy for field `RXTHREN`
pub struct RXTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHREN_W<'a> {
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
///Reader of field `RXTHRLEN`
pub type RXTHRLEN_R = crate::R<u16, u16>;
///Write proxy for field `RXTHRLEN`
pub struct RXTHRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHRLEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 17)) | (((value as u32) & 0x01ff) << 17);
        self.w
    }
}
///Reader of field `ARPEN`
pub type ARPEN_R = crate::R<bool, bool>;
///Write proxy for field `ARPEN`
pub struct ARPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - ISO IN endpoint threshold enable
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:10 - Transmit threshold length
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bit 16 - Receive threshold enable
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 17:25 - Receive threshold length
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    ///Bit 27 - Arbiter parking enable
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W {
        NONISOTHREN_W { w: self }
    }
    ///Bit 1 - ISO IN endpoint threshold enable
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W {
        ISOTHREN_W { w: self }
    }
    ///Bits 2:10 - Transmit threshold length
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W {
        TXTHRLEN_W { w: self }
    }
    ///Bit 16 - Receive threshold enable
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W {
        RXTHREN_W { w: self }
    }
    ///Bits 17:25 - Receive threshold length
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W {
        RXTHRLEN_W { w: self }
    }
    ///Bit 27 - Arbiter parking enable
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W {
        ARPEN_W { w: self }
    }
}
