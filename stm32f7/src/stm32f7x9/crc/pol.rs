///Reader of register POL
pub type R = crate::R<u32, super::POL>;
///Writer for register POL
pub type W = crate::W<u32, super::POL>;
///Register POL `reset()`'s with value 0
impl crate::ResetValue for super::POL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `POL`
pub type POL_R = crate::R<u32, u32>;
///Write proxy for field `POL`
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Programmable polynomial
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Programmable polynomial
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
}
