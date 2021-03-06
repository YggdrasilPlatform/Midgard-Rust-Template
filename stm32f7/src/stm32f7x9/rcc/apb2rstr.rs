///Reader of register APB2RSTR
pub type R = crate::R<u32, super::APB2RSTR>;
///Writer for register APB2RSTR
pub type W = crate::W<u32, super::APB2RSTR>;
///Register APB2RSTR `reset()`'s with value 0
impl crate::ResetValue for super::APB2RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///TIM1 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1RST_A {
    ///1: Reset the selected module
    RESET = 1,
}
impl From<TIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TIM1RST`
pub type TIM1RST_R = crate::R<bool, TIM1RST_A>;
impl TIM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TIM1RST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TIM1RST_A::RESET),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST_A::RESET
    }
}
///Write proxy for field `TIM1RST`
pub struct TIM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///TIM8 reset
pub type TIM8RST_A = TIM1RST_A;
///Reader of field `TIM8RST`
pub type TIM8RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `TIM8RST`
pub struct TIM8RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM8RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///USART1 reset
pub type USART1RST_A = TIM1RST_A;
///Reader of field `USART1RST`
pub type USART1RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `USART1RST`
pub struct USART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///USART6 reset
pub type USART6RST_A = TIM1RST_A;
///Reader of field `USART6RST`
pub type USART6RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `USART6RST`
pub struct USART6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART6RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///ADC interface reset (common to all ADCs)
pub type ADCRST_A = TIM1RST_A;
///Reader of field `ADCRST`
pub type ADCRST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `ADCRST`
pub struct ADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///SPI 1 reset
pub type SPI1RST_A = TIM1RST_A;
///Reader of field `SPI1RST`
pub type SPI1RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SPI1RST`
pub struct SPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///SPI4 reset
pub type SPI4RST_A = TIM1RST_A;
///Reader of field `SPI4RST`
pub type SPI4RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SPI4RST`
pub struct SPI4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///System configuration controller reset
pub type SYSCFGRST_A = TIM1RST_A;
///Reader of field `SYSCFGRST`
pub type SYSCFGRST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SYSCFGRST`
pub struct SYSCFGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///TIM9 reset
pub type TIM9RST_A = TIM1RST_A;
///Reader of field `TIM9RST`
pub type TIM9RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `TIM9RST`
pub struct TIM9RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM9RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///TIM10 reset
pub type TIM10RST_A = TIM1RST_A;
///Reader of field `TIM10RST`
pub type TIM10RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `TIM10RST`
pub struct TIM10RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM10RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM10RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///TIM11 reset
pub type TIM11RST_A = TIM1RST_A;
///Reader of field `TIM11RST`
pub type TIM11RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `TIM11RST`
pub struct TIM11RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM11RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM11RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///SPI5 reset
pub type SPI5RST_A = TIM1RST_A;
///Reader of field `SPI5RST`
pub type SPI5RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SPI5RST`
pub struct SPI5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI5RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///SPI6 reset
pub type SPI6RST_A = TIM1RST_A;
///Reader of field `SPI6RST`
pub type SPI6RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SPI6RST`
pub struct SPI6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI6RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///SAI1 reset
pub type SAI1RST_A = TIM1RST_A;
///Reader of field `SAI1RST`
pub type SAI1RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SAI1RST`
pub struct SAI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///LTDC reset
pub type LTDCRST_A = TIM1RST_A;
///Reader of field `LTDCRST`
pub type LTDCRST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `LTDCRST`
pub struct LTDCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LTDCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///SAI2 reset
pub type SAI2RST_A = TIM1RST_A;
///Reader of field `SAI2RST`
pub type SAI2RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SAI2RST`
pub struct SAI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAI2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
///SDMMC1 reset
pub type SDMMC1RST_A = TIM1RST_A;
///Reader of field `SDMMC1RST`
pub type SDMMC1RST_R = crate::R<bool, TIM1RST_A>;
///Write proxy for field `SDMMC1RST`
pub struct SDMMC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::RESET)
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
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    pub fn tim10rst(&self) -> TIM10RST_R {
        TIM10RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    pub fn tim11rst(&self) -> TIM11RST_R {
        TIM11RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - SPI5 reset
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - SPI6 reset
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - SAI1 reset
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 26 - LTDC reset
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 23 - SAI2 reset
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 11 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W {
        TIM1RST_W { w: self }
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W {
        TIM8RST_W { w: self }
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W { w: self }
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W {
        USART6RST_W { w: self }
    }
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W { w: self }
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W { w: self }
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W {
        SPI4RST_W { w: self }
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W {
        SYSCFGRST_W { w: self }
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W {
        TIM9RST_W { w: self }
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    pub fn tim10rst(&mut self) -> TIM10RST_W {
        TIM10RST_W { w: self }
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    pub fn tim11rst(&mut self) -> TIM11RST_W {
        TIM11RST_W { w: self }
    }
    ///Bit 20 - SPI5 reset
    #[inline(always)]
    pub fn spi5rst(&mut self) -> SPI5RST_W {
        SPI5RST_W { w: self }
    }
    ///Bit 21 - SPI6 reset
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W {
        SPI6RST_W { w: self }
    }
    ///Bit 22 - SAI1 reset
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W {
        SAI1RST_W { w: self }
    }
    ///Bit 26 - LTDC reset
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W {
        LTDCRST_W { w: self }
    }
    ///Bit 23 - SAI2 reset
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W {
        SAI2RST_W { w: self }
    }
    ///Bit 11 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W {
        SDMMC1RST_W { w: self }
    }
}
