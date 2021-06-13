///Reader of register MACVLANTR
pub type R = crate::R<u32, super::MACVLANTR>;
///Writer for register MACVLANTR
pub type W = crate::W<u32, super::MACVLANTR>;
///Register MACVLANTR `reset()`'s with value 0
impl crate::ResetValue for super::MACVLANTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `VLANTI`
pub type VLANTI_R = crate::R<u16, u16>;
///Write proxy for field `VLANTI`
pub struct VLANTI_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///12-bit VLAN tag comparison
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLANTC_A {
    ///0: Full 16 bit VLAN identifiers are used for comparison and filtering
    VLANTC16 = 0,
    ///1: 12 bit VLAN identifies are used for comparison and filtering
    VLANTC12 = 1,
}
impl From<VLANTC_A> for bool {
    #[inline(always)]
    fn from(variant: VLANTC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `VLANTC`
pub type VLANTC_R = crate::R<bool, VLANTC_A>;
impl VLANTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VLANTC_A {
        match self.bits {
            false => VLANTC_A::VLANTC16,
            true => VLANTC_A::VLANTC12,
        }
    }
    ///Checks if the value of the field is `VLANTC16`
    #[inline(always)]
    pub fn is_vlantc16(&self) -> bool {
        *self == VLANTC_A::VLANTC16
    }
    ///Checks if the value of the field is `VLANTC12`
    #[inline(always)]
    pub fn is_vlantc12(&self) -> bool {
        *self == VLANTC_A::VLANTC12
    }
}
///Write proxy for field `VLANTC`
pub struct VLANTC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VLANTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Full 16 bit VLAN identifiers are used for comparison and filtering
    #[inline(always)]
    pub fn vlantc16(self) -> &'a mut W {
        self.variant(VLANTC_A::VLANTC16)
    }
    ///12 bit VLAN identifies are used for comparison and filtering
    #[inline(always)]
    pub fn vlantc12(self) -> &'a mut W {
        self.variant(VLANTC_A::VLANTC12)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - VLAN tag identifier (for receive frames)
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - 12-bit VLAN tag comparison
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - VLAN tag identifier (for receive frames)
    #[inline(always)]
    pub fn vlanti(&mut self) -> VLANTI_W {
        VLANTI_W { w: self }
    }
    ///Bit 16 - 12-bit VLAN tag comparison
    #[inline(always)]
    pub fn vlantc(&mut self) -> VLANTC_W {
        VLANTC_W { w: self }
    }
}
