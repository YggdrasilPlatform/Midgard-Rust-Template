///Reader of register OTG_HS_DIEPTSIZ0
pub type R = crate::R<u32, super::OTG_HS_DIEPTSIZ0>;
///Writer for register OTG_HS_DIEPTSIZ0
pub type W = crate::W<u32, super::OTG_HS_DIEPTSIZ0>;
///Register OTG_HS_DIEPTSIZ0 `reset()`'s with value 0
impl crate::ResetValue for super::OTG_HS_DIEPTSIZ0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `XFRSIZ`
pub type XFRSIZ_R = crate::R<u8, u8>;
///Write proxy for field `XFRSIZ`
pub struct XFRSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRSIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Reader of field `PKTCNT`
pub type PKTCNT_R = crate::R<u8, u8>;
///Write proxy for field `PKTCNT`
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
impl R {
    ///Bits 0:6 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 19:20 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:6 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W {
        XFRSIZ_W { w: self }
    }
    ///Bits 19:20 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
}
