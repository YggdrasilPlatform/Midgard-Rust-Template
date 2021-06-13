///Reader of register I2SPR
pub type R = crate::R<u32, super::I2SPR>;
///Writer for register I2SPR
pub type W = crate::W<u32, super::I2SPR>;
///Register I2SPR `reset()`'s with value 0x0a
impl crate::ResetValue for super::I2SPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
///Master clock output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKOE_A {
    ///0: Master clock output is disabled
    DISABLED = 0,
    ///1: Master clock output is enabled
    ENABLED = 1,
}
impl From<MCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MCKOE`
pub type MCKOE_R = crate::R<bool, MCKOE_A>;
impl MCKOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCKOE_A {
        match self.bits {
            false => MCKOE_A::DISABLED,
            true => MCKOE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOE_A::ENABLED
    }
}
///Write proxy for field `MCKOE`
pub struct MCKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCKOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Master clock output is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKOE_A::DISABLED)
    }
    ///Master clock output is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Odd factor for the prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODD_A {
    ///0: Real divider value is I2SDIV * 2
    EVEN = 0,
    ///1: Real divider value is (I2SDIV * 2) + 1
    ODD = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ODD`
pub type ODD_R = crate::R<bool, ODD_A>;
impl ODD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::EVEN,
            true => ODD_A::ODD,
        }
    }
    ///Checks if the value of the field is `EVEN`
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ODD_A::EVEN
    }
    ///Checks if the value of the field is `ODD`
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ODD_A::ODD
    }
}
///Write proxy for field `ODD`
pub struct ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> ODD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ODD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Real divider value is I2SDIV * 2
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(ODD_A::EVEN)
    }
    ///Real divider value is (I2SDIV * 2) + 1
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(ODD_A::ODD)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Reader of field `I2SDIV`
pub type I2SDIV_R = crate::R<u8, u8>;
///Write proxy for field `I2SDIV`
pub struct I2SDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bit 9 - Master clock output enable
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Odd factor for the prescaler
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 0:7 - I2S Linear prescaler
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bit 9 - Master clock output enable
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W {
        MCKOE_W { w: self }
    }
    ///Bit 8 - Odd factor for the prescaler
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W {
        ODD_W { w: self }
    }
    ///Bits 0:7 - I2S Linear prescaler
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W {
        I2SDIV_W { w: self }
    }
}
