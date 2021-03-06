///Reader of register CVR
pub type R = crate::R<u32, super::CVR>;
///Writer for register CVR
pub type W = crate::W<u32, super::CVR>;
///Register CVR `reset()`'s with value 0
impl crate::ResetValue for super::CVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CURRENT`
pub type CURRENT_R = crate::R<u32, u32>;
///Write proxy for field `CURRENT`
pub struct CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W {
        CURRENT_W { w: self }
    }
}
