///Reader of register BKP%sR
pub type R = crate::R<u32, super::BKPR>;
///Writer for register BKP%sR
pub type W = crate::W<u32, super::BKPR>;
///Register BKP%sR `reset()`'s with value 0
impl crate::ResetValue for super::BKPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `BKP`
pub type BKP_R = crate::R<u32, u32>;
///Write proxy for field `BKP`
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
}
