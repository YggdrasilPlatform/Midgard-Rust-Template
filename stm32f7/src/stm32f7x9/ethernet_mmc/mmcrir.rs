///Reader of register MMCRIR
pub type R = crate::R<u32, super::MMCRIR>;
///Writer for register MMCRIR
pub type W = crate::W<u32, super::MMCRIR>;
///Register MMCRIR `reset()`'s with value 0
impl crate::ResetValue for super::MMCRIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RFCES`
pub type RFCES_R = crate::R<bool, bool>;
///Write proxy for field `RFCES`
pub struct RFCES_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `RFAES`
pub type RFAES_R = crate::R<bool, bool>;
///Write proxy for field `RFAES`
pub struct RFAES_W<'a> {
    w: &'a mut W,
}
impl<'a> RFAES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Reader of field `RGUFS`
pub type RGUFS_R = crate::R<bool, bool>;
///Write proxy for field `RGUFS`
pub struct RGUFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RGUFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    ///Bit 5 - Received frames CRC error status
    #[inline(always)]
    pub fn rfces(&self) -> RFCES_R {
        RFCES_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Received frames alignment error status
    #[inline(always)]
    pub fn rfaes(&self) -> RFAES_R {
        RFAES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 17 - Received good Unicast frames status
    #[inline(always)]
    pub fn rgufs(&self) -> RGUFS_R {
        RGUFS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    ///Bit 5 - Received frames CRC error status
    #[inline(always)]
    pub fn rfces(&mut self) -> RFCES_W {
        RFCES_W { w: self }
    }
    ///Bit 6 - Received frames alignment error status
    #[inline(always)]
    pub fn rfaes(&mut self) -> RFAES_W {
        RFAES_W { w: self }
    }
    ///Bit 17 - Received good Unicast frames status
    #[inline(always)]
    pub fn rgufs(&mut self) -> RGUFS_W {
        RGUFS_W { w: self }
    }
}
