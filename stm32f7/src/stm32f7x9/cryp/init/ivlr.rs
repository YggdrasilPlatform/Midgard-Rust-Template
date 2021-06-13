///Reader of register IVLR
pub type R = crate::R<u32, super::IVLR>;
///Writer for register IVLR
pub type W = crate::W<u32, super::IVLR>;
///Register IVLR `reset()`'s with value 0
impl crate::ResetValue for super::IVLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `IV`
pub type IV_R = crate::R<u32, u32>;
///Write proxy for field `IV`
pub struct IV_W<'a> {
    w: &'a mut W,
}
impl<'a> IV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - IV31
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - IV31
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W {
        IV_W { w: self }
    }
}
