///Reader of register DHR12R1
pub type R = crate::R<u32, super::DHR12R1>;
///Writer for register DHR12R1
pub type W = crate::W<u32, super::DHR12R1>;
///Register DHR12R1 `reset()`'s with value 0
impl crate::ResetValue for super::DHR12R1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DACC1DHR`
pub type DACC1DHR_R = crate::R<u16, u16>;
///Write proxy for field `DACC1DHR`
pub struct DACC1DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W { w: self }
    }
}
