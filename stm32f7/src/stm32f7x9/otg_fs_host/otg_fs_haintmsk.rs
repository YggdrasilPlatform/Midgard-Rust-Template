///Reader of register OTG_FS_HAINTMSK
pub type R = crate::R<u32, super::OTG_FS_HAINTMSK>;
///Writer for register OTG_FS_HAINTMSK
pub type W = crate::W<u32, super::OTG_FS_HAINTMSK>;
///Register OTG_FS_HAINTMSK `reset()`'s with value 0
impl crate::ResetValue for super::OTG_FS_HAINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HAINTM`
pub type HAINTM_R = crate::R<u16, u16>;
///Write proxy for field `HAINTM`
pub struct HAINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> HAINTM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    pub fn haintm(&mut self) -> HAINTM_W {
        HAINTM_W { w: self }
    }
}
