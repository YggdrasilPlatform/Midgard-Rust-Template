///Reader of register CSR1
pub type R = crate::R<u32, super::CSR1>;
///Writer for register CSR1
pub type W = crate::W<u32, super::CSR1>;
///Register CSR1 `reset()`'s with value 0
impl crate::ResetValue for super::CSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `WUIF`
pub type WUIF_R = crate::R<bool, bool>;
///Reader of field `SBF`
pub type SBF_R = crate::R<bool, bool>;
///Reader of field `PVDO`
pub type PVDO_R = crate::R<bool, bool>;
///Reader of field `BRR`
pub type BRR_R = crate::R<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `VOSRDY`
pub type VOSRDY_R = crate::R<bool, bool>;
///Write proxy for field `VOSRDY`
pub struct VOSRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> VOSRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Reader of field `ODRDY`
pub type ODRDY_R = crate::R<bool, bool>;
///Write proxy for field `ODRDY`
pub struct ODRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ODRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `ODSWRDY`
pub type ODSWRDY_R = crate::R<bool, bool>;
///Write proxy for field `ODSWRDY`
pub struct ODSWRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ODSWRDY_W<'a> {
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
///Reader of field `UDRDY`
pub type UDRDY_R = crate::R<u8, u8>;
///Write proxy for field `UDRDY`
pub struct UDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRDY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    ///Bit 0 - Wakeup internal flag
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - PVD output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Backup regulator ready
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 14 - Regulator voltage scaling output selection ready bit
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - Over-drive mode ready
    #[inline(always)]
    pub fn odrdy(&self) -> ODRDY_R {
        ODRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Over-drive mode switching ready
    #[inline(always)]
    pub fn odswrdy(&self) -> ODSWRDY_R {
        ODSWRDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bits 18:19 - Under-drive ready flag
    #[inline(always)]
    pub fn udrdy(&self) -> UDRDY_R {
        UDRDY_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W {
        BRE_W { w: self }
    }
    ///Bit 14 - Regulator voltage scaling output selection ready bit
    #[inline(always)]
    pub fn vosrdy(&mut self) -> VOSRDY_W {
        VOSRDY_W { w: self }
    }
    ///Bit 16 - Over-drive mode ready
    #[inline(always)]
    pub fn odrdy(&mut self) -> ODRDY_W {
        ODRDY_W { w: self }
    }
    ///Bit 17 - Over-drive mode switching ready
    #[inline(always)]
    pub fn odswrdy(&mut self) -> ODSWRDY_W {
        ODSWRDY_W { w: self }
    }
    ///Bits 18:19 - Under-drive ready flag
    #[inline(always)]
    pub fn udrdy(&mut self) -> UDRDY_W {
        UDRDY_W { w: self }
    }
}
