///Reader of register OOR
pub type R = crate::R<u32, super::OOR>;
///Writer for register OOR
pub type W = crate::W<u32, super::OOR>;
///Register OOR `reset()`'s with value 0
impl crate::ResetValue for super::OOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LO`
pub type LO_R = crate::R<u16, u16>;
///Write proxy for field `LO`
pub struct LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    ///Bits 0:13 - Line Offset
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Line Offset
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W {
        LO_W { w: self }
    }
}
