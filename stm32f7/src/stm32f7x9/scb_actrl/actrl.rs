///Reader of register ACTRL
pub type R = crate::R<u32, super::ACTRL>;
///Writer for register ACTRL
pub type W = crate::W<u32, super::ACTRL>;
///Register ACTRL `reset()`'s with value 0
impl crate::ResetValue for super::ACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DISFOLD`
pub type DISFOLD_R = crate::R<bool, bool>;
///Write proxy for field `DISFOLD`
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
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
///Reader of field `FPEXCODIS`
pub type FPEXCODIS_R = crate::R<bool, bool>;
///Write proxy for field `FPEXCODIS`
pub struct FPEXCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPEXCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Reader of field `DISRAMODE`
pub type DISRAMODE_R = crate::R<bool, bool>;
///Write proxy for field `DISRAMODE`
pub struct DISRAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISRAMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `DISITMATBFLUSH`
pub type DISITMATBFLUSH_R = crate::R<bool, bool>;
///Write proxy for field `DISITMATBFLUSH`
pub struct DISITMATBFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> DISITMATBFLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    ///Bit 2 - DISFOLD
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 10 - FPEXCODIS
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - DISRAMODE
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - DISITMATBFLUSH
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - DISFOLD
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    ///Bit 10 - FPEXCODIS
    #[inline(always)]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W {
        FPEXCODIS_W { w: self }
    }
    ///Bit 11 - DISRAMODE
    #[inline(always)]
    pub fn disramode(&mut self) -> DISRAMODE_W {
        DISRAMODE_W { w: self }
    }
    ///Bit 12 - DISITMATBFLUSH
    #[inline(always)]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W {
        DISITMATBFLUSH_W { w: self }
    }
}
