///Reader of register IER
pub type R = crate::R<u32, super::IER>;
///Writer for register IER
pub type W = crate::W<u32, super::IER>;
///Register IER `reset()`'s with value 0
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LINE_IE`
pub type LINE_IE_R = crate::R<bool, bool>;
///Write proxy for field `LINE_IE`
pub struct LINE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Reader of field `VSYNC_IE`
pub type VSYNC_IE_R = crate::R<bool, bool>;
///Write proxy for field `VSYNC_IE`
pub struct VSYNC_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_IE_W<'a> {
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
///Reader of field `ERR_IE`
pub type ERR_IE_R = crate::R<bool, bool>;
///Write proxy for field `ERR_IE`
pub struct ERR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `OVR_IE`
pub type OVR_IE_R = crate::R<bool, bool>;
///Write proxy for field `OVR_IE`
pub struct OVR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_IE_W<'a> {
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
///Reader of field `FRAME_IE`
pub type FRAME_IE_R = crate::R<bool, bool>;
///Write proxy for field `FRAME_IE`
pub struct FRAME_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_IE_W<'a> {
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
impl R {
    ///Bit 4 - Line interrupt enable
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - VSYNC interrupt enable
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Synchronization error interrupt enable
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Capture complete interrupt enable
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 4 - Line interrupt enable
    #[inline(always)]
    pub fn line_ie(&mut self) -> LINE_IE_W {
        LINE_IE_W { w: self }
    }
    ///Bit 3 - VSYNC interrupt enable
    #[inline(always)]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W {
        VSYNC_IE_W { w: self }
    }
    ///Bit 2 - Synchronization error interrupt enable
    #[inline(always)]
    pub fn err_ie(&mut self) -> ERR_IE_W {
        ERR_IE_W { w: self }
    }
    ///Bit 1 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OVR_IE_W {
        OVR_IE_W { w: self }
    }
    ///Bit 0 - Capture complete interrupt enable
    #[inline(always)]
    pub fn frame_ie(&mut self) -> FRAME_IE_W {
        FRAME_IE_W { w: self }
    }
}
