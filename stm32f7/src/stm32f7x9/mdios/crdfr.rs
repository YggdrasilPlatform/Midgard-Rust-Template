///Reader of register CRDFR
pub type R = crate::R<u32, super::CRDFR>;
///Writer for register CRDFR
pub type W = crate::W<u32, super::CRDFR>;
///Register CRDFR `reset()`'s with value 0
impl crate::ResetValue for super::CRDFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CRDF`
pub type CRDF_R = crate::R<u32, u32>;
///Write proxy for field `CRDF`
pub struct CRDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRDF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Clear the read flag
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Clear the read flag
    #[inline(always)]
    pub fn crdf(&mut self) -> CRDF_W {
        CRDF_W { w: self }
    }
}
