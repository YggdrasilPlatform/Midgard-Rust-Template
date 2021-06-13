///Writer for register KLR
pub type W = crate::W<u32, super::KLR>;
///Register KLR `reset()`'s with value 0
impl crate::ResetValue for super::KLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `b2`
pub struct B2_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:31 - b224
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W {
        B2_W { w: self }
    }
}
