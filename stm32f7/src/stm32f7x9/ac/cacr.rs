///Reader of register CACR
pub type R = crate::R<u32, super::CACR>;
///Writer for register CACR
pub type W = crate::W<u32, super::CACR>;
///Register CACR `reset()`'s with value 0
impl crate::ResetValue for super::CACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SIWT`
pub type SIWT_R = crate::R<bool, bool>;
///Write proxy for field `SIWT`
pub struct SIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> SIWT_W<'a> {
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
///Reader of field `ECCEN`
pub type ECCEN_R = crate::R<bool, bool>;
///Write proxy for field `ECCEN`
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
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
///Reader of field `FORCEWT`
pub type FORCEWT_R = crate::R<bool, bool>;
///Write proxy for field `FORCEWT`
pub struct FORCEWT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEWT_W<'a> {
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
impl R {
    ///Bit 0 - SIWT
    #[inline(always)]
    pub fn siwt(&self) -> SIWT_R {
        SIWT_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - ECCEN
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - FORCEWT
    #[inline(always)]
    pub fn forcewt(&self) -> FORCEWT_R {
        FORCEWT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - SIWT
    #[inline(always)]
    pub fn siwt(&mut self) -> SIWT_W {
        SIWT_W { w: self }
    }
    ///Bit 1 - ECCEN
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    ///Bit 2 - FORCEWT
    #[inline(always)]
    pub fn forcewt(&mut self) -> FORCEWT_W {
        FORCEWT_W { w: self }
    }
}
