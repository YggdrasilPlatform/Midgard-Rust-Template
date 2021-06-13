///Reader of register JPEG_CONFR4
pub type R = crate::R<u32, super::JPEG_CONFR4>;
///Writer for register JPEG_CONFR4
pub type W = crate::W<u32, super::JPEG_CONFR4>;
///Register JPEG_CONFR4 `reset()`'s with value 0
impl crate::ResetValue for super::JPEG_CONFR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HD`
pub type HD_R = crate::R<bool, bool>;
///Write proxy for field `HD`
pub struct HD_W<'a> {
    w: &'a mut W,
}
impl<'a> HD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `HA`
pub type HA_R = crate::R<bool, bool>;
///Write proxy for field `HA`
pub struct HA_W<'a> {
    w: &'a mut W,
}
impl<'a> HA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `QT`
pub type QT_R = crate::R<u8, u8>;
///Write proxy for field `QT`
pub struct QT_W<'a> {
    w: &'a mut W,
}
impl<'a> QT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `NB`
pub type NB_R = crate::R<u8, u8>;
///Write proxy for field `NB`
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Reader of field `VSF`
pub type VSF_R = crate::R<u8, u8>;
///Write proxy for field `VSF`
pub struct VSF_W<'a> {
    w: &'a mut W,
}
impl<'a> VSF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `HSF`
pub type HSF_R = crate::R<u8, u8>;
///Write proxy for field `HSF`
pub struct HSF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    ///Bit 0 - Huffman DC
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Huffman AC
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:3 - Quantization Table
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 4:7 - Number of Block
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Vertical Sampling Factor
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Horizontal Sampling Factor
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Huffman DC
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W {
        HD_W { w: self }
    }
    ///Bit 1 - Huffman AC
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W {
        HA_W { w: self }
    }
    ///Bits 2:3 - Quantization Table
    #[inline(always)]
    pub fn qt(&mut self) -> QT_W {
        QT_W { w: self }
    }
    ///Bits 4:7 - Number of Block
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
    ///Bits 8:11 - Vertical Sampling Factor
    #[inline(always)]
    pub fn vsf(&mut self) -> VSF_W {
        VSF_W { w: self }
    }
    ///Bits 12:15 - Horizontal Sampling Factor
    #[inline(always)]
    pub fn hsf(&mut self) -> HSF_W {
        HSF_W { w: self }
    }
}
