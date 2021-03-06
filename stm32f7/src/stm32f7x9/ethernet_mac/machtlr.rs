///Reader of register MACHTLR
pub type R = crate::R<u32, super::MACHTLR>;
///Writer for register MACHTLR
pub type W = crate::W<u32, super::MACHTLR>;
///Register MACHTLR `reset()`'s with value 0
impl crate::ResetValue for super::MACHTLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HTL`
pub type HTL_R = crate::R<u32, u32>;
///Write proxy for field `HTL`
pub struct HTL_W<'a> {
    w: &'a mut W,
}
impl<'a> HTL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Lower 32 bits of hash table
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Lower 32 bits of hash table
    #[inline(always)]
    pub fn htl(&mut self) -> HTL_W {
        HTL_W { w: self }
    }
}
