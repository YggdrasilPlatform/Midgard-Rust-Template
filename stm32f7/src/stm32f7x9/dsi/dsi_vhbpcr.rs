///Reader of register DSI_VHBPCR
pub type R = crate::R<u32, super::DSI_VHBPCR>;
///Writer for register DSI_VHBPCR
pub type W = crate::W<u32, super::DSI_VHBPCR>;
///Register DSI_VHBPCR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_VHBPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HBP`
pub type HBP_R = crate::R<u16, u16>;
///Write proxy for field `HBP`
pub struct HBP_W<'a> {
    w: &'a mut W,
}
impl<'a> HBP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - Horizontal Back-Porch duration
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Horizontal Back-Porch duration
    #[inline(always)]
    pub fn hbp(&mut self) -> HBP_W {
        HBP_W { w: self }
    }
}
