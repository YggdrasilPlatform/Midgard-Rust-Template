///Reader of register APB1RSTR
pub type R = crate::R<u32, super::APB1RSTR>;
///Writer for register APB1RSTR
pub type W = crate::W<u32, super::APB1RSTR>;
///Register APB1RSTR `reset()`'s with value 0
impl crate::ResetValue for super::APB1RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///TIM2 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2RST_A {
    ///1: Reset the selected module
    RESET = 1,
}
impl From<TIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TIM2RST`
pub type TIM2RST_R = crate::R<bool, TIM2RST_A>;
impl TIM2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TIM2RST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TIM2RST_A::RESET),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST_A::RESET
    }
}
///Write proxy for field `TIM2RST`
pub struct TIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///TIM3 reset
pub type TIM3RST_A = TIM2RST_A;
///Reader of field `TIM3RST`
pub type TIM3RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM3RST`
pub struct TIM3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM3RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///TIM4 reset
pub type TIM4RST_A = TIM2RST_A;
///Reader of field `TIM4RST`
pub type TIM4RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM4RST`
pub struct TIM4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///TIM5 reset
pub type TIM5RST_A = TIM2RST_A;
///Reader of field `TIM5RST`
pub type TIM5RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM5RST`
pub struct TIM5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM5RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///TIM6 reset
pub type TIM6RST_A = TIM2RST_A;
///Reader of field `TIM6RST`
pub type TIM6RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM6RST`
pub struct TIM6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM6RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///TIM7 reset
pub type TIM7RST_A = TIM2RST_A;
///Reader of field `TIM7RST`
pub type TIM7RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM7RST`
pub struct TIM7RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM7RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///TIM12 reset
pub type TIM12RST_A = TIM2RST_A;
///Reader of field `TIM12RST`
pub type TIM12RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM12RST`
pub struct TIM12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM12RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///TIM13 reset
pub type TIM13RST_A = TIM2RST_A;
///Reader of field `TIM13RST`
pub type TIM13RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM13RST`
pub struct TIM13RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM13RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///TIM14 reset
pub type TIM14RST_A = TIM2RST_A;
///Reader of field `TIM14RST`
pub type TIM14RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `TIM14RST`
pub struct TIM14RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM14RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///Window watchdog reset
pub type WWDGRST_A = TIM2RST_A;
///Reader of field `WWDGRST`
pub type WWDGRST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `WWDGRST`
pub struct WWDGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WWDGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///SPI 2 reset
pub type SPI2RST_A = TIM2RST_A;
///Reader of field `SPI2RST`
pub type SPI2RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `SPI2RST`
pub struct SPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///SPI 3 reset
pub type SPI3RST_A = TIM2RST_A;
///Reader of field `SPI3RST`
pub type SPI3RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `SPI3RST`
pub struct SPI3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI3RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///USART 2 reset
pub type UART2RST_A = TIM2RST_A;
///Reader of field `UART2RST`
pub type UART2RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `UART2RST`
pub struct UART2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UART2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///USART 3 reset
pub type UART3RST_A = TIM2RST_A;
///Reader of field `UART3RST`
pub type UART3RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `UART3RST`
pub struct UART3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UART3RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///USART 4 reset
pub type UART4RST_A = TIM2RST_A;
///Reader of field `UART4RST`
pub type UART4RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `UART4RST`
pub struct UART4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UART4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///USART 5 reset
pub type UART5RST_A = TIM2RST_A;
///Reader of field `UART5RST`
pub type UART5RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `UART5RST`
pub struct UART5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UART5RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///I2C 1 reset
pub type I2C1RST_A = TIM2RST_A;
///Reader of field `I2C1RST`
pub type I2C1RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `I2C1RST`
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///I2C 2 reset
pub type I2C2RST_A = TIM2RST_A;
///Reader of field `I2C2RST`
pub type I2C2RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `I2C2RST`
pub struct I2C2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///I2C3 reset
pub type I2C3RST_A = TIM2RST_A;
///Reader of field `I2C3RST`
pub type I2C3RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `I2C3RST`
pub struct I2C3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C3RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///CAN1 reset
pub type CAN1RST_A = TIM2RST_A;
///Reader of field `CAN1RST`
pub type CAN1RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `CAN1RST`
pub struct CAN1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CAN1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///CAN2 reset
pub type CAN2RST_A = TIM2RST_A;
///Reader of field `CAN2RST`
pub type CAN2RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `CAN2RST`
pub struct CAN2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CAN2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///Power interface reset
pub type PWRRST_A = TIM2RST_A;
///Reader of field `PWRRST`
pub type PWRRST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `PWRRST`
pub struct PWRRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PWRRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
///DAC reset
pub type DACRST_A = TIM2RST_A;
///Reader of field `DACRST`
pub type DACRST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `DACRST`
pub struct DACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DACRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DACRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
///UART7 reset
pub type UART7RST_A = TIM2RST_A;
///Reader of field `UART7RST`
pub type UART7RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `UART7RST`
pub struct UART7RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UART7RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///UART8 reset
pub type UART8RST_A = TIM2RST_A;
///Reader of field `UART8RST`
pub type UART8RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `UART8RST`
pub struct UART8RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UART8RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
///SPDIF-RX reset
pub type SPDIFRXRST_A = TIM2RST_A;
///Reader of field `SPDIFRXRST`
pub type SPDIFRXRST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `SPDIFRXRST`
pub struct SPDIFRXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFRXRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPDIFRXRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///HDMI-CEC reset
pub type CECRST_A = TIM2RST_A;
///Reader of field `CECRST`
pub type CECRST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `CECRST`
pub struct CECRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CECRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CECRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///Low power timer 1 reset
pub type LPTIM1RST_A = TIM2RST_A;
///Reader of field `LPTIM1RST`
pub type LPTIM1RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `LPTIM1RST`
pub struct LPTIM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
///I2C 4 reset
pub type I2C4RST_A = TIM2RST_A;
///Reader of field `I2C4RST`
pub type I2C4RST_R = crate::R<bool, TIM2RST_A>;
///Write proxy for field `I2C4RST`
pub struct I2C4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    ///Bit 0 - TIM2 reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM3 reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TIM4 reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - TIM7 reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - TIM12 reset
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - TIM13 reset
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - TIM14 reset
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 14 - SPI 2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - SPI 3 reset
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn uart2rst(&self) -> UART2RST_R {
        UART2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - USART 3 reset
    #[inline(always)]
    pub fn uart3rst(&self) -> UART3RST_R {
        UART3RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - USART 4 reset
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - USART 5 reset
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - I2C 1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - I2C 2 reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 25 - CAN1 reset
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - CAN2 reset
    #[inline(always)]
    pub fn can2rst(&self) -> CAN2RST_R {
        CAN2RST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - DAC reset
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - UART7 reset
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - UART8 reset
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 16 - SPDIF-RX reset
    #[inline(always)]
    pub fn spdifrxrst(&self) -> SPDIFRXRST_R {
        SPDIFRXRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 27 - HDMI-CEC reset
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 9 - Low power timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 24 - I2C 4 reset
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W {
        TIM2RST_W { w: self }
    }
    ///Bit 1 - TIM3 reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W {
        TIM3RST_W { w: self }
    }
    ///Bit 2 - TIM4 reset
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W {
        TIM4RST_W { w: self }
    }
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W {
        TIM5RST_W { w: self }
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W {
        TIM6RST_W { w: self }
    }
    ///Bit 5 - TIM7 reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W {
        TIM7RST_W { w: self }
    }
    ///Bit 6 - TIM12 reset
    #[inline(always)]
    pub fn tim12rst(&mut self) -> TIM12RST_W {
        TIM12RST_W { w: self }
    }
    ///Bit 7 - TIM13 reset
    #[inline(always)]
    pub fn tim13rst(&mut self) -> TIM13RST_W {
        TIM13RST_W { w: self }
    }
    ///Bit 8 - TIM14 reset
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W {
        TIM14RST_W { w: self }
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W {
        WWDGRST_W { w: self }
    }
    ///Bit 14 - SPI 2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W {
        SPI2RST_W { w: self }
    }
    ///Bit 15 - SPI 3 reset
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W {
        SPI3RST_W { w: self }
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn uart2rst(&mut self) -> UART2RST_W {
        UART2RST_W { w: self }
    }
    ///Bit 18 - USART 3 reset
    #[inline(always)]
    pub fn uart3rst(&mut self) -> UART3RST_W {
        UART3RST_W { w: self }
    }
    ///Bit 19 - USART 4 reset
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W {
        UART4RST_W { w: self }
    }
    ///Bit 20 - USART 5 reset
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W {
        UART5RST_W { w: self }
    }
    ///Bit 21 - I2C 1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    ///Bit 22 - I2C 2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W {
        I2C2RST_W { w: self }
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W {
        I2C3RST_W { w: self }
    }
    ///Bit 25 - CAN1 reset
    #[inline(always)]
    pub fn can1rst(&mut self) -> CAN1RST_W {
        CAN1RST_W { w: self }
    }
    ///Bit 26 - CAN2 reset
    #[inline(always)]
    pub fn can2rst(&mut self) -> CAN2RST_W {
        CAN2RST_W { w: self }
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W {
        PWRRST_W { w: self }
    }
    ///Bit 29 - DAC reset
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W {
        DACRST_W { w: self }
    }
    ///Bit 30 - UART7 reset
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W {
        UART7RST_W { w: self }
    }
    ///Bit 31 - UART8 reset
    #[inline(always)]
    pub fn uart8rst(&mut self) -> UART8RST_W {
        UART8RST_W { w: self }
    }
    ///Bit 16 - SPDIF-RX reset
    #[inline(always)]
    pub fn spdifrxrst(&mut self) -> SPDIFRXRST_W {
        SPDIFRXRST_W { w: self }
    }
    ///Bit 27 - HDMI-CEC reset
    #[inline(always)]
    pub fn cecrst(&mut self) -> CECRST_W {
        CECRST_W { w: self }
    }
    ///Bit 9 - Low power timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W {
        LPTIM1RST_W { w: self }
    }
    ///Bit 24 - I2C 4 reset
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W {
        I2C4RST_W { w: self }
    }
}
