///Reader of register DSI_TCCR3
pub type R = crate::R<u32, super::DSI_TCCR3>;
///Writer for register DSI_TCCR3
pub type W = crate::W<u32, super::DSI_TCCR3>;
///Register DSI_TCCR3 `reset()`'s with value 0
impl crate::ResetValue for super::DSI_TCCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HSWR_TOCNT`
pub type HSWR_TOCNT_R = crate::R<u16, u16>;
///Write proxy for field `HSWR_TOCNT`
pub struct HSWR_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSWR_TOCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `PM`
pub type PM_R = crate::R<bool, bool>;
///Write proxy for field `PM`
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
impl R {
    ///Bits 0:15 - High-Speed Write Timeout Counter
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 24 - Presp mode
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - High-Speed Write Timeout Counter
    #[inline(always)]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W {
        HSWR_TOCNT_W { w: self }
    }
    ///Bit 24 - Presp mode
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
}
