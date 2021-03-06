///Reader of register HUFFENC_AC1_68
pub type R = crate::R<u32, super::HUFFENC_AC1_68>;
///Writer for register HUFFENC_AC1_68
pub type W = crate::W<u32, super::HUFFENC_AC1_68>;
///Register HUFFENC_AC1_68 `reset()`'s with value 0
impl crate::ResetValue for super::HUFFENC_AC1_68 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DHTMem_RAM`
pub type DHTMEM_RAM_R = crate::R<u32, u32>;
///Write proxy for field `DHTMem_RAM`
pub struct DHTMEM_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DHTMEM_RAM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - DHTMem RAM
    #[inline(always)]
    pub fn dhtmem_ram(&self) -> DHTMEM_RAM_R {
        DHTMEM_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - DHTMem RAM
    #[inline(always)]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W {
        DHTMEM_RAM_W { w: self }
    }
}
