///Reader of register ISR
pub type R = crate::R<u32, super::ISR>;
///Writer for register ISR
pub type W = crate::W<u32, super::ISR>;
///Register ISR `reset()`'s with value 0x07
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
///Alarm A write flag
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAWF_A {
    ///0: Alarm update not allowed
    UPDATENOTALLOWED = 0,
    ///1: Alarm update allowed
    UPDATEALLOWED = 1,
}
impl From<ALRAWF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAWF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ALRAWF`
pub type ALRAWF_R = crate::R<bool, ALRAWF_A>;
impl ALRAWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRAWF_A {
        match self.bits {
            false => ALRAWF_A::UPDATENOTALLOWED,
            true => ALRAWF_A::UPDATEALLOWED,
        }
    }
    ///Checks if the value of the field is `UPDATENOTALLOWED`
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == ALRAWF_A::UPDATENOTALLOWED
    }
    ///Checks if the value of the field is `UPDATEALLOWED`
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == ALRAWF_A::UPDATEALLOWED
    }
}
///Alarm B write flag
pub type ALRBWF_A = ALRAWF_A;
///Reader of field `ALRBWF`
pub type ALRBWF_R = crate::R<bool, ALRAWF_A>;
///Wakeup timer write flag
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTWF_A {
    ///0: Wakeup timer configuration update not allowed
    UPDATENOTALLOWED = 0,
    ///1: Wakeup timer configuration update allowed
    UPDATEALLOWED = 1,
}
impl From<WUTWF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WUTWF`
pub type WUTWF_R = crate::R<bool, WUTWF_A>;
impl WUTWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTWF_A {
        match self.bits {
            false => WUTWF_A::UPDATENOTALLOWED,
            true => WUTWF_A::UPDATEALLOWED,
        }
    }
    ///Checks if the value of the field is `UPDATENOTALLOWED`
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWF_A::UPDATENOTALLOWED
    }
    ///Checks if the value of the field is `UPDATEALLOWED`
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWF_A::UPDATEALLOWED
    }
}
///Shift operation pending
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHPF_A {
    ///0: No shift operation is pending
    NOSHIFTPENDING = 0,
    ///1: A shift operation is pending
    SHIFTPENDING = 1,
}
impl From<SHPF_A> for bool {
    #[inline(always)]
    fn from(variant: SHPF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SHPF`
pub type SHPF_R = crate::R<bool, SHPF_A>;
impl SHPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SHPF_A {
        match self.bits {
            false => SHPF_A::NOSHIFTPENDING,
            true => SHPF_A::SHIFTPENDING,
        }
    }
    ///Checks if the value of the field is `NOSHIFTPENDING`
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPF_A::NOSHIFTPENDING
    }
    ///Checks if the value of the field is `SHIFTPENDING`
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPF_A::SHIFTPENDING
    }
}
///Write proxy for field `SHPF`
pub struct SHPF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHPF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SHPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No shift operation is pending
    #[inline(always)]
    pub fn no_shift_pending(self) -> &'a mut W {
        self.variant(SHPF_A::NOSHIFTPENDING)
    }
    ///A shift operation is pending
    #[inline(always)]
    pub fn shift_pending(self) -> &'a mut W {
        self.variant(SHPF_A::SHIFTPENDING)
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
///Initialization status flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITS_A {
    ///0: Calendar has not been initialized
    NOTINITALIZED = 0,
    ///1: Calendar has been initialized
    INITALIZED = 1,
}
impl From<INITS_A> for bool {
    #[inline(always)]
    fn from(variant: INITS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `INITS`
pub type INITS_R = crate::R<bool, INITS_A>;
impl INITS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INITS_A {
        match self.bits {
            false => INITS_A::NOTINITALIZED,
            true => INITS_A::INITALIZED,
        }
    }
    ///Checks if the value of the field is `NOTINITALIZED`
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITS_A::NOTINITALIZED
    }
    ///Checks if the value of the field is `INITALIZED`
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITS_A::INITALIZED
    }
}
///Registers synchronization flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    ///0: Calendar shadow registers not yet synchronized
    NOTSYNCED = 0,
    ///1: Calendar shadow registers synchronized
    SYNCED = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RSF`
pub type RSF_R = crate::R<bool, RSF_A>;
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::NOTSYNCED,
            true => RSF_A::SYNCED,
        }
    }
    ///Checks if the value of the field is `NOTSYNCED`
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSF_A::NOTSYNCED
    }
    ///Checks if the value of the field is `SYNCED`
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSF_A::SYNCED
    }
}
///Registers synchronization flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_AW {
    ///0: This flag is cleared by software by writing 0
    CLEAR = 0,
}
impl From<RSF_AW> for bool {
    #[inline(always)]
    fn from(variant: RSF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `RSF`
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RSF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSF_AW::CLEAR)
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
///Initialization flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITF_A {
    ///0: Calendar registers update is not allowed
    NOTALLOWED = 0,
    ///1: Calendar registers update is allowed
    ALLOWED = 1,
}
impl From<INITF_A> for bool {
    #[inline(always)]
    fn from(variant: INITF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `INITF`
pub type INITF_R = crate::R<bool, INITF_A>;
impl INITF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INITF_A {
        match self.bits {
            false => INITF_A::NOTALLOWED,
            true => INITF_A::ALLOWED,
        }
    }
    ///Checks if the value of the field is `NOTALLOWED`
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITF_A::NOTALLOWED
    }
    ///Checks if the value of the field is `ALLOWED`
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITF_A::ALLOWED
    }
}
///Initialization mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    ///0: Free running mode
    FREERUNNINGMODE = 0,
    ///1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    INITMODE = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `INIT`
pub type INIT_R = crate::R<bool, INIT_A>;
impl INIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::FREERUNNINGMODE,
            true => INIT_A::INITMODE,
        }
    }
    ///Checks if the value of the field is `FREERUNNINGMODE`
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT_A::FREERUNNINGMODE
    }
    ///Checks if the value of the field is `INITMODE`
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT_A::INITMODE
    }
}
///Write proxy for field `INIT`
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Free running mode
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut W {
        self.variant(INIT_A::FREERUNNINGMODE)
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut W {
        self.variant(INIT_A::INITMODE)
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
///Alarm A flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAF_A {
    ///1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)
    MATCH = 1,
}
impl From<ALRAF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ALRAF`
pub type ALRAF_R = crate::R<bool, ALRAF_A>;
impl ALRAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ALRAF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ALRAF_A::MATCH),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == ALRAF_A::MATCH
    }
}
///Alarm A flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAF_AW {
    ///0: This flag is cleared by software by writing 0
    CLEAR = 0,
}
impl From<ALRAF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `ALRAF`
pub struct ALRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRAF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRAF_AW::CLEAR)
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
///Alarm B flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBF_A {
    ///1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)
    MATCH = 1,
}
impl From<ALRBF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ALRBF`
pub type ALRBF_R = crate::R<bool, ALRBF_A>;
impl ALRBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ALRBF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ALRBF_A::MATCH),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == ALRBF_A::MATCH
    }
}
///Alarm B flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBF_AW {
    ///0: This flag is cleared by software by writing 0
    CLEAR = 0,
}
impl From<ALRBF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `ALRBF`
pub struct ALRBF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRBF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRBF_AW::CLEAR)
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
///Wakeup timer flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTF_A {
    ///1: This flag is set by hardware when the wakeup auto-reload counter reaches 0
    ZERO = 1,
}
impl From<WUTF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WUTF`
pub type WUTF_R = crate::R<bool, WUTF_A>;
impl WUTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WUTF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WUTF_A::ZERO),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `ZERO`
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTF_A::ZERO
    }
}
///Wakeup timer flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTF_AW {
    ///0: This flag is cleared by software by writing 0
    CLEAR = 0,
}
impl From<WUTF_AW> for bool {
    #[inline(always)]
    fn from(variant: WUTF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `WUTF`
pub struct WUTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUTF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUTF_AW::CLEAR)
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
///Time-stamp flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_A {
    ///1: This flag is set by hardware when a time-stamp event occurs
    TIMESTAMPEVENT = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TSF`
pub type TSF_R = crate::R<bool, TSF_A>;
impl TSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TSF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TSF_A::TIMESTAMPEVENT),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `TIMESTAMPEVENT`
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSF_A::TIMESTAMPEVENT
    }
}
///Time-stamp flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_AW {
    ///0: This flag is cleared by software by writing 0
    CLEAR = 0,
}
impl From<TSF_AW> for bool {
    #[inline(always)]
    fn from(variant: TSF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `TSF`
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSF_AW::CLEAR)
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
///Time-stamp overflow flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVF_A {
    ///1: This flag is set by hardware when a time-stamp event occurs while TSF is already set
    OVERFLOW = 1,
}
impl From<TSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TSOVF`
pub type TSOVF_R = crate::R<bool, TSOVF_A>;
impl TSOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TSOVF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TSOVF_A::OVERFLOW),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `OVERFLOW`
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVF_A::OVERFLOW
    }
}
///Time-stamp overflow flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVF_AW {
    ///0: This flag is cleared by software by writing 0
    CLEAR = 0,
}
impl From<TSOVF_AW> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `TSOVF`
pub struct TSOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOVF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSOVF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSOVF_AW::CLEAR)
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
///Tamper detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1F_A {
    ///1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input
    TAMPERED = 1,
}
impl From<TAMP1F_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TAMP1F`
pub type TAMP1F_R = crate::R<bool, TAMP1F_A>;
impl TAMP1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TAMP1F_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TAMP1F_A::TAMPERED),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `TAMPERED`
    #[inline(always)]
    pub fn is_tampered(&self) -> bool {
        *self == TAMP1F_A::TAMPERED
    }
}
///Tamper detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1F_AW {
    ///0: Flag cleared by software writing 0
    CLEAR = 0,
}
impl From<TAMP1F_AW> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `TAMP1F`
pub struct TAMP1F_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP1F_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Flag cleared by software writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TAMP1F_AW::CLEAR)
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
///RTC_TAMP2 detection flag
pub type TAMP2F_A = TAMP1F_A;
///Reader of field `TAMP2F`
pub type TAMP2F_R = crate::R<bool, TAMP1F_A>;
///RTC_TAMP2 detection flag
pub type TAMP2F_AW = TAMP1F_AW;
///Write proxy for field `TAMP2F`
pub struct TAMP2F_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP2F_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Flag cleared by software writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TAMP1F_AW::CLEAR)
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
///RTC_TAMP3 detection flag
pub type TAMP3F_A = TAMP1F_A;
///Reader of field `TAMP3F`
pub type TAMP3F_R = crate::R<bool, TAMP1F_A>;
///RTC_TAMP3 detection flag
pub type TAMP3F_AW = TAMP1F_AW;
///Write proxy for field `TAMP3F`
pub struct TAMP3F_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP3F_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Flag cleared by software writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TAMP1F_AW::CLEAR)
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
///Recalibration pending Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECALPF_A {
    ///1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
    PENDING = 1,
}
impl From<RECALPF_A> for bool {
    #[inline(always)]
    fn from(variant: RECALPF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RECALPF`
pub type RECALPF_R = crate::R<bool, RECALPF_A>;
impl RECALPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RECALPF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RECALPF_A::PENDING),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `PENDING`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPF_A::PENDING
    }
}
impl R {
    ///Bit 0 - Alarm A write flag
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Alarm B write flag
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Wakeup timer write flag
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Initialization status flag
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Initialization flag
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Tamper detection flag
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - RTC_TAMP2 detection flag
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - RTC_TAMP3 detection flag
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Recalibration pending Flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&mut self) -> SHPF_W {
        SHPF_W { w: self }
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&mut self) -> ALRAF_W {
        ALRAF_W { w: self }
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&mut self) -> ALRBF_W {
        ALRBF_W { w: self }
    }
    ///Bit 10 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&mut self) -> WUTF_W {
        WUTF_W { w: self }
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    pub fn tsovf(&mut self) -> TSOVF_W {
        TSOVF_W { w: self }
    }
    ///Bit 13 - Tamper detection flag
    #[inline(always)]
    pub fn tamp1f(&mut self) -> TAMP1F_W {
        TAMP1F_W { w: self }
    }
    ///Bit 14 - RTC_TAMP2 detection flag
    #[inline(always)]
    pub fn tamp2f(&mut self) -> TAMP2F_W {
        TAMP2F_W { w: self }
    }
    ///Bit 15 - RTC_TAMP3 detection flag
    #[inline(always)]
    pub fn tamp3f(&mut self) -> TAMP3F_W {
        TAMP3F_W { w: self }
    }
}
