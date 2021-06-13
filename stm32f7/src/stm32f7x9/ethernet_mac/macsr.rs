///Reader of register MACSR
pub type R = crate::R<u32, super::MACSR>;
///Writer for register MACSR
pub type W = crate::W<u32, super::MACSR>;
///Register MACSR `reset()`'s with value 0
impl crate::ResetValue for super::MACSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PMTS`
pub type PMTS_R = crate::R<bool, bool>;
///Reader of field `MMCS`
pub type MMCS_R = crate::R<bool, bool>;
///Reader of field `MMCRS`
pub type MMCRS_R = crate::R<bool, bool>;
///Reader of field `MMCTS`
pub type MMCTS_R = crate::R<bool, bool>;
///Reader of field `TSTS`
pub type TSTS_R = crate::R<bool, bool>;
///Write proxy for field `TSTS`
pub struct TSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTS_W<'a> {
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
impl R {
    ///Bit 3 - PMT status
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - MMC status
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - MMC receive status
    #[inline(always)]
    pub fn mmcrs(&self) -> MMCRS_R {
        MMCRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - MMC transmit status
    #[inline(always)]
    pub fn mmcts(&self) -> MMCTS_R {
        MMCTS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 9 - Time stamp trigger status
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    ///Bit 9 - Time stamp trigger status
    #[inline(always)]
    pub fn tsts(&mut self) -> TSTS_W {
        TSTS_W { w: self }
    }
}
