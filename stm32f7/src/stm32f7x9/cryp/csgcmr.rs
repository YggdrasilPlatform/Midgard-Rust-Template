///Reader of register CSGCM%sR
pub type R = crate::R<u32, super::CSGCMR>;
///Writer for register CSGCM%sR
pub type W = crate::W<u32, super::CSGCMR>;
///Register CSGCM%sR `reset()`'s with value 0
impl crate::ResetValue for super::CSGCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSGCMR`
pub type CSGCMR_R = crate::R<u32, u32>;
///Write proxy for field `CSGCMR`
pub struct CSGCMR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSGCM0R
    #[inline(always)]
    pub fn csgcmr(&self) -> CSGCMR_R {
        CSGCMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSGCM0R
    #[inline(always)]
    pub fn csgcmr(&mut self) -> CSGCMR_W {
        CSGCMR_W { w: self }
    }
}
