///Reader of register CWRFR
pub type R = crate::R<u32, super::CWRFR>;
///Writer for register CWRFR
pub type W = crate::W<u32, super::CWRFR>;
///Register CWRFR `reset()`'s with value 0
impl crate::ResetValue for super::CWRFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CWRF`
pub type CWRF_R = crate::R<u32, u32>;
///Write proxy for field `CWRF`
pub struct CWRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWRF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Clear the write flag
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Clear the write flag
    #[inline(always)]
    pub fn cwrf(&mut self) -> CWRF_W {
        CWRF_W { w: self }
    }
}
