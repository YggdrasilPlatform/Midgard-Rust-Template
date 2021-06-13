///Reader of register IMSCR
pub type R = crate::R<u32, super::IMSCR>;
///Writer for register IMSCR
pub type W = crate::W<u32, super::IMSCR>;
///Register IMSCR `reset()`'s with value 0
impl crate::ResetValue for super::IMSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OUTIM`
pub type OUTIM_R = crate::R<bool, bool>;
///Write proxy for field `OUTIM`
pub struct OUTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTIM_W<'a> {
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
///Reader of field `INIM`
pub type INIM_R = crate::R<bool, bool>;
///Write proxy for field `INIM`
pub struct INIM_W<'a> {
    w: &'a mut W,
}
impl<'a> INIM_W<'a> {
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
    ///Bit 1 - Output FIFO service interrupt mask
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Input FIFO service interrupt mask
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Output FIFO service interrupt mask
    #[inline(always)]
    pub fn outim(&mut self) -> OUTIM_W {
        OUTIM_W { w: self }
    }
    ///Bit 0 - Input FIFO service interrupt mask
    #[inline(always)]
    pub fn inim(&mut self) -> INIM_W {
        INIM_W { w: self }
    }
}
