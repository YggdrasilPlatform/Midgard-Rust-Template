///Reader of register APB2LPENR
pub type R = crate::R<u32, super::APB2LPENR>;
///Writer for register APB2LPENR
pub type W = crate::W<u32, super::APB2LPENR>;
///Register APB2LPENR `reset()`'s with value 0x0007_5f33
impl crate::ResetValue for super::APB2LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_5f33
    }
}
///TIM1 clock enable during Sleep mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1LPEN_A {
    ///0: Selected module is disabled during Sleep mode
    DISABLEDINSLEEP = 0,
    ///1: Selected module is enabled during Sleep mode
    ENABLEDINSLEEP = 1,
}
impl From<TIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TIM1LPEN`
pub type TIM1LPEN_R = crate::R<bool, TIM1LPEN_A>;
impl TIM1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM1LPEN_A {
        match self.bits {
            false => TIM1LPEN_A::DISABLEDINSLEEP,
            true => TIM1LPEN_A::ENABLEDINSLEEP,
        }
    }
    ///Checks if the value of the field is `DISABLEDINSLEEP`
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN_A::DISABLEDINSLEEP
    }
    ///Checks if the value of the field is `ENABLEDINSLEEP`
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN_A::ENABLEDINSLEEP
    }
}
///Write proxy for field `TIM1LPEN`
pub struct TIM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
///TIM8 clock enable during Sleep mode
pub type TIM8LPEN_A = TIM1LPEN_A;
///Reader of field `TIM8LPEN`
pub type TIM8LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `TIM8LPEN`
pub struct TIM8LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM8LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
///USART1 clock enable during Sleep mode
pub type USART1LPEN_A = TIM1LPEN_A;
///Reader of field `USART1LPEN`
pub type USART1LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `USART1LPEN`
pub struct USART1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///USART6 clock enable during Sleep mode
pub type USART6LPEN_A = TIM1LPEN_A;
///Reader of field `USART6LPEN`
pub type USART6LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `USART6LPEN`
pub struct USART6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART6LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///ADC1 clock enable during Sleep mode
pub type ADC1LPEN_A = TIM1LPEN_A;
///Reader of field `ADC1LPEN`
pub type ADC1LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `ADC1LPEN`
pub struct ADC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADC1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
///ADC2 clock enable during Sleep mode
pub type ADC2LPEN_A = TIM1LPEN_A;
///Reader of field `ADC2LPEN`
pub type ADC2LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `ADC2LPEN`
pub struct ADC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADC2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
///ADC 3 clock enable during Sleep mode
pub type ADC3LPEN_A = TIM1LPEN_A;
///Reader of field `ADC3LPEN`
pub type ADC3LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `ADC3LPEN`
pub struct ADC3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADC3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///SPI 1 clock enable during Sleep mode
pub type SPI1LPEN_A = TIM1LPEN_A;
///Reader of field `SPI1LPEN`
pub type SPI1LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SPI1LPEN`
pub struct SPI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///SPI 4 clock enable during Sleep mode
pub type SPI4LPEN_A = TIM1LPEN_A;
///Reader of field `SPI4LPEN`
pub type SPI4LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SPI4LPEN`
pub struct SPI4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI4LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///System configuration controller clock enable during Sleep mode
pub type SYSCFGLPEN_A = TIM1LPEN_A;
///Reader of field `SYSCFGLPEN`
pub type SYSCFGLPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SYSCFGLPEN`
pub struct SYSCFGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGLPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///TIM9 clock enable during sleep mode
pub type TIM9LPEN_A = TIM1LPEN_A;
///Reader of field `TIM9LPEN`
pub type TIM9LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `TIM9LPEN`
pub struct TIM9LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM9LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
///TIM10 clock enable during Sleep mode
pub type TIM10LPEN_A = TIM1LPEN_A;
///Reader of field `TIM10LPEN`
pub type TIM10LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `TIM10LPEN`
pub struct TIM10LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM10LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM10LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///TIM11 clock enable during Sleep mode
pub type TIM11LPEN_A = TIM1LPEN_A;
///Reader of field `TIM11LPEN`
pub type TIM11LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `TIM11LPEN`
pub struct TIM11LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM11LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM11LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///SPI 5 clock enable during Sleep mode
pub type SPI5LPEN_A = TIM1LPEN_A;
///Reader of field `SPI5LPEN`
pub type SPI5LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SPI5LPEN`
pub struct SPI5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI5LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///SPI 6 clock enable during Sleep mode
pub type SPI6LPEN_A = TIM1LPEN_A;
///Reader of field `SPI6LPEN`
pub type SPI6LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SPI6LPEN`
pub struct SPI6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI6LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///SAI1 clock enable during sleep mode
pub type SAI1LPEN_A = TIM1LPEN_A;
///Reader of field `SAI1LPEN`
pub type SAI1LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SAI1LPEN`
pub struct SAI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
///LTDC clock enable during sleep mode
pub type LTDCLPEN_A = TIM1LPEN_A;
///Reader of field `LTDCLPEN`
pub type LTDCLPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `LTDCLPEN`
pub struct LTDCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCLPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LTDCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///SAI2 clock enable during sleep mode
pub type SAI2LPEN_A = TIM1LPEN_A;
///Reader of field `SAI2LPEN`
pub type SAI2LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SAI2LPEN`
pub struct SAI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///SDMMC1 clock enable during Sleep mode
pub type SDMMC1LPEN_A = TIM1LPEN_A;
///Reader of field `SDMMC1LPEN`
pub type SDMMC1LPEN_R = crate::R<bool, TIM1LPEN_A>;
///Write proxy for field `SDMMC1LPEN`
pub struct SDMMC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1LPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLEDINSLEEP)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    ///Bit 0 - TIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM8 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - USART6 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 8 - ADC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - ADC2 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc2lpen(&self) -> ADC2LPEN_R {
        ADC2LPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - ADC 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc3lpen(&self) -> ADC3LPEN_R {
        ADC3LPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 12 - SPI 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - SPI 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - TIM9 clock enable during sleep mode
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - TIM10 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim10lpen(&self) -> TIM10LPEN_R {
        TIM10LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - TIM11 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - SPI 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - SPI 6 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - SAI1 clock enable during sleep mode
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 26 - LTDC clock enable during sleep mode
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 23 - SAI2 clock enable during sleep mode
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 11 - SDMMC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W {
        TIM1LPEN_W { w: self }
    }
    ///Bit 1 - TIM8 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W {
        TIM8LPEN_W { w: self }
    }
    ///Bit 4 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W {
        USART1LPEN_W { w: self }
    }
    ///Bit 5 - USART6 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W {
        USART6LPEN_W { w: self }
    }
    ///Bit 8 - ADC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W {
        ADC1LPEN_W { w: self }
    }
    ///Bit 9 - ADC2 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc2lpen(&mut self) -> ADC2LPEN_W {
        ADC2LPEN_W { w: self }
    }
    ///Bit 10 - ADC 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc3lpen(&mut self) -> ADC3LPEN_W {
        ADC3LPEN_W { w: self }
    }
    ///Bit 12 - SPI 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W {
        SPI1LPEN_W { w: self }
    }
    ///Bit 13 - SPI 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W {
        SPI4LPEN_W { w: self }
    }
    ///Bit 14 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W {
        SYSCFGLPEN_W { w: self }
    }
    ///Bit 16 - TIM9 clock enable during sleep mode
    #[inline(always)]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W {
        TIM9LPEN_W { w: self }
    }
    ///Bit 17 - TIM10 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim10lpen(&mut self) -> TIM10LPEN_W {
        TIM10LPEN_W { w: self }
    }
    ///Bit 18 - TIM11 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W {
        TIM11LPEN_W { w: self }
    }
    ///Bit 20 - SPI 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W {
        SPI5LPEN_W { w: self }
    }
    ///Bit 21 - SPI 6 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W {
        SPI6LPEN_W { w: self }
    }
    ///Bit 22 - SAI1 clock enable during sleep mode
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W {
        SAI1LPEN_W { w: self }
    }
    ///Bit 26 - LTDC clock enable during sleep mode
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W {
        LTDCLPEN_W { w: self }
    }
    ///Bit 23 - SAI2 clock enable during sleep mode
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W {
        SAI2LPEN_W { w: self }
    }
    ///Bit 11 - SDMMC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W {
        SDMMC1LPEN_W { w: self }
    }
}
