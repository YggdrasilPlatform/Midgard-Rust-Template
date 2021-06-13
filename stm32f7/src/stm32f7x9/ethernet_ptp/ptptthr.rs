///Reader of register PTPTTHR
pub type R = crate::R<u32, super::PTPTTHR>;
///Writer for register PTPTTHR
pub type W = crate::W<u32, super::PTPTTHR>;
///Register PTPTTHR `reset()`'s with value 0
impl crate::ResetValue for super::PTPTTHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TTSH`
pub type TTSH_R = crate::R<u32, u32>;
///Write proxy for field `TTSH`
pub struct TTSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - 0
    #[inline(always)]
    pub fn ttsh(&self) -> TTSH_R {
        TTSH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - 0
    #[inline(always)]
    pub fn ttsh(&mut self) -> TTSH_W {
        TTSH_W { w: self }
    }
}
