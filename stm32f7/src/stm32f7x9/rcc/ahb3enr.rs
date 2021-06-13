///Reader of register AHB3ENR
pub type R = crate::R<u32, super::AHB3ENR>;
///Writer for register AHB3ENR
pub type W = crate::W<u32, super::AHB3ENR>;
///Register AHB3ENR `reset()`'s with value 0
impl crate::ResetValue for super::AHB3ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Flexible memory controller module clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCEN_A {
    ///0: The selected clock is disabled
    DISABLED = 0,
    ///1: The selected clock is enabled
    ENABLED = 1,
}
impl From<FMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FMCEN`
pub type FMCEN_R = crate::R<bool, FMCEN_A>;
impl FMCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMCEN_A {
        match self.bits {
            false => FMCEN_A::DISABLED,
            true => FMCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCEN_A::ENABLED
    }
}
///Write proxy for field `FMCEN`
pub struct FMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FMCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Quad SPI memory controller clock enable
pub type QSPIEN_A = FMCEN_A;
///Reader of field `QSPIEN`
pub type QSPIEN_R = crate::R<bool, FMCEN_A>;
///Write proxy for field `QSPIEN`
pub struct QSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: QSPIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    ///Bit 0 - Flexible memory controller module clock enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Quad SPI memory controller clock enable
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller module clock enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W { w: self }
    }
    ///Bit 1 - Quad SPI memory controller clock enable
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W {
        QSPIEN_W { w: self }
    }
}
