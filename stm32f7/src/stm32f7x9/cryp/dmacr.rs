///Reader of register DMACR
pub type R = crate::R<u32, super::DMACR>;
///Writer for register DMACR
pub type W = crate::W<u32, super::DMACR>;
///Register DMACR `reset()`'s with value 0
impl crate::ResetValue for super::DMACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DOEN`
pub type DOEN_R = crate::R<bool, bool>;
///Write proxy for field `DOEN`
pub struct DOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOEN_W<'a> {
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
///Reader of field `DIEN`
pub type DIEN_R = crate::R<bool, bool>;
///Write proxy for field `DIEN`
pub struct DIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIEN_W<'a> {
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
    ///Bit 1 - DMA output enable
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - DMA input enable
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - DMA output enable
    #[inline(always)]
    pub fn doen(&mut self) -> DOEN_W {
        DOEN_W { w: self }
    }
    ///Bit 0 - DMA input enable
    #[inline(always)]
    pub fn dien(&mut self) -> DIEN_W {
        DIEN_W { w: self }
    }
}
