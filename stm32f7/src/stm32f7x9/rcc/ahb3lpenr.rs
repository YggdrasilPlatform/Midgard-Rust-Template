///Reader of register AHB3LPENR
pub type R = crate::R<u32, super::AHB3LPENR>;
///Writer for register AHB3LPENR
pub type W = crate::W<u32, super::AHB3LPENR>;
///Register AHB3LPENR `reset()`'s with value 0x01
impl crate::ResetValue for super::AHB3LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
///Flexible memory controller module clock enable during Sleep mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCLPEN_A {
    ///0: Selected module is disabled during Sleep mode
    DISABLEDINSLEEP = 0,
    ///1: Selected module is enabled during Sleep mode
    ENABLEDINSLEEP = 1,
}
impl From<FMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FMCLPEN`
pub type FMCLPEN_R = crate::R<bool, FMCLPEN_A>;
impl FMCLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMCLPEN_A {
        match self.bits {
            false => FMCLPEN_A::DISABLEDINSLEEP,
            true => FMCLPEN_A::ENABLEDINSLEEP,
        }
    }
    ///Checks if the value of the field is `DISABLEDINSLEEP`
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FMCLPEN_A::DISABLEDINSLEEP
    }
    ///Checks if the value of the field is `ENABLEDINSLEEP`
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FMCLPEN_A::ENABLEDINSLEEP
    }
}
///Write proxy for field `FMCLPEN`
pub struct FMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCLPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FMCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::ENABLEDINSLEEP)
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
///Quand SPI memory controller clock enable during Sleep mode
pub type QSPILPEN_A = FMCLPEN_A;
///Reader of field `QSPILPEN`
pub type QSPILPEN_R = crate::R<bool, FMCLPEN_A>;
///Write proxy for field `QSPILPEN`
pub struct QSPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPILPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: QSPILPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::ENABLEDINSLEEP)
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
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Quand SPI memory controller clock enable during Sleep mode
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
    ///Bit 1 - Quand SPI memory controller clock enable during Sleep mode
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W {
        QSPILPEN_W { w: self }
    }
}
