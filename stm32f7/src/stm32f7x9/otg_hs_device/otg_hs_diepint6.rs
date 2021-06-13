///Reader of register OTG_HS_DIEPINT6
pub type R = crate::R<u32, super::OTG_HS_DIEPINT6>;
///Writer for register OTG_HS_DIEPINT6
pub type W = crate::W<u32, super::OTG_HS_DIEPINT6>;
///Register OTG_HS_DIEPINT6 `reset()`'s with value 0
impl crate::ResetValue for super::OTG_HS_DIEPINT6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `XFRC`
pub type XFRC_R = crate::R<bool, bool>;
///Write proxy for field `XFRC`
pub struct XFRC_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRC_W<'a> {
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
///Reader of field `EPDISD`
pub type EPDISD_R = crate::R<bool, bool>;
///Write proxy for field `EPDISD`
pub struct EPDISD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISD_W<'a> {
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
///Reader of field `TOC`
pub type TOC_R = crate::R<bool, bool>;
///Write proxy for field `TOC`
pub struct TOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC_W<'a> {
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
///Reader of field `ITTXFE`
pub type ITTXFE_R = crate::R<bool, bool>;
///Write proxy for field `ITTXFE`
pub struct ITTXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITTXFE_W<'a> {
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
///Reader of field `INEPNE`
pub type INEPNE_R = crate::R<bool, bool>;
///Write proxy for field `INEPNE`
pub struct INEPNE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNE_W<'a> {
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
///Reader of field `TXFE`
pub type TXFE_R = crate::R<bool, bool>;
///Reader of field `TXFIFOUDRN`
pub type TXFIFOUDRN_R = crate::R<bool, bool>;
///Write proxy for field `TXFIFOUDRN`
pub struct TXFIFOUDRN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOUDRN_W<'a> {
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
///Reader of field `BNA`
pub type BNA_R = crate::R<bool, bool>;
///Write proxy for field `BNA`
pub struct BNA_W<'a> {
    w: &'a mut W,
}
impl<'a> BNA_W<'a> {
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
///Reader of field `PKTDRPSTS`
pub type PKTDRPSTS_R = crate::R<bool, bool>;
///Write proxy for field `PKTDRPSTS`
pub struct PKTDRPSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTDRPSTS_W<'a> {
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
///Reader of field `BERR`
pub type BERR_R = crate::R<bool, bool>;
///Write proxy for field `BERR`
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
///Reader of field `NAK`
pub type NAK_R = crate::R<bool, bool>;
///Write proxy for field `NAK`
pub struct NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - Timeout condition
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - IN token received when TxFIFO is empty
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 6 - IN endpoint NAK effective
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Transmit FIFO empty
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Transmit Fifo Underrun
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Buffer not available interrupt
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 11 - Packet dropped status
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - NAK interrupt
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W { w: self }
    }
    ///Bit 1 - Endpoint disabled interrupt
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W {
        EPDISD_W { w: self }
    }
    ///Bit 3 - Timeout condition
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W {
        TOC_W { w: self }
    }
    ///Bit 4 - IN token received when TxFIFO is empty
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W {
        ITTXFE_W { w: self }
    }
    ///Bit 6 - IN endpoint NAK effective
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W {
        INEPNE_W { w: self }
    }
    ///Bit 8 - Transmit Fifo Underrun
    #[inline(always)]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W {
        TXFIFOUDRN_W { w: self }
    }
    ///Bit 9 - Buffer not available interrupt
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W {
        BNA_W { w: self }
    }
    ///Bit 11 - Packet dropped status
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W {
        PKTDRPSTS_W { w: self }
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
    ///Bit 13 - NAK interrupt
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W { w: self }
    }
}
