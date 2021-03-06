///Reader of register JPEG_CONFR1
pub type R = crate::R<u32, super::JPEG_CONFR1>;
///Writer for register JPEG_CONFR1
pub type W = crate::W<u32, super::JPEG_CONFR1>;
///Register JPEG_CONFR1 `reset()`'s with value 0
impl crate::ResetValue for super::JPEG_CONFR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `NF`
pub type NF_R = crate::R<u8, u8>;
///Write proxy for field `NF`
pub struct NF_W<'a> {
    w: &'a mut W,
}
impl<'a> NF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `DE`
pub type DE_R = crate::R<bool, bool>;
///Write proxy for field `DE`
pub struct DE_W<'a> {
    w: &'a mut W,
}
impl<'a> DE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `COLORSPACE`
pub type COLORSPACE_R = crate::R<u8, u8>;
///Write proxy for field `COLORSPACE`
pub struct COLORSPACE_W<'a> {
    w: &'a mut W,
}
impl<'a> COLORSPACE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///Reader of field `NS`
pub type NS_R = crate::R<u8, u8>;
///Write proxy for field `NS`
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
///Reader of field `HDR`
pub type HDR_R = crate::R<bool, bool>;
///Write proxy for field `HDR`
pub struct HDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDR_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Reader of field `YSIZE`
pub type YSIZE_R = crate::R<u16, u16>;
///Write proxy for field `YSIZE`
pub struct YSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> YSIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Number of color components
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new((self.bits & 0x03) as u8)
    }
    ///Bit 3 - Decoding Enable
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:5 - Color Space
    #[inline(always)]
    pub fn colorspace(&self) -> COLORSPACE_R {
        COLORSPACE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 6:7 - Number of components for Scan
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bit 8 - Header Processing
    #[inline(always)]
    pub fn hdr(&self) -> HDR_R {
        HDR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 16:31 - Y Size
    #[inline(always)]
    pub fn ysize(&self) -> YSIZE_R {
        YSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:1 - Number of color components
    #[inline(always)]
    pub fn nf(&mut self) -> NF_W {
        NF_W { w: self }
    }
    ///Bit 3 - Decoding Enable
    #[inline(always)]
    pub fn de(&mut self) -> DE_W {
        DE_W { w: self }
    }
    ///Bits 4:5 - Color Space
    #[inline(always)]
    pub fn colorspace(&mut self) -> COLORSPACE_W {
        COLORSPACE_W { w: self }
    }
    ///Bits 6:7 - Number of components for Scan
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    ///Bit 8 - Header Processing
    #[inline(always)]
    pub fn hdr(&mut self) -> HDR_W {
        HDR_W { w: self }
    }
    ///Bits 16:31 - Y Size
    #[inline(always)]
    pub fn ysize(&mut self) -> YSIZE_W {
        YSIZE_W { w: self }
    }
}
