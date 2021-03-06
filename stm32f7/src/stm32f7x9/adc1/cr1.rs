///Reader of register CR1
pub type R = crate::R<u32, super::CR1>;
///Writer for register CR1
pub type W = crate::W<u32, super::CR1>;
///Register CR1 `reset()`'s with value 0
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Overrun interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRIE_A {
    ///0: Overrun interrupt disabled
    DISABLED = 0,
    ///1: Overrun interrupt enabled
    ENABLED = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVRIE`
pub type OVRIE_R = crate::R<bool, OVRIE_A>;
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::DISABLED,
            true => OVRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIE_A::ENABLED
    }
}
///Write proxy for field `OVRIE`
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRIE_A::DISABLED)
    }
    ///Overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRIE_A::ENABLED)
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
///Resolution
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    ///0: 12-bit (15 ADCCLK cycles)
    TWELVEBIT = 0,
    ///1: 10-bit (13 ADCCLK cycles)
    TENBIT = 1,
    ///2: 8-bit (11 ADCCLK cycles)
    EIGHTBIT = 2,
    ///3: 6-bit (9 ADCCLK cycles)
    SIXBIT = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
///Reader of field `RES`
pub type RES_R = crate::R<u8, RES_A>;
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::TWELVEBIT,
            1 => RES_A::TENBIT,
            2 => RES_A::EIGHTBIT,
            3 => RES_A::SIXBIT,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `TWELVEBIT`
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES_A::TWELVEBIT
    }
    ///Checks if the value of the field is `TENBIT`
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES_A::TENBIT
    }
    ///Checks if the value of the field is `EIGHTBIT`
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES_A::EIGHTBIT
    }
    ///Checks if the value of the field is `SIXBIT`
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == RES_A::SIXBIT
    }
}
///Write proxy for field `RES`
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///12-bit (15 ADCCLK cycles)
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(RES_A::TWELVEBIT)
    }
    ///10-bit (13 ADCCLK cycles)
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(RES_A::TENBIT)
    }
    ///8-bit (11 ADCCLK cycles)
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(RES_A::EIGHTBIT)
    }
    ///6-bit (9 ADCCLK cycles)
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(RES_A::SIXBIT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
///Analog watchdog enable on regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDEN_A {
    ///0: Analog watchdog disabled on regular channels
    DISABLED = 0,
    ///1: Analog watchdog enabled on regular channels
    ENABLED = 1,
}
impl From<AWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: AWDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AWDEN`
pub type AWDEN_R = crate::R<bool, AWDEN_A>;
impl AWDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWDEN_A {
        match self.bits {
            false => AWDEN_A::DISABLED,
            true => AWDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDEN_A::ENABLED
    }
}
///Write proxy for field `AWDEN`
pub struct AWDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Analog watchdog disabled on regular channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDEN_A::DISABLED)
    }
    ///Analog watchdog enabled on regular channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDEN_A::ENABLED)
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
///Analog watchdog enable on injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAWDEN_A {
    ///0: Analog watchdog disabled on injected channels
    DISABLED = 0,
    ///1: Analog watchdog enabled on injected channels
    ENABLED = 1,
}
impl From<JAWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JAWDEN`
pub type JAWDEN_R = crate::R<bool, JAWDEN_A>;
impl JAWDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JAWDEN_A {
        match self.bits {
            false => JAWDEN_A::DISABLED,
            true => JAWDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWDEN_A::ENABLED
    }
}
///Write proxy for field `JAWDEN`
pub struct JAWDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JAWDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JAWDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Analog watchdog disabled on injected channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWDEN_A::DISABLED)
    }
    ///Analog watchdog enabled on injected channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWDEN_A::ENABLED)
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
///Reader of field `DISCNUM`
pub type DISCNUM_R = crate::R<u8, u8>;
///Write proxy for field `DISCNUM`
pub struct DISCNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCNUM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
///Discontinuous mode on injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDISCEN_A {
    ///0: Discontinuous mode on injected channels disabled
    DISABLED = 0,
    ///1: Discontinuous mode on injected channels enabled
    ENABLED = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JDISCEN`
pub type JDISCEN_R = crate::R<bool, JDISCEN_A>;
impl JDISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::DISABLED,
            true => JDISCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN_A::ENABLED
    }
}
///Write proxy for field `JDISCEN`
pub struct JDISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDISCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JDISCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::DISABLED)
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::ENABLED)
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
///Discontinuous mode on regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCEN_A {
    ///0: Discontinuous mode on regular channels disabled
    DISABLED = 0,
    ///1: Discontinuous mode on regular channels enabled
    ENABLED = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DISCEN`
pub type DISCEN_R = crate::R<bool, DISCEN_A>;
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::DISABLED,
            true => DISCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN_A::ENABLED
    }
}
///Write proxy for field `DISCEN`
pub struct DISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DISCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::DISABLED)
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::ENABLED)
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
///Automatic injected group conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAUTO_A {
    ///0: Automatic injected group conversion disabled
    DISABLED = 0,
    ///1: Automatic injected group conversion enabled
    ENABLED = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JAUTO`
pub type JAUTO_R = crate::R<bool, JAUTO_A>;
impl JAUTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::DISABLED,
            true => JAUTO_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO_A::ENABLED
    }
}
///Write proxy for field `JAUTO`
pub struct JAUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> JAUTO_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JAUTO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTO_A::DISABLED)
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTO_A::ENABLED)
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
///Enable the watchdog on a single channel in scan mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDSGL_A {
    ///0: Analog watchdog enabled on all channels
    ALLCHANNELS = 0,
    ///1: Analog watchdog enabled on a single channel
    SINGLECHANNEL = 1,
}
impl From<AWDSGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AWDSGL`
pub type AWDSGL_R = crate::R<bool, AWDSGL_A>;
impl AWDSGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWDSGL_A {
        match self.bits {
            false => AWDSGL_A::ALLCHANNELS,
            true => AWDSGL_A::SINGLECHANNEL,
        }
    }
    ///Checks if the value of the field is `ALLCHANNELS`
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWDSGL_A::ALLCHANNELS
    }
    ///Checks if the value of the field is `SINGLECHANNEL`
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWDSGL_A::SINGLECHANNEL
    }
}
///Write proxy for field `AWDSGL`
pub struct AWDSGL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDSGL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWDSGL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Analog watchdog enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut W {
        self.variant(AWDSGL_A::ALLCHANNELS)
    }
    ///Analog watchdog enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(AWDSGL_A::SINGLECHANNEL)
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
///Scan mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCAN_A {
    ///0: Scan mode disabled
    DISABLED = 0,
    ///1: Scan mode enabled
    ENABLED = 1,
}
impl From<SCAN_A> for bool {
    #[inline(always)]
    fn from(variant: SCAN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SCAN`
pub type SCAN_R = crate::R<bool, SCAN_A>;
impl SCAN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCAN_A {
        match self.bits {
            false => SCAN_A::DISABLED,
            true => SCAN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCAN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCAN_A::ENABLED
    }
}
///Write proxy for field `SCAN`
pub struct SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SCAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Scan mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCAN_A::DISABLED)
    }
    ///Scan mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCAN_A::ENABLED)
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
///Interrupt enable for injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOCIE_A {
    ///0: JEOC interrupt disabled
    DISABLED = 0,
    ///1: JEOC interrupt enabled
    ENABLED = 1,
}
impl From<JEOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JEOCIE`
pub type JEOCIE_R = crate::R<bool, JEOCIE_A>;
impl JEOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOCIE_A {
        match self.bits {
            false => JEOCIE_A::DISABLED,
            true => JEOCIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE_A::ENABLED
    }
}
///Write proxy for field `JEOCIE`
pub struct JEOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JEOCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///JEOC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::DISABLED)
    }
    ///JEOC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::ENABLED)
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
///Analog watchdog interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDIE_A {
    ///0: Analogue watchdog interrupt disabled
    DISABLED = 0,
    ///1: Analogue watchdog interrupt enabled
    ENABLED = 1,
}
impl From<AWDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AWDIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AWDIE`
pub type AWDIE_R = crate::R<bool, AWDIE_A>;
impl AWDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWDIE_A {
        match self.bits {
            false => AWDIE_A::DISABLED,
            true => AWDIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE_A::ENABLED
    }
}
///Write proxy for field `AWDIE`
pub struct AWDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Analogue watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDIE_A::DISABLED)
    }
    ///Analogue watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDIE_A::ENABLED)
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
///Interrupt enable for EOC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCIE_A {
    ///0: EOC interrupt disabled
    DISABLED = 0,
    ///1: EOC interrupt enabled
    ENABLED = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOCIE`
pub type EOCIE_R = crate::R<bool, EOCIE_A>;
impl EOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::DISABLED,
            true => EOCIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE_A::ENABLED
    }
}
///Write proxy for field `EOCIE`
pub struct EOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::DISABLED)
    }
    ///EOC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::ENABLED)
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
///Reader of field `AWDCH`
pub type AWDCH_R = crate::R<u8, u8>;
///Write proxy for field `AWDCH`
pub struct AWDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDCH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    ///Bit 26 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bits 24:25 - Resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bit 26 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    ///Bits 24:25 - Resolution
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    pub fn awden(&mut self) -> AWDEN_W {
        AWDEN_W { w: self }
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    pub fn jawden(&mut self) -> JAWDEN_W {
        JAWDEN_W { w: self }
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W {
        DISCNUM_W { w: self }
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W {
        JDISCEN_W { w: self }
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W {
        DISCEN_W { w: self }
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W {
        JAUTO_W { w: self }
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W {
        AWDSGL_W { w: self }
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W { w: self }
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W {
        JEOCIE_W { w: self }
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W {
        AWDIE_W { w: self }
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W { w: self }
    }
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W {
        AWDCH_W { w: self }
    }
}
