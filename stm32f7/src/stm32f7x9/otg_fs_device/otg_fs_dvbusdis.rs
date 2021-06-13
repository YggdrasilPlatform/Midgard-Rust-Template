///Reader of register OTG_FS_DVBUSDIS
pub type R = crate::R<u32, super::OTG_FS_DVBUSDIS>;
///Writer for register OTG_FS_DVBUSDIS
pub type W = crate::W<u32, super::OTG_FS_DVBUSDIS>;
///Register OTG_FS_DVBUSDIS `reset()`'s with value 0x17d7
impl crate::ResetValue for super::OTG_FS_DVBUSDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x17d7
    }
}
///Reader of field `VBUSDT`
pub type VBUSDT_R = crate::R<u16, u16>;
///Write proxy for field `VBUSDT`
pub struct VBUSDT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSDT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Device VBUS discharge time
    #[inline(always)]
    pub fn vbusdt(&self) -> VBUSDT_R {
        VBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Device VBUS discharge time
    #[inline(always)]
    pub fn vbusdt(&mut self) -> VBUSDT_W {
        VBUSDT_W { w: self }
    }
}
