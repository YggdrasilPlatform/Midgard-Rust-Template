///Reader of register DMAIER
pub type R = crate::R<u32, super::DMAIER>;
///Writer for register DMAIER
pub type W = crate::W<u32, super::DMAIER>;
///Register DMAIER `reset()`'s with value 0
impl crate::ResetValue for super::DMAIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TIE`
pub type TIE_R = crate::R<bool, bool>;
///Write proxy for field `TIE`
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
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
///Reader of field `TPSIE`
pub type TPSIE_R = crate::R<bool, bool>;
///Write proxy for field `TPSIE`
pub struct TPSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPSIE_W<'a> {
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
///Reader of field `TBUIE`
pub type TBUIE_R = crate::R<bool, bool>;
///Write proxy for field `TBUIE`
pub struct TBUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUIE_W<'a> {
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
///Reader of field `TJTIE`
pub type TJTIE_R = crate::R<bool, bool>;
///Write proxy for field `TJTIE`
pub struct TJTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TJTIE_W<'a> {
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
///Reader of field `ROIE`
pub type ROIE_R = crate::R<bool, bool>;
///Write proxy for field `ROIE`
pub struct ROIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROIE_W<'a> {
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
///Reader of field `TUIE`
pub type TUIE_R = crate::R<bool, bool>;
///Write proxy for field `TUIE`
pub struct TUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TUIE_W<'a> {
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
///Reader of field `RIE`
pub type RIE_R = crate::R<bool, bool>;
///Write proxy for field `RIE`
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
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
///Reader of field `RBUIE`
pub type RBUIE_R = crate::R<bool, bool>;
///Write proxy for field `RBUIE`
pub struct RBUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBUIE_W<'a> {
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
///Reader of field `RPSIE`
pub type RPSIE_R = crate::R<bool, bool>;
///Write proxy for field `RPSIE`
pub struct RPSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPSIE_W<'a> {
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
///Reader of field `RWTIE`
pub type RWTIE_R = crate::R<bool, bool>;
///Write proxy for field `RWTIE`
pub struct RWTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWTIE_W<'a> {
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
///Reader of field `ETIE`
pub type ETIE_R = crate::R<bool, bool>;
///Write proxy for field `ETIE`
pub struct ETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETIE_W<'a> {
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
///Reader of field `FBEIE`
pub type FBEIE_R = crate::R<bool, bool>;
///Write proxy for field `FBEIE`
pub struct FBEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBEIE_W<'a> {
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
///Reader of field `ERIE`
pub type ERIE_R = crate::R<bool, bool>;
///Write proxy for field `ERIE`
pub struct ERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERIE_W<'a> {
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
///Reader of field `AISE`
pub type AISE_R = crate::R<bool, bool>;
///Write proxy for field `AISE`
pub struct AISE_W<'a> {
    w: &'a mut W,
}
impl<'a> AISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Reader of field `NISE`
pub type NISE_R = crate::R<bool, bool>;
///Write proxy for field `NISE`
pub struct NISE_W<'a> {
    w: &'a mut W,
}
impl<'a> NISE_W<'a> {
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
impl R {
    ///Bit 0 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Transmit process stopped interrupt enable
    #[inline(always)]
    pub fn tpsie(&self) -> TPSIE_R {
        TPSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Transmit buffer unavailable interrupt enable
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Transmit jabber timeout interrupt enable
    #[inline(always)]
    pub fn tjtie(&self) -> TJTIE_R {
        TJTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Receive overflow interrupt enable
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Transmit underflow interrupt enable
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Receive buffer unavailable interrupt enable
    #[inline(always)]
    pub fn rbuie(&self) -> RBUIE_R {
        RBUIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Receive process stopped interrupt enable
    #[inline(always)]
    pub fn rpsie(&self) -> RPSIE_R {
        RPSIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Receive watchdog timeout interrupt enable
    #[inline(always)]
    pub fn rwtie(&self) -> RWTIE_R {
        RWTIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Early transmit interrupt enable
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 13 - Fatal bus error interrupt enable
    #[inline(always)]
    pub fn fbeie(&self) -> FBEIE_R {
        FBEIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Early receive interrupt enable
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Abnormal interrupt summary enable
    #[inline(always)]
    pub fn aise(&self) -> AISE_R {
        AISE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Normal interrupt summary enable
    #[inline(always)]
    pub fn nise(&self) -> NISE_R {
        NISE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    ///Bit 1 - Transmit process stopped interrupt enable
    #[inline(always)]
    pub fn tpsie(&mut self) -> TPSIE_W {
        TPSIE_W { w: self }
    }
    ///Bit 2 - Transmit buffer unavailable interrupt enable
    #[inline(always)]
    pub fn tbuie(&mut self) -> TBUIE_W {
        TBUIE_W { w: self }
    }
    ///Bit 3 - Transmit jabber timeout interrupt enable
    #[inline(always)]
    pub fn tjtie(&mut self) -> TJTIE_W {
        TJTIE_W { w: self }
    }
    ///Bit 4 - Receive overflow interrupt enable
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W {
        ROIE_W { w: self }
    }
    ///Bit 5 - Transmit underflow interrupt enable
    #[inline(always)]
    pub fn tuie(&mut self) -> TUIE_W {
        TUIE_W { w: self }
    }
    ///Bit 6 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    ///Bit 7 - Receive buffer unavailable interrupt enable
    #[inline(always)]
    pub fn rbuie(&mut self) -> RBUIE_W {
        RBUIE_W { w: self }
    }
    ///Bit 8 - Receive process stopped interrupt enable
    #[inline(always)]
    pub fn rpsie(&mut self) -> RPSIE_W {
        RPSIE_W { w: self }
    }
    ///Bit 9 - Receive watchdog timeout interrupt enable
    #[inline(always)]
    pub fn rwtie(&mut self) -> RWTIE_W {
        RWTIE_W { w: self }
    }
    ///Bit 10 - Early transmit interrupt enable
    #[inline(always)]
    pub fn etie(&mut self) -> ETIE_W {
        ETIE_W { w: self }
    }
    ///Bit 13 - Fatal bus error interrupt enable
    #[inline(always)]
    pub fn fbeie(&mut self) -> FBEIE_W {
        FBEIE_W { w: self }
    }
    ///Bit 14 - Early receive interrupt enable
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W {
        ERIE_W { w: self }
    }
    ///Bit 15 - Abnormal interrupt summary enable
    #[inline(always)]
    pub fn aise(&mut self) -> AISE_W {
        AISE_W { w: self }
    }
    ///Bit 16 - Normal interrupt summary enable
    #[inline(always)]
    pub fn nise(&mut self) -> NISE_W {
        NISE_W { w: self }
    }
}
