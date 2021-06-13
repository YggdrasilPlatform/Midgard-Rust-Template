///Reader of register HUFFBASE23
pub type R = crate::R<u32, super::HUFFBASE23>;
///Writer for register HUFFBASE23
pub type W = crate::W<u32, super::HUFFBASE23>;
///Register HUFFBASE23 `reset()`'s with value 0
impl crate::ResetValue for super::HUFFBASE23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HuffBase_RAM_0`
pub type HUFFBASE_RAM_0_R = crate::R<u16, u16>;
///Write proxy for field `HuffBase_RAM_0`
pub struct HUFFBASE_RAM_0_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFBASE_RAM_0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
///Reader of field `HuffBase_RAM_1`
pub type HUFFBASE_RAM_1_R = crate::R<u16, u16>;
///Write proxy for field `HuffBase_RAM_1`
pub struct HUFFBASE_RAM_1_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFBASE_RAM_1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:8 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_0(&self) -> HUFFBASE_RAM_0_R {
        HUFFBASE_RAM_0_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_1(&self) -> HUFFBASE_RAM_1_R {
        HUFFBASE_RAM_1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:8 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_0(&mut self) -> HUFFBASE_RAM_0_W {
        HUFFBASE_RAM_0_W { w: self }
    }
    ///Bits 16:24 - HuffBase RAM
    #[inline(always)]
    pub fn huff_base_ram_1(&mut self) -> HUFFBASE_RAM_1_W {
        HUFFBASE_RAM_1_W { w: self }
    }
}
