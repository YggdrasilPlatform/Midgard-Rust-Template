///Reader of register BDCR
pub type R = crate::R<u32, super::BDCR>;
///Writer for register BDCR
pub type W = crate::W<u32, super::BDCR>;
///Register BDCR `reset()`'s with value 0
impl crate::ResetValue for super::BDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Backup domain software reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRST_A {
    ///0: Reset not activated
    DISABLED = 0,
    ///1: Reset the entire RTC domain
    ENABLED = 1,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BDRST`
pub type BDRST_R = crate::R<bool, BDRST_A>;
impl BDRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BDRST_A {
        match self.bits {
            false => BDRST_A::DISABLED,
            true => BDRST_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDRST_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDRST_A::ENABLED
    }
}
///Write proxy for field `BDRST`
pub struct BDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BDRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BDRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset not activated
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDRST_A::DISABLED)
    }
    ///Reset the entire RTC domain
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDRST_A::ENABLED)
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
///RTC clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    ///0: RTC clock disabled
    DISABLED = 0,
    ///1: RTC clock enabled
    ENABLED = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RTCEN`
pub type RTCEN_R = crate::R<bool, RTCEN_A>;
impl RTCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::DISABLED,
            true => RTCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::ENABLED
    }
}
///Write proxy for field `RTCEN`
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///RTC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::DISABLED)
    }
    ///RTC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLED)
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
///External low-speed oscillator bypass
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYP_A {
    ///0: LSE crystal oscillator not bypassed
    NOTBYPASSED = 0,
    ///1: LSE crystal oscillator bypassed with external clock
    BYPASSED = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LSEBYP`
pub type LSEBYP_R = crate::R<bool, LSEBYP_A>;
impl LSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::NOTBYPASSED,
            true => LSEBYP_A::BYPASSED,
        }
    }
    ///Checks if the value of the field is `NOTBYPASSED`
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP_A::NOTBYPASSED
    }
    ///Checks if the value of the field is `BYPASSED`
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP_A::BYPASSED
    }
}
///Write proxy for field `LSEBYP`
pub struct LSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEBYP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSEBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///LSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NOTBYPASSED)
    }
    ///LSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::BYPASSED)
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
///External low-speed oscillator ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDY_A {
    ///0: LSE oscillator not ready
    NOTREADY = 0,
    ///1: LSE oscillator ready
    READY = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LSERDY`
pub type LSERDY_R = crate::R<bool, LSERDY_A>;
impl LSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::NOTREADY,
            true => LSERDY_A::READY,
        }
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDY_A::NOTREADY
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDY_A::READY
    }
}
///External low-speed oscillator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEON_A {
    ///0: LSE oscillator Off
    OFF = 0,
    ///1: LSE oscillator On
    ON = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LSEON`
pub type LSEON_R = crate::R<bool, LSEON_A>;
impl LSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::OFF,
            true => LSEON_A::ON,
        }
    }
    ///Checks if the value of the field is `OFF`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON_A::OFF
    }
    ///Checks if the value of the field is `ON`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON_A::ON
    }
}
///Write proxy for field `LSEON`
pub struct LSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///LSE oscillator Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::OFF)
    }
    ///LSE oscillator On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::ON)
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
///LSE oscillator drive capability
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSEDRV_A {
    ///0: Low drive capacity
    LOW = 0,
    ///1: Medium-high drive capacity
    MEDIUMHIGH = 1,
    ///2: Medium-low drive capacity
    MEDIUMLOW = 2,
    ///3: High drive capacity
    HIGH = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
///Reader of field `LSEDRV`
pub type LSEDRV_R = crate::R<u8, LSEDRV_A>;
impl LSEDRV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::LOW,
            1 => LSEDRV_A::MEDIUMHIGH,
            2 => LSEDRV_A::MEDIUMLOW,
            3 => LSEDRV_A::HIGH,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV_A::LOW
    }
    ///Checks if the value of the field is `MEDIUMHIGH`
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV_A::MEDIUMHIGH
    }
    ///Checks if the value of the field is `MEDIUMLOW`
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV_A::MEDIUMLOW
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV_A::HIGH
    }
}
///Write proxy for field `LSEDRV`
pub struct LSEDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEDRV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSEDRV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Low drive capacity
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::LOW)
    }
    ///Medium-high drive capacity
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDIUMHIGH)
    }
    ///Medium-low drive capacity
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDIUMLOW)
    }
    ///High drive capacity
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::HIGH)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
///RTC clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSEL_A {
    ///0: No clock
    NOCLOCK = 0,
    ///1: LSE oscillator clock used as RTC clock
    LSE = 1,
    ///2: LSI oscillator clock used as RTC clock
    LSI = 2,
    ///3: HSE oscillator clock divided by a prescaler used as RTC clock
    HSE = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
///Reader of field `RTCSEL`
pub type RTCSEL_R = crate::R<u8, RTCSEL_A>;
impl RTCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NOCLOCK,
            1 => RTCSEL_A::LSE,
            2 => RTCSEL_A::LSI,
            3 => RTCSEL_A::HSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOCLOCK`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL_A::NOCLOCK
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL_A::LSE
    }
    ///Checks if the value of the field is `LSI`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL_A::LSI
    }
    ///Checks if the value of the field is `HSE`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL_A::HSE
    }
}
///Write proxy for field `RTCSEL`
pub struct RTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NOCLOCK)
    }
    ///LSE oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSE)
    }
    ///LSI oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSI)
    }
    ///HSE oscillator clock divided by a prescaler used as RTC clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSEL_A::HSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 2 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - External low-speed oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 3:4 - LSE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W {
        BDRST_W { w: self }
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    ///Bit 2 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W {
        LSEBYP_W { w: self }
    }
    ///Bit 0 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W {
        LSEON_W { w: self }
    }
    ///Bits 3:4 - LSE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W {
        LSEDRV_W { w: self }
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W { w: self }
    }
}
