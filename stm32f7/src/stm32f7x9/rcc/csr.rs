///Reader of register CSR
pub type R = crate::R<u32, super::CSR>;
///Writer for register CSR
pub type W = crate::W<u32, super::CSR>;
///Register CSR `reset()`'s with value 0x0e00_0000
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e00_0000
    }
}
///Low-power reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWRRSTF_A {
    ///0: No reset has occured
    NORESET = 0,
    ///1: A reset has occured
    RESET = 1,
}
impl From<LPWRRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LPWRRSTF`
pub type LPWRRSTF_R = crate::R<bool, LPWRRSTF_A>;
impl LPWRRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPWRRSTF_A {
        match self.bits {
            false => LPWRRSTF_A::NORESET,
            true => LPWRRSTF_A::RESET,
        }
    }
    ///Checks if the value of the field is `NORESET`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == LPWRRSTF_A::NORESET
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPWRRSTF_A::RESET
    }
}
///Write proxy for field `LPWRRSTF`
pub struct LPWRRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPWRRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
///Window watchdog reset flag
pub type WWDGRSTF_A = LPWRRSTF_A;
///Reader of field `WWDGRSTF`
pub type WWDGRSTF_R = crate::R<bool, LPWRRSTF_A>;
///Write proxy for field `WWDGRSTF`
pub struct WWDGRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WWDGRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
///Independent watchdog reset flag
pub type WDGRSTF_A = LPWRRSTF_A;
///Reader of field `WDGRSTF`
pub type WDGRSTF_R = crate::R<bool, LPWRRSTF_A>;
///Write proxy for field `WDGRSTF`
pub struct WDGRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WDGRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
///Software reset flag
pub type SFTRSTF_A = LPWRRSTF_A;
///Reader of field `SFTRSTF`
pub type SFTRSTF_R = crate::R<bool, LPWRRSTF_A>;
///Write proxy for field `SFTRSTF`
pub struct SFTRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SFTRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
///POR/PDR reset flag
pub type PORRSTF_A = LPWRRSTF_A;
///Reader of field `PORRSTF`
pub type PORRSTF_R = crate::R<bool, LPWRRSTF_A>;
///Write proxy for field `PORRSTF`
pub struct PORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PORRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
///PIN reset flag
pub type PADRSTF_A = LPWRRSTF_A;
///Reader of field `PADRSTF`
pub type PADRSTF_R = crate::R<bool, LPWRRSTF_A>;
///Write proxy for field `PADRSTF`
pub struct PADRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PADRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PADRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
///BOR reset flag
pub type BORRSTF_A = LPWRRSTF_A;
///Reader of field `BORRSTF`
pub type BORRSTF_R = crate::R<bool, LPWRRSTF_A>;
///Write proxy for field `BORRSTF`
pub struct BORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BORRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BORRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
///Remove reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVF_A {
    ///1: Clears the reset flag
    CLEAR = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RMVF`
pub type RMVF_R = crate::R<bool, RMVF_A>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RMVF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RMVF_A::CLEAR),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF_A::CLEAR
    }
}
///Write proxy for field `RMVF`
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RMVF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the reset flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::CLEAR)
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
///Internal low-speed oscillator ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDY_A {
    ///0: LSI oscillator not ready
    NOTREADY = 0,
    ///1: LSI oscillator ready
    READY = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LSIRDY`
pub type LSIRDY_R = crate::R<bool, LSIRDY_A>;
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::NOTREADY,
            true => LSIRDY_A::READY,
        }
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY_A::NOTREADY
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY_A::READY
    }
}
///Internal low-speed oscillator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSION_A {
    ///0: LSI oscillator Off
    OFF = 0,
    ///1: LSI oscillator On
    ON = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LSION`
pub type LSION_R = crate::R<bool, LSION_A>;
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::OFF,
            true => LSION_A::ON,
        }
    }
    ///Checks if the value of the field is `OFF`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION_A::OFF
    }
    ///Checks if the value of the field is `ON`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION_A::ON
    }
}
///Write proxy for field `LSION`
pub struct LSION_W<'a> {
    w: &'a mut W,
}
impl<'a> LSION_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///LSI oscillator Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSION_A::OFF)
    }
    ///LSI oscillator On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSION_A::ON)
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
impl R {
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn wdgrstf(&self) -> WDGRSTF_R {
        WDGRSTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 1 - Internal low-speed oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W {
        LPWRRSTF_W { w: self }
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W {
        WWDGRSTF_W { w: self }
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn wdgrstf(&mut self) -> WDGRSTF_W {
        WDGRSTF_W { w: self }
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W {
        SFTRSTF_W { w: self }
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W { w: self }
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W {
        PADRSTF_W { w: self }
    }
    ///Bit 25 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W {
        BORRSTF_W { w: self }
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W { w: self }
    }
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W {
        LSION_W { w: self }
    }
}
