///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EOP`
pub type EOP_R = crate::R<bool, bool>;
///Write proxy for field `EOP`
pub struct EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_W<'a> {
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
///Reader of field `OPERR`
pub type OPERR_R = crate::R<bool, bool>;
///Write proxy for field `OPERR`
pub struct OPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERR_W<'a> {
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
///Reader of field `WRPERR`
pub type WRPERR_R = crate::R<bool, bool>;
///Write proxy for field `WRPERR`
pub struct WRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPERR_W<'a> {
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
///Reader of field `PGAERR`
pub type PGAERR_R = crate::R<bool, bool>;
///Write proxy for field `PGAERR`
pub struct PGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGAERR_W<'a> {
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
///Reader of field `PGPERR`
pub type PGPERR_R = crate::R<bool, bool>;
///Write proxy for field `PGPERR`
pub struct PGPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGPERR_W<'a> {
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
///Reader of field `ERSERR`
pub type ERSERR_R = crate::R<bool, bool>;
///Write proxy for field `ERSERR`
pub struct ERSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Reader of field `BSY`
pub type BSY_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Programming parallelism error
    #[inline(always)]
    pub fn pgperr(&self) -> PGPERR_R {
        PGPERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn erserr(&self) -> ERSERR_R {
        ERSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W { w: self }
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W {
        OPERR_W { w: self }
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W { w: self }
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W {
        PGAERR_W { w: self }
    }
    ///Bit 6 - Programming parallelism error
    #[inline(always)]
    pub fn pgperr(&mut self) -> PGPERR_W {
        PGPERR_W { w: self }
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn erserr(&mut self) -> ERSERR_W {
        ERSERR_W { w: self }
    }
}
