///Reader of register DMATPDR
pub type R = crate::R<u32, super::DMATPDR>;
///Writer for register DMATPDR
pub type W = crate::W<u32, super::DMATPDR>;
///Register DMATPDR `reset()`'s with value 0
impl crate::ResetValue for super::DMATPDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Transmit poll demand
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TPD_A {
    ///0: Poll the transmit descriptor list
    POLL = 0,
}
impl From<TPD_A> for u32 {
    #[inline(always)]
    fn from(variant: TPD_A) -> Self {
        variant as _
    }
}
///Reader of field `TPD`
pub type TPD_R = crate::R<u32, TPD_A>;
impl TPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TPD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TPD_A::POLL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `POLL`
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        *self == TPD_A::POLL
    }
}
///Write proxy for field `TPD`
pub struct TPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TPD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Poll the transmit descriptor list
    #[inline(always)]
    pub fn poll(self) -> &'a mut W {
        self.variant(TPD_A::POLL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Transmit poll demand
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Transmit poll demand
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W {
        TPD_W { w: self }
    }
}
