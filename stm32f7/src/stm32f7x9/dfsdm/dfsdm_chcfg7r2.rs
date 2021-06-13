///Reader of register DFSDM_CHCFG7R2
pub type R = crate::R<u32, super::DFSDM_CHCFG7R2>;
///Writer for register DFSDM_CHCFG7R2
pub type W = crate::W<u32, super::DFSDM_CHCFG7R2>;
///Register DFSDM_CHCFG7R2 `reset()`'s with value 0
impl crate::ResetValue for super::DFSDM_CHCFG7R2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DTRBS`
pub type DTRBS_R = crate::R<u8, u8>;
///Write proxy for field `DTRBS`
pub struct DTRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRBS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
///Reader of field `OFFSET`
pub type OFFSET_R = crate::R<u32, u32>;
///Write proxy for field `OFFSET`
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    ///Bits 3:7 - Data right bit-shift for channel 7
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:31 - 24-bit calibration offset for channel 7
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    ///Bits 3:7 - Data right bit-shift for channel 7
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DTRBS_W {
        DTRBS_W { w: self }
    }
    ///Bits 8:31 - 24-bit calibration offset for channel 7
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
}
