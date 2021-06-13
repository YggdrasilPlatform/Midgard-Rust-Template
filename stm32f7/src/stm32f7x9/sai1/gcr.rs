///Reader of register GCR
pub type R = crate::R<u32, super::GCR>;
///Writer for register GCR
pub type W = crate::W<u32, super::GCR>;
///Register GCR `reset()`'s with value 0
impl crate::ResetValue for super::GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SYNCIN`
pub type SYNCIN_R = crate::R<u8, u8>;
///Write proxy for field `SYNCIN`
pub struct SYNCIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCIN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `SYNCOUT`
pub type SYNCOUT_R = crate::R<u8, u8>;
///Write proxy for field `SYNCOUT`
pub struct SYNCOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCOUT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    pub fn syncin(&mut self) -> SYNCIN_W {
        SYNCIN_W { w: self }
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W {
        SYNCOUT_W { w: self }
    }
}
