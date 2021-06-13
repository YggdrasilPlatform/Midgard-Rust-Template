///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EN`
pub type EN_R = crate::R<bool, bool>;
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
///Reader of field `WRIE`
pub type WRIE_R = crate::R<bool, bool>;
///Write proxy for field `WRIE`
pub struct WRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRIE_W<'a> {
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
///Reader of field `RDIE`
pub type RDIE_R = crate::R<bool, bool>;
///Write proxy for field `RDIE`
pub struct RDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIE_W<'a> {
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
///Reader of field `EIE`
pub type EIE_R = crate::R<bool, bool>;
///Write proxy for field `EIE`
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
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
///Reader of field `DPC`
pub type DPC_R = crate::R<bool, bool>;
///Write proxy for field `DPC`
pub struct DPC_W<'a> {
    w: &'a mut W,
}
impl<'a> DPC_W<'a> {
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
///Reader of field `PORT_ADDRESS`
pub type PORT_ADDRESS_R = crate::R<u8, u8>;
///Write proxy for field `PORT_ADDRESS`
pub struct PORT_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_ADDRESS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Register write interrupt enable
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Register Read Interrupt Enable
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 7 - Disable Preamble Check
    #[inline(always)]
    pub fn dpc(&self) -> DPC_R {
        DPC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:12 - Slaves's address
    #[inline(always)]
    pub fn port_address(&self) -> PORT_ADDRESS_R {
        PORT_ADDRESS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    ///Bit 1 - Register write interrupt enable
    #[inline(always)]
    pub fn wrie(&mut self) -> WRIE_W {
        WRIE_W { w: self }
    }
    ///Bit 2 - Register Read Interrupt Enable
    #[inline(always)]
    pub fn rdie(&mut self) -> RDIE_W {
        RDIE_W { w: self }
    }
    ///Bit 3 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    ///Bit 7 - Disable Preamble Check
    #[inline(always)]
    pub fn dpc(&mut self) -> DPC_W {
        DPC_W { w: self }
    }
    ///Bits 8:12 - Slaves's address
    #[inline(always)]
    pub fn port_address(&mut self) -> PORT_ADDRESS_W {
        PORT_ADDRESS_W { w: self }
    }
}
