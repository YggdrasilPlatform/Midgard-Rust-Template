///Reader of register PLLCFGR
pub type R = crate::R<u32, super::PLLCFGR>;
///Writer for register PLLCFGR
pub type W = crate::W<u32, super::PLLCFGR>;
///Register PLLCFGR `reset()`'s with value 0x2400_3010
impl crate::ResetValue for super::PLLCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2400_3010
    }
}
///Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRC_A {
    ///0: HSI clock selected as PLL and PLLI2S clock entry
    HSI = 0,
    ///1: HSE oscillator clock selected as PLL and PLLI2S clock entry
    HSE = 1,
}
impl From<PLLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PLLSRC`
pub type PLLSRC_R = crate::R<bool, PLLSRC_A>;
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            false => PLLSRC_A::HSI,
            true => PLLSRC_A::HSE,
        }
    }
    ///Checks if the value of the field is `HSI`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC_A::HSI
    }
    ///Checks if the value of the field is `HSE`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::HSE
    }
}
///Write proxy for field `PLLSRC`
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///HSI clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI)
    }
    ///HSE oscillator clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE)
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
///Reader of field `PLLR`
pub type PLLR_R = crate::R<u8, u8>;
///Write proxy for field `PLLR`
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
///Reader of field `PLLM`
pub type PLLM_R = crate::R<u8, u8>;
///Write proxy for field `PLLM`
pub struct PLLM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
///Reader of field `PLLN`
pub type PLLN_R = crate::R<u16, u16>;
///Write proxy for field `PLLN`
pub struct PLLN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
///Main PLL (PLL) division factor for main system clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLP_A {
    ///0: PLLP=2
    DIV2 = 0,
    ///1: PLLP=4
    DIV4 = 1,
    ///2: PLLP=6
    DIV6 = 2,
    ///3: PLLP=8
    DIV8 = 3,
}
impl From<PLLP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as _
    }
}
///Reader of field `PLLP`
pub type PLLP_R = crate::R<u8, PLLP_A>;
impl PLLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            0 => PLLP_A::DIV2,
            1 => PLLP_A::DIV4,
            2 => PLLP_A::DIV6,
            3 => PLLP_A::DIV8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLP_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLP_A::DIV4
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLP_A::DIV6
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLP_A::DIV8
    }
}
///Write proxy for field `PLLP`
pub struct PLLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///PLLP=2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLP_A::DIV2)
    }
    ///PLLP=4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLP_A::DIV4)
    }
    ///PLLP=6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLP_A::DIV6)
    }
    ///PLLP=8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLP_A::DIV8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Reader of field `PLLQ`
pub type PLLQ_R = crate::R<u8, u8>;
///Write proxy for field `PLLQ`
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bits 28:30 - PLL division factor for DSI clock
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    ///Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    ///Bits 16:17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    ///Bits 28:30 - PLL division factor for DSI clock
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
    ///Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W {
        PLLM_W { w: self }
    }
    ///Bits 6:14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W {
        PLLN_W { w: self }
    }
    ///Bits 16:17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W {
        PLLP_W { w: self }
    }
    ///Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
}
