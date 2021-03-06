///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0x83
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x83
    }
}
///PLLI2S clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLI2SRDY_A {
    ///0: Clock not ready
    NOTREADY = 0,
    ///1: Clock ready
    READY = 1,
}
impl From<PLLI2SRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLI2SRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PLLI2SRDY`
pub type PLLI2SRDY_R = crate::R<bool, PLLI2SRDY_A>;
impl PLLI2SRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SRDY_A {
        match self.bits {
            false => PLLI2SRDY_A::NOTREADY,
            true => PLLI2SRDY_A::READY,
        }
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PLLI2SRDY_A::NOTREADY
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PLLI2SRDY_A::READY
    }
}
///PLLI2S enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLI2SON_A {
    ///0: Clock Off
    OFF = 0,
    ///1: Clock On
    ON = 1,
}
impl From<PLLI2SON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLI2SON_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PLLI2SON`
pub type PLLI2SON_R = crate::R<bool, PLLI2SON_A>;
impl PLLI2SON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SON_A {
        match self.bits {
            false => PLLI2SON_A::OFF,
            true => PLLI2SON_A::ON,
        }
    }
    ///Checks if the value of the field is `OFF`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PLLI2SON_A::OFF
    }
    ///Checks if the value of the field is `ON`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PLLI2SON_A::ON
    }
}
///Write proxy for field `PLLI2SON`
pub struct PLLI2SON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLI2SON_A::OFF)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLI2SON_A::ON)
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
///Main PLL (PLL) clock ready flag
pub type PLLRDY_A = PLLI2SRDY_A;
///Reader of field `PLLRDY`
pub type PLLRDY_R = crate::R<bool, PLLI2SRDY_A>;
///Main PLL (PLL) enable
pub type PLLON_A = PLLI2SON_A;
///Reader of field `PLLON`
pub type PLLON_R = crate::R<bool, PLLI2SON_A>;
///Write proxy for field `PLLON`
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLI2SON_A::OFF)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLI2SON_A::ON)
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
///Clock security system enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSON_A {
    ///0: Clock security system disabled (clock detector OFF)
    OFF = 0,
    ///1: Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    ON = 1,
}
impl From<CSSON_A> for bool {
    #[inline(always)]
    fn from(variant: CSSON_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CSSON`
pub type CSSON_R = crate::R<bool, CSSON_A>;
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSON_A {
        match self.bits {
            false => CSSON_A::OFF,
            true => CSSON_A::ON,
        }
    }
    ///Checks if the value of the field is `OFF`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CSSON_A::OFF
    }
    ///Checks if the value of the field is `ON`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CSSON_A::ON
    }
}
///Write proxy for field `CSSON`
pub struct CSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSSON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock security system disabled (clock detector OFF)
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CSSON_A::OFF)
    }
    ///Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CSSON_A::ON)
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
///HSE clock bypass
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYP_A {
    ///0: HSE crystal oscillator not bypassed
    NOTBYPASSED = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    BYPASSED = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HSEBYP`
pub type HSEBYP_R = crate::R<bool, HSEBYP_A>;
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NOTBYPASSED,
            true => HSEBYP_A::BYPASSED,
        }
    }
    ///Checks if the value of the field is `NOTBYPASSED`
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP_A::NOTBYPASSED
    }
    ///Checks if the value of the field is `BYPASSED`
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP_A::BYPASSED
    }
}
///Write proxy for field `HSEBYP`
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSEBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NOTBYPASSED)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::BYPASSED)
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
///HSE clock ready flag
pub type HSERDY_A = PLLI2SRDY_A;
///Reader of field `HSERDY`
pub type HSERDY_R = crate::R<bool, PLLI2SRDY_A>;
///HSE clock enable
pub type HSEON_A = PLLI2SON_A;
///Reader of field `HSEON`
pub type HSEON_R = crate::R<bool, PLLI2SON_A>;
///Write proxy for field `HSEON`
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLI2SON_A::OFF)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLI2SON_A::ON)
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
///Reader of field `HSICAL`
pub type HSICAL_R = crate::R<u8, u8>;
///Reader of field `HSITRIM`
pub type HSITRIM_R = crate::R<u8, u8>;
///Write proxy for field `HSITRIM`
pub struct HSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITRIM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
///Internal high-speed clock ready flag
pub type HSIRDY_A = PLLI2SRDY_A;
///Reader of field `HSIRDY`
pub type HSIRDY_R = crate::R<bool, PLLI2SRDY_A>;
///Internal high-speed clock enable
pub type HSION_A = PLLI2SON_A;
///Reader of field `HSION`
pub type HSION_R = crate::R<bool, PLLI2SON_A>;
///Write proxy for field `HSION`
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLI2SON_A::OFF)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLI2SON_A::ON)
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
///PLLSAI clock ready flag
pub type PLLSAIRDY_A = PLLI2SRDY_A;
///Reader of field `PLLSAIRDY`
pub type PLLSAIRDY_R = crate::R<bool, PLLI2SRDY_A>;
///PLLSAI enable
pub type PLLSAION_A = PLLI2SON_A;
///Reader of field `PLLSAION`
pub type PLLSAION_R = crate::R<bool, PLLI2SON_A>;
///Write proxy for field `PLLSAION`
pub struct PLLSAION_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAION_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLSAION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLI2SON_A::OFF)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLI2SON_A::ON)
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
impl R {
    ///Bit 27 - PLLI2S clock ready flag
    #[inline(always)]
    pub fn plli2srdy(&self) -> PLLI2SRDY_R {
        PLLI2SRDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - PLLI2S enable
    #[inline(always)]
    pub fn plli2son(&self) -> PLLI2SON_R {
        PLLI2SON_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - Main PLL (PLL) clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - Main PLL (PLL) enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 8:15 - Internal high-speed clock calibration
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 3:7 - Internal high-speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bit 1 - Internal high-speed clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 29 - PLLSAI clock ready flag
    #[inline(always)]
    pub fn pllsairdy(&self) -> PLLSAIRDY_R {
        PLLSAIRDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - PLLSAI enable
    #[inline(always)]
    pub fn pllsaion(&self) -> PLLSAION_R {
        PLLSAION_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    ///Bit 26 - PLLI2S enable
    #[inline(always)]
    pub fn plli2son(&mut self) -> PLLI2SON_W {
        PLLI2SON_W { w: self }
    }
    ///Bit 24 - Main PLL (PLL) enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W {
        CSSON_W { w: self }
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    ///Bits 3:7 - Internal high-speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    ///Bit 28 - PLLSAI enable
    #[inline(always)]
    pub fn pllsaion(&mut self) -> PLLSAION_W {
        PLLSAION_W { w: self }
    }
}
