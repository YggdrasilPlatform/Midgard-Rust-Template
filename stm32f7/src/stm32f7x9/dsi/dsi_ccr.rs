///Reader of register DSI_CCR
pub type R = crate::R<u32, super::DSI_CCR>;
///Writer for register DSI_CCR
pub type W = crate::W<u32, super::DSI_CCR>;
///Register DSI_CCR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TXECKDIV`
pub type TXECKDIV_R = crate::R<u8, u8>;
///Write proxy for field `TXECKDIV`
pub struct TXECKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXECKDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `TOCKDIV`
pub type TOCKDIV_R = crate::R<u8, u8>;
///Write proxy for field `TOCKDIV`
pub struct TOCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCKDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:7 - TX Escape Clock Division
    #[inline(always)]
    pub fn txeckdiv(&self) -> TXECKDIV_R {
        TXECKDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Timeout Clock Division
    #[inline(always)]
    pub fn tockdiv(&self) -> TOCKDIV_R {
        TOCKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - TX Escape Clock Division
    #[inline(always)]
    pub fn txeckdiv(&mut self) -> TXECKDIV_W {
        TXECKDIV_W { w: self }
    }
    ///Bits 8:15 - Timeout Clock Division
    #[inline(always)]
    pub fn tockdiv(&mut self) -> TOCKDIV_W {
        TOCKDIV_W { w: self }
    }
}
