///Reader of register DMARPDR
pub type R = crate::R<u32, super::DMARPDR>;
///Writer for register DMARPDR
pub type W = crate::W<u32, super::DMARPDR>;
///Register DMARPDR `reset()`'s with value 0
impl crate::ResetValue for super::DMARPDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Receive poll demand
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RPD_A {
    ///0: Poll the receive descriptor list
    POLL = 0,
}
impl From<RPD_A> for u32 {
    #[inline(always)]
    fn from(variant: RPD_A) -> Self {
        variant as _
    }
}
///Reader of field `RPD`
pub type RPD_R = crate::R<u32, RPD_A>;
impl RPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RPD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RPD_A::POLL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `POLL`
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        *self == RPD_A::POLL
    }
}
///Write proxy for field `RPD`
pub struct RPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Poll the receive descriptor list
    #[inline(always)]
    pub fn poll(self) -> &'a mut W {
        self.variant(RPD_A::POLL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Receive poll demand
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Receive poll demand
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W {
        RPD_W { w: self }
    }
}
