///Reader of register HTR
pub type R = crate::R<u32, super::HTR>;
///Writer for register HTR
pub type W = crate::W<u32, super::HTR>;
///Register HTR `reset()`'s with value 0x0fff
impl crate::ResetValue for super::HTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
///Reader of field `HT`
pub type HT_R = crate::R<u16, u16>;
///Write proxy for field `HT`
pub struct HT_W<'a> {
    w: &'a mut W,
}
impl<'a> HT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - Analog watchdog higher threshold
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog higher threshold
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W {
        HT_W { w: self }
    }
}
