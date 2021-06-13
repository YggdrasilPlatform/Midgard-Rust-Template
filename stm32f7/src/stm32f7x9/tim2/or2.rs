///Reader of register OR2
pub type R = crate::R<u32, super::OR2>;
///Writer for register OR2
pub type W = crate::W<u32, super::OR2>;
///Register OR2 `reset()`'s with value 0
impl crate::ResetValue for super::OR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ETRSEL`
pub type ETRSEL_R = crate::R<u8, u8>;
///Write proxy for field `ETRSEL`
pub struct ETRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
impl R {
    ///Bits 14:16 - ETR source selection
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x07) as u8)
    }
}
impl W {
    ///Bits 14:16 - ETR source selection
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W {
        ETRSEL_W { w: self }
    }
}
