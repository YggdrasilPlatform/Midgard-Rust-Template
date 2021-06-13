///Reader of register APB2ENR
pub type R = crate::R<u32, super::APB2ENR>;
///Writer for register APB2ENR
pub type W = crate::W<u32, super::APB2ENR>;
///Register APB2ENR `reset()`'s with value 0
impl crate::ResetValue for super::APB2ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///TIM1 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1EN_A {
    ///0: The selected clock is disabled
    DISABLED = 0,
    ///1: The selected clock is enabled
    ENABLED = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TIM1EN`
pub type TIM1EN_R = crate::R<bool, TIM1EN_A>;
impl TIM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::DISABLED,
            true => TIM1EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN_A::ENABLED
    }
}
///Write proxy for field `TIM1EN`
pub struct TIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///TIM8 clock enable
pub type TIM8EN_A = TIM1EN_A;
///Reader of field `TIM8EN`
pub type TIM8EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `TIM8EN`
pub struct TIM8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM8EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///USART1 clock enable
pub type USART1EN_A = TIM1EN_A;
///Reader of field `USART1EN`
pub type USART1EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `USART1EN`
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///USART6 clock enable
pub type USART6EN_A = TIM1EN_A;
///Reader of field `USART6EN`
pub type USART6EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `USART6EN`
pub struct USART6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART6EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///ADC1 clock enable
pub type ADC1EN_A = TIM1EN_A;
///Reader of field `ADC1EN`
pub type ADC1EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `ADC1EN`
pub struct ADC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADC1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///ADC2 clock enable
pub type ADC2EN_A = TIM1EN_A;
///Reader of field `ADC2EN`
pub type ADC2EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `ADC2EN`
pub struct ADC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADC2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///ADC3 clock enable
pub type ADC3EN_A = TIM1EN_A;
///Reader of field `ADC3EN`
pub type ADC3EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `ADC3EN`
pub struct ADC3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADC3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///SPI1 clock enable
pub type SPI1EN_A = TIM1EN_A;
///Reader of field `SPI1EN`
pub type SPI1EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SPI1EN`
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///SPI4 clock enable
pub type SPI4EN_A = TIM1EN_A;
///Reader of field `SPI4EN`
pub type SPI4EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SPI4EN`
pub struct SPI4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI4EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///System configuration controller clock enable
pub type SYSCFGEN_A = TIM1EN_A;
///Reader of field `SYSCFGEN`
pub type SYSCFGEN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SYSCFGEN`
pub struct SYSCFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///TIM9 clock enable
pub type TIM9EN_A = TIM1EN_A;
///Reader of field `TIM9EN`
pub type TIM9EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `TIM9EN`
pub struct TIM9EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM9EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///TIM10 clock enable
pub type TIM10EN_A = TIM1EN_A;
///Reader of field `TIM10EN`
pub type TIM10EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `TIM10EN`
pub struct TIM10EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM10EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM10EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///TIM11 clock enable
pub type TIM11EN_A = TIM1EN_A;
///Reader of field `TIM11EN`
pub type TIM11EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `TIM11EN`
pub struct TIM11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM11EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM11EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///SPI5 clock enable
pub type SPI5EN_A = TIM1EN_A;
///Reader of field `SPI5EN`
pub type SPI5EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SPI5EN`
pub struct SPI5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI5EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///SPI6 clock enable
pub type SPI6EN_A = TIM1EN_A;
///Reader of field `SPI6EN`
pub type SPI6EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SPI6EN`
pub struct SPI6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI6EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///SAI1 clock enable
pub type SAI1EN_A = TIM1EN_A;
///Reader of field `SAI1EN`
pub type SAI1EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SAI1EN`
pub struct SAI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///LTDC clock enable
pub type LTDCEN_A = TIM1EN_A;
///Reader of field `LTDCEN`
pub type LTDCEN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `LTDCEN`
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LTDCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///SAI2 clock enable
pub type SAI2EN_A = TIM1EN_A;
///Reader of field `SAI2EN`
pub type SAI2EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SAI2EN`
pub struct SAI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
///SDMMC1 clock enable
pub type SDMMC1EN_A = TIM1EN_A;
///Reader of field `SDMMC1EN`
pub type SDMMC1EN_R = crate::R<bool, TIM1EN_A>;
///Write proxy for field `SDMMC1EN`
pub struct SDMMC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
    ///Bit 0 - TIM1 clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM8 clock enable
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - USART6 clock enable
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 8 - ADC1 clock enable
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - ADC2 clock enable
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - ADC3 clock enable
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - SPI4 clock enable
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - System configuration controller clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - TIM9 clock enable
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - TIM10 clock enable
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - TIM11 clock enable
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - SPI5 clock enable
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - SPI6 clock enable
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 26 - LTDC clock enable
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 23 - SAI2 clock enable
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 11 - SDMMC1 clock enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM1 clock enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W { w: self }
    }
    ///Bit 1 - TIM8 clock enable
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W {
        TIM8EN_W { w: self }
    }
    ///Bit 4 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    ///Bit 5 - USART6 clock enable
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W {
        USART6EN_W { w: self }
    }
    ///Bit 8 - ADC1 clock enable
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W {
        ADC1EN_W { w: self }
    }
    ///Bit 9 - ADC2 clock enable
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W {
        ADC2EN_W { w: self }
    }
    ///Bit 10 - ADC3 clock enable
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W {
        ADC3EN_W { w: self }
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    ///Bit 13 - SPI4 clock enable
    #[inline(always)]
    pub fn spi4en(&mut self) -> SPI4EN_W {
        SPI4EN_W { w: self }
    }
    ///Bit 14 - System configuration controller clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W {
        SYSCFGEN_W { w: self }
    }
    ///Bit 16 - TIM9 clock enable
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W {
        TIM9EN_W { w: self }
    }
    ///Bit 17 - TIM10 clock enable
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W {
        TIM10EN_W { w: self }
    }
    ///Bit 18 - TIM11 clock enable
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W {
        TIM11EN_W { w: self }
    }
    ///Bit 20 - SPI5 clock enable
    #[inline(always)]
    pub fn spi5en(&mut self) -> SPI5EN_W {
        SPI5EN_W { w: self }
    }
    ///Bit 21 - SPI6 clock enable
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W {
        SPI6EN_W { w: self }
    }
    ///Bit 22 - SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W {
        SAI1EN_W { w: self }
    }
    ///Bit 26 - LTDC clock enable
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
    }
    ///Bit 23 - SAI2 clock enable
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W {
        SAI2EN_W { w: self }
    }
    ///Bit 11 - SDMMC1 clock enable
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W {
        SDMMC1EN_W { w: self }
    }
}
