///Reader of register AHBSCR
pub type R = crate::R<u32, super::AHBSCR>;
///Writer for register AHBSCR
pub type W = crate::W<u32, super::AHBSCR>;
///Register AHBSCR `reset()`'s with value 0
impl crate::ResetValue for super::AHBSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CTL`
pub type CTL_R = crate::R<u8, u8>;
///Write proxy for field `CTL`
pub struct CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `TPRI`
pub type TPRI_R = crate::R<u16, u16>;
///Write proxy for field `TPRI`
pub struct TPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> TPRI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | (((value as u32) & 0x01ff) << 2);
        self.w
    }
}
///Reader of field `INITCOUNT`
pub type INITCOUNT_R = crate::R<u8, u8>;
///Write proxy for field `INITCOUNT`
pub struct INITCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INITCOUNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
impl R {
    ///Bits 0:1 - CTL
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:10 - TPRI
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bits 11:15 - INITCOUNT
    #[inline(always)]
    pub fn initcount(&self) -> INITCOUNT_R {
        INITCOUNT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:1 - CTL
    #[inline(always)]
    pub fn ctl(&mut self) -> CTL_W {
        CTL_W { w: self }
    }
    ///Bits 2:10 - TPRI
    #[inline(always)]
    pub fn tpri(&mut self) -> TPRI_W {
        TPRI_W { w: self }
    }
    ///Bits 11:15 - INITCOUNT
    #[inline(always)]
    pub fn initcount(&mut self) -> INITCOUNT_W {
        INITCOUNT_W { w: self }
    }
}
