///Reader of register OTG_HS_GAHBCFG
pub type R = crate::R<u32, super::OTG_HS_GAHBCFG>;
///Writer for register OTG_HS_GAHBCFG
pub type W = crate::W<u32, super::OTG_HS_GAHBCFG>;
///Register OTG_HS_GAHBCFG `reset()`'s with value 0
impl crate::ResetValue for super::OTG_HS_GAHBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `GINT`
pub type GINT_R = crate::R<bool, bool>;
///Write proxy for field `GINT`
pub struct GINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_W<'a> {
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
///Reader of field `HBSTLEN`
pub type HBSTLEN_R = crate::R<u8, u8>;
///Write proxy for field `HBSTLEN`
pub struct HBSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HBSTLEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
///Reader of field `DMAEN`
pub type DMAEN_R = crate::R<bool, bool>;
///Write proxy for field `DMAEN`
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
///Reader of field `TXFELVL`
pub type TXFELVL_R = crate::R<bool, bool>;
///Write proxy for field `TXFELVL`
pub struct TXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFELVL_W<'a> {
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
///Reader of field `PTXFELVL`
pub type PTXFELVL_R = crate::R<bool, bool>;
///Write proxy for field `PTXFELVL`
pub struct PTXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFELVL_W<'a> {
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
impl R {
    ///Bit 0 - Global interrupt mask
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:4 - Burst length/type
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 5 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 7 - TxFIFO empty level
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Periodic TxFIFO empty level
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Global interrupt mask
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W {
        GINT_W { w: self }
    }
    ///Bits 1:4 - Burst length/type
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W {
        HBSTLEN_W { w: self }
    }
    ///Bit 5 - DMA enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Bit 7 - TxFIFO empty level
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W {
        TXFELVL_W { w: self }
    }
    ///Bit 8 - Periodic TxFIFO empty level
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W {
        PTXFELVL_W { w: self }
    }
}
