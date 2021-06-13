///Reader of register CSR%s
pub type R = crate::R<u32, super::CSR>;
///Writer for register CSR%s
pub type W = crate::W<u32, super::CSR>;
///Register CSR%s `reset()`'s with value 0
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR`
pub type CSR_R = crate::R<u32, u32>;
///Write proxy for field `CSR`
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
}
