///Reader of register HUFFSYMB60
pub type R = crate::R<u32, super::HUFFSYMB60>;
///Writer for register HUFFSYMB60
pub type W = crate::W<u32, super::HUFFSYMB60>;
///Register HUFFSYMB60 `reset()`'s with value 0
impl crate::ResetValue for super::HUFFSYMB60 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HuffSymb_RAM`
pub type HUFFSYMB_RAM_R = crate::R<u32, u32>;
///Write proxy for field `HuffSymb_RAM`
pub struct HUFFSYMB_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFSYMB_RAM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - DHTSymb RAM
    #[inline(always)]
    pub fn huff_symb_ram(&self) -> HUFFSYMB_RAM_R {
        HUFFSYMB_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - DHTSymb RAM
    #[inline(always)]
    pub fn huff_symb_ram(&mut self) -> HUFFSYMB_RAM_W {
        HUFFSYMB_RAM_W { w: self }
    }
}
