///Reader of register QMEM0_0
pub type R = crate::R<u32, super::QMEM0_0>;
///Writer for register QMEM0_0
pub type W = crate::W<u32, super::QMEM0_0>;
///Register QMEM0_0 `reset()`'s with value 0
impl crate::ResetValue for super::QMEM0_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `QMem_RAM`
pub type QMEM_RAM_R = crate::R<u32, u32>;
///Write proxy for field `QMem_RAM`
pub struct QMEM_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> QMEM_RAM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - QMem RAM
    #[inline(always)]
    pub fn qmem_ram(&self) -> QMEM_RAM_R {
        QMEM_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - QMem RAM
    #[inline(always)]
    pub fn qmem_ram(&mut self) -> QMEM_RAM_W {
        QMEM_RAM_W { w: self }
    }
}
