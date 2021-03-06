///Reader of register OTG_FS_GLPMCFG
pub type R = crate::R<u32, super::OTG_FS_GLPMCFG>;
///Writer for register OTG_FS_GLPMCFG
pub type W = crate::W<u32, super::OTG_FS_GLPMCFG>;
///Register OTG_FS_GLPMCFG `reset()`'s with value 0x0200_0400
impl crate::ResetValue for super::OTG_FS_GLPMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0400
    }
}
///Reader of field `LPMEN`
pub type LPMEN_R = crate::R<bool, bool>;
///Write proxy for field `LPMEN`
pub struct LPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMEN_W<'a> {
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
///Reader of field `LPMACK`
pub type LPMACK_R = crate::R<bool, bool>;
///Write proxy for field `LPMACK`
pub struct LPMACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMACK_W<'a> {
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
///Reader of field `BESL`
pub type BESL_R = crate::R<u8, u8>;
///Write proxy for field `BESL`
pub struct BESL_W<'a> {
    w: &'a mut W,
}
impl<'a> BESL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
///Reader of field `REMWAKE`
pub type REMWAKE_R = crate::R<bool, bool>;
///Write proxy for field `REMWAKE`
pub struct REMWAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> REMWAKE_W<'a> {
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
///Reader of field `L1SSEN`
pub type L1SSEN_R = crate::R<bool, bool>;
///Write proxy for field `L1SSEN`
pub struct L1SSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1SSEN_W<'a> {
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
///Reader of field `BESLTHRS`
pub type BESLTHRS_R = crate::R<u8, u8>;
///Write proxy for field `BESLTHRS`
pub struct BESLTHRS_W<'a> {
    w: &'a mut W,
}
impl<'a> BESLTHRS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `L1DSEN`
pub type L1DSEN_R = crate::R<bool, bool>;
///Write proxy for field `L1DSEN`
pub struct L1DSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1DSEN_W<'a> {
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
///Reader of field `LPMRST`
pub type LPMRST_R = crate::R<u8, u8>;
///Reader of field `SLPSTS`
pub type SLPSTS_R = crate::R<bool, bool>;
///Reader of field `L1RSMOK`
pub type L1RSMOK_R = crate::R<bool, bool>;
///Reader of field `LPMCHIDX`
pub type LPMCHIDX_R = crate::R<u8, u8>;
///Write proxy for field `LPMCHIDX`
pub struct LPMCHIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMCHIDX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 17)) | (((value as u32) & 0x0f) << 17);
        self.w
    }
}
///Reader of field `LPMRCNT`
pub type LPMRCNT_R = crate::R<u8, u8>;
///Write proxy for field `LPMRCNT`
pub struct LPMRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMRCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
///Reader of field `SNDLPM`
pub type SNDLPM_R = crate::R<bool, bool>;
///Write proxy for field `SNDLPM`
pub struct SNDLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SNDLPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `LPMRCNTSTS`
pub type LPMRCNTSTS_R = crate::R<u8, u8>;
///Reader of field `ENBESL`
pub type ENBESL_R = crate::R<bool, bool>;
///Write proxy for field `ENBESL`
pub struct ENBESL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBESL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - LPM token acknowledge enable
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:5 - Best effort service latency
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - bRemoteWake value
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - L1 Shallow Sleep enable
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:11 - BESL threshold
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - L1 deep sleep enable
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 13:14 - LPM response
    #[inline(always)]
    pub fn lpmrst(&self) -> LPMRST_R {
        LPMRST_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    ///Bit 15 - Port sleep status
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Sleep State Resume OK
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 17:20 - LPM Channel Index
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:23 - LPM retry count
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LPMRCNT_R {
        LPMRCNT_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    ///Bit 24 - Send LPM transaction
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 25:27 - LPM retry count status
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    ///Bit 28 - Enable best effort service latency
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W {
        LPMEN_W { w: self }
    }
    ///Bit 1 - LPM token acknowledge enable
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W {
        LPMACK_W { w: self }
    }
    ///Bits 2:5 - Best effort service latency
    #[inline(always)]
    pub fn besl(&mut self) -> BESL_W {
        BESL_W { w: self }
    }
    ///Bit 6 - bRemoteWake value
    #[inline(always)]
    pub fn remwake(&mut self) -> REMWAKE_W {
        REMWAKE_W { w: self }
    }
    ///Bit 7 - L1 Shallow Sleep enable
    #[inline(always)]
    pub fn l1ssen(&mut self) -> L1SSEN_W {
        L1SSEN_W { w: self }
    }
    ///Bits 8:11 - BESL threshold
    #[inline(always)]
    pub fn beslthrs(&mut self) -> BESLTHRS_W {
        BESLTHRS_W { w: self }
    }
    ///Bit 12 - L1 deep sleep enable
    #[inline(always)]
    pub fn l1dsen(&mut self) -> L1DSEN_W {
        L1DSEN_W { w: self }
    }
    ///Bits 17:20 - LPM Channel Index
    #[inline(always)]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W {
        LPMCHIDX_W { w: self }
    }
    ///Bits 21:23 - LPM retry count
    #[inline(always)]
    pub fn lpmrcnt(&mut self) -> LPMRCNT_W {
        LPMRCNT_W { w: self }
    }
    ///Bit 24 - Send LPM transaction
    #[inline(always)]
    pub fn sndlpm(&mut self) -> SNDLPM_W {
        SNDLPM_W { w: self }
    }
    ///Bit 28 - Enable best effort service latency
    #[inline(always)]
    pub fn enbesl(&mut self) -> ENBESL_W {
        ENBESL_W { w: self }
    }
}
