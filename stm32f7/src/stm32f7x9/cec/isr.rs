///Reader of register ISR
pub type R = crate::R<u32, super::ISR>;
///Writer for register ISR
pub type W = crate::W<u32, super::ISR>;
///Register ISR `reset()`'s with value 0
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TXACKE`
pub type TXACKE_R = crate::R<bool, bool>;
///Write proxy for field `TXACKE`
pub struct TXACKE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACKE_W<'a> {
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
///Reader of field `TXERR`
pub type TXERR_R = crate::R<bool, bool>;
///Write proxy for field `TXERR`
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
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
///Reader of field `TXUDR`
pub type TXUDR_R = crate::R<bool, bool>;
///Write proxy for field `TXUDR`
pub struct TXUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDR_W<'a> {
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
///Reader of field `TXEND`
pub type TXEND_R = crate::R<bool, bool>;
///Write proxy for field `TXEND`
pub struct TXEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `TXBR`
pub type TXBR_R = crate::R<bool, bool>;
///Write proxy for field `TXBR`
pub struct TXBR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBR_W<'a> {
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
///Reader of field `ARBLST`
pub type ARBLST_R = crate::R<bool, bool>;
///Write proxy for field `ARBLST`
pub struct ARBLST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLST_W<'a> {
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
///Reader of field `RXACKE`
pub type RXACKE_R = crate::R<bool, bool>;
///Write proxy for field `RXACKE`
pub struct RXACKE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACKE_W<'a> {
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
///Reader of field `LBPE`
pub type LBPE_R = crate::R<bool, bool>;
///Write proxy for field `LBPE`
pub struct LBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPE_W<'a> {
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
///Reader of field `SBPE`
pub type SBPE_R = crate::R<bool, bool>;
///Write proxy for field `SBPE`
pub struct SBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBPE_W<'a> {
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
///Reader of field `BRE`
pub type BRE_R = crate::R<bool, bool>;
///Write proxy for field `BRE`
pub struct BRE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRE_W<'a> {
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
///Reader of field `RXOVR`
pub type RXOVR_R = crate::R<bool, bool>;
///Write proxy for field `RXOVR`
pub struct RXOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVR_W<'a> {
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
///Reader of field `RXEND`
pub type RXEND_R = crate::R<bool, bool>;
///Write proxy for field `RXEND`
pub struct RXEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEND_W<'a> {
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
///Reader of field `RXBR`
pub type RXBR_R = crate::R<bool, bool>;
///Write proxy for field `RXBR`
pub struct RXBR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBR_W<'a> {
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
    ///Bit 12 - Tx-Missing acknowledge error
    #[inline(always)]
    pub fn txacke(&self) -> TXACKE_R {
        TXACKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Tx-Error
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Tx-Buffer Underrun
    #[inline(always)]
    pub fn txudr(&self) -> TXUDR_R {
        TXUDR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - End of Transmission
    #[inline(always)]
    pub fn txend(&self) -> TXEND_R {
        TXEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Tx-Byte Request
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Arbitration Lost
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Rx-Missing Acknowledge
    #[inline(always)]
    pub fn rxacke(&self) -> RXACKE_R {
        RXACKE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Rx-Long Bit Period Error
    #[inline(always)]
    pub fn lbpe(&self) -> LBPE_R {
        LBPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Rx-Short Bit period error
    #[inline(always)]
    pub fn sbpe(&self) -> SBPE_R {
        SBPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Rx-Bit rising error
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Rx-Overrun
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - End Of Reception
    #[inline(always)]
    pub fn rxend(&self) -> RXEND_R {
        RXEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Rx-Byte Received
    #[inline(always)]
    pub fn rxbr(&self) -> RXBR_R {
        RXBR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 12 - Tx-Missing acknowledge error
    #[inline(always)]
    pub fn txacke(&mut self) -> TXACKE_W {
        TXACKE_W { w: self }
    }
    ///Bit 11 - Tx-Error
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    ///Bit 10 - Tx-Buffer Underrun
    #[inline(always)]
    pub fn txudr(&mut self) -> TXUDR_W {
        TXUDR_W { w: self }
    }
    ///Bit 9 - End of Transmission
    #[inline(always)]
    pub fn txend(&mut self) -> TXEND_W {
        TXEND_W { w: self }
    }
    ///Bit 8 - Tx-Byte Request
    #[inline(always)]
    pub fn txbr(&mut self) -> TXBR_W {
        TXBR_W { w: self }
    }
    ///Bit 7 - Arbitration Lost
    #[inline(always)]
    pub fn arblst(&mut self) -> ARBLST_W {
        ARBLST_W { w: self }
    }
    ///Bit 6 - Rx-Missing Acknowledge
    #[inline(always)]
    pub fn rxacke(&mut self) -> RXACKE_W {
        RXACKE_W { w: self }
    }
    ///Bit 5 - Rx-Long Bit Period Error
    #[inline(always)]
    pub fn lbpe(&mut self) -> LBPE_W {
        LBPE_W { w: self }
    }
    ///Bit 4 - Rx-Short Bit period error
    #[inline(always)]
    pub fn sbpe(&mut self) -> SBPE_W {
        SBPE_W { w: self }
    }
    ///Bit 3 - Rx-Bit rising error
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W {
        BRE_W { w: self }
    }
    ///Bit 2 - Rx-Overrun
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W {
        RXOVR_W { w: self }
    }
    ///Bit 1 - End Of Reception
    #[inline(always)]
    pub fn rxend(&mut self) -> RXEND_W {
        RXEND_W { w: self }
    }
    ///Bit 0 - Rx-Byte Received
    #[inline(always)]
    pub fn rxbr(&mut self) -> RXBR_W {
        RXBR_W { w: self }
    }
}
