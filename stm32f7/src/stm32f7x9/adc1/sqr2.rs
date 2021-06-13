///Reader of register SQR2
pub type R = crate::R<u32, super::SQR2>;
///Writer for register SQR2
pub type W = crate::W<u32, super::SQR2>;
///Register SQR2 `reset()`'s with value 0
impl crate::ResetValue for super::SQR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SQ12`
pub type SQ12_R = crate::R<u8, u8>;
///Write proxy for field `SQ12`
pub struct SQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ12_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
///Reader of field `SQ11`
pub type SQ11_R = crate::R<u8, u8>;
///Write proxy for field `SQ11`
pub struct SQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ11_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
///Reader of field `SQ10`
pub type SQ10_R = crate::R<u8, u8>;
///Write proxy for field `SQ10`
pub struct SQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ10_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
///Reader of field `SQ9`
pub type SQ9_R = crate::R<u8, u8>;
///Write proxy for field `SQ9`
pub struct SQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ9_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
///Reader of field `SQ8`
pub type SQ8_R = crate::R<u8, u8>;
///Write proxy for field `SQ8`
pub struct SQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ8_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
///Reader of field `SQ7`
pub type SQ7_R = crate::R<u8, u8>;
///Write proxy for field `SQ7`
pub struct SQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ7_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    ///Bits 25:29 - 12th conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    ///Bits 20:24 - 11th conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 15:19 - 10th conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 10:14 - 9th conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 5:9 - 8th conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 0:4 - 7th conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 25:29 - 12th conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W {
        SQ12_W { w: self }
    }
    ///Bits 20:24 - 11th conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W {
        SQ11_W { w: self }
    }
    ///Bits 15:19 - 10th conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W {
        SQ10_W { w: self }
    }
    ///Bits 10:14 - 9th conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ9_W {
        SQ9_W { w: self }
    }
    ///Bits 5:9 - 8th conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W {
        SQ8_W { w: self }
    }
    ///Bits 0:4 - 7th conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W {
        SQ7_W { w: self }
    }
}
