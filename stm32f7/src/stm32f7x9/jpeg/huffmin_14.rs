///Reader of register HUFFMIN_14
pub type R = crate::R<u32, super::HUFFMIN_14>;
///Writer for register HUFFMIN_14
pub type W = crate::W<u32, super::HUFFMIN_14>;
///Register HUFFMIN_14 `reset()`'s with value 0
impl crate::ResetValue for super::HUFFMIN_14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HuffMin_RAM`
pub type HUFFMIN_RAM_R = crate::R<u32, u32>;
///Write proxy for field `HuffMin_RAM`
pub struct HUFFMIN_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFMIN_RAM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - HuffMin RAM
    #[inline(always)]
    pub fn huff_min_ram(&self) -> HUFFMIN_RAM_R {
        HUFFMIN_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - HuffMin RAM
    #[inline(always)]
    pub fn huff_min_ram(&mut self) -> HUFFMIN_RAM_W {
        HUFFMIN_RAM_W { w: self }
    }
}
