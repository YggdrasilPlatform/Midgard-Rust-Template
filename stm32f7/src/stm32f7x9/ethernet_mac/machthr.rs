///Reader of register MACHTHR
pub type R = crate::R<u32, super::MACHTHR>;
///Writer for register MACHTHR
pub type W = crate::W<u32, super::MACHTHR>;
///Register MACHTHR `reset()`'s with value 0
impl crate::ResetValue for super::MACHTHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HTH`
pub type HTH_R = crate::R<u32, u32>;
///Write proxy for field `HTH`
pub struct HTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HTH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Upper 32 bits of hash table
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Upper 32 bits of hash table
    #[inline(always)]
    pub fn hth(&mut self) -> HTH_W {
        HTH_W { w: self }
    }
}
