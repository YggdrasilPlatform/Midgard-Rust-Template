///Reader of register OTG_HS_DTXFSTS7
pub type R = crate::R<u32, super::OTG_HS_DTXFSTS7>;
///Writer for register OTG_HS_DTXFSTS7
pub type W = crate::W<u32, super::OTG_HS_DTXFSTS7>;
///Register OTG_HS_DTXFSTS7 `reset()`'s with value 0
impl crate::ResetValue for super::OTG_HS_DTXFSTS7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `INEPTFSAV`
pub type INEPTFSAV_R = crate::R<u16, u16>;
///Write proxy for field `INEPTFSAV`
pub struct INEPTFSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTFSAV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - IN endpoint TxFIFO space avail
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - IN endpoint TxFIFO space avail
    #[inline(always)]
    pub fn ineptfsav(&mut self) -> INEPTFSAV_W {
        INEPTFSAV_W { w: self }
    }
}
