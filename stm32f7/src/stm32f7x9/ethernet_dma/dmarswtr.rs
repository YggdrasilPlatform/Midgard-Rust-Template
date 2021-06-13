///Reader of register DMARSWTR
pub type R = crate::R<u32, super::DMARSWTR>;
///Writer for register DMARSWTR
pub type W = crate::W<u32, super::DMARSWTR>;
///Register DMARSWTR `reset()`'s with value 0
impl crate::ResetValue for super::DMARSWTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RSWTC`
pub type RSWTC_R = crate::R<u8, u8>;
///Write proxy for field `RSWTC`
pub struct RSWTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSWTC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Receive status watchdog timer count
    #[inline(always)]
    pub fn rswtc(&self) -> RSWTC_R {
        RSWTC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Receive status watchdog timer count
    #[inline(always)]
    pub fn rswtc(&mut self) -> RSWTC_W {
        RSWTC_W { w: self }
    }
}
