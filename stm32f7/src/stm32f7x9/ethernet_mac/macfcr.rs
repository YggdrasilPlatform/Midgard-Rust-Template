///Reader of register MACFCR
pub type R = crate::R<u32, super::MACFCR>;
///Writer for register MACFCR
pub type W = crate::W<u32, super::MACFCR>;
///Register MACFCR `reset()`'s with value 0
impl crate::ResetValue for super::MACFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Flow control busy/back pressure activate
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCB_A {
    ///1: In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
    PAUSEORBACKPRESSURE = 1,
    ///0: In half duplex only, deasserts back pressure
    DISABLEBACKPRESSURE = 0,
}
impl From<FCB_A> for bool {
    #[inline(always)]
    fn from(variant: FCB_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FCB`
pub type FCB_R = crate::R<bool, FCB_A>;
impl FCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FCB_A {
        match self.bits {
            true => FCB_A::PAUSEORBACKPRESSURE,
            false => FCB_A::DISABLEBACKPRESSURE,
        }
    }
    ///Checks if the value of the field is `PAUSEORBACKPRESSURE`
    #[inline(always)]
    pub fn is_pause_or_back_pressure(&self) -> bool {
        *self == FCB_A::PAUSEORBACKPRESSURE
    }
    ///Checks if the value of the field is `DISABLEBACKPRESSURE`
    #[inline(always)]
    pub fn is_disable_back_pressure(&self) -> bool {
        *self == FCB_A::DISABLEBACKPRESSURE
    }
}
///Write proxy for field `FCB`
pub struct FCB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FCB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
    #[inline(always)]
    pub fn pause_or_back_pressure(self) -> &'a mut W {
        self.variant(FCB_A::PAUSEORBACKPRESSURE)
    }
    ///In half duplex only, deasserts back pressure
    #[inline(always)]
    pub fn disable_back_pressure(self) -> &'a mut W {
        self.variant(FCB_A::DISABLEBACKPRESSURE)
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
///Transmit flow control enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFCE_A {
    ///0: In full duplex, flow control is disabled. In half duplex, back pressure is disabled
    DISABLED = 0,
    ///1: In full duplex, flow control is enabled. In half duplex, back pressure is enabled
    ENABLED = 1,
}
impl From<TFCE_A> for bool {
    #[inline(always)]
    fn from(variant: TFCE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TFCE`
pub type TFCE_R = crate::R<bool, TFCE_A>;
impl TFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TFCE_A {
        match self.bits {
            false => TFCE_A::DISABLED,
            true => TFCE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFCE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFCE_A::ENABLED
    }
}
///Write proxy for field `TFCE`
pub struct TFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TFCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///In full duplex, flow control is disabled. In half duplex, back pressure is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TFCE_A::DISABLED)
    }
    ///In full duplex, flow control is enabled. In half duplex, back pressure is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TFCE_A::ENABLED)
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
///Receive flow control enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCE_A {
    ///0: Pause frames are not decoded
    DISABLED = 0,
    ///1: MAC decodes received Pause frames and disables its transmitted for a specified time
    ENABLED = 1,
}
impl From<RFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RFCE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RFCE`
pub type RFCE_R = crate::R<bool, RFCE_A>;
impl RFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RFCE_A {
        match self.bits {
            false => RFCE_A::DISABLED,
            true => RFCE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFCE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFCE_A::ENABLED
    }
}
///Write proxy for field `RFCE`
pub struct RFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RFCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Pause frames are not decoded
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFCE_A::DISABLED)
    }
    ///MAC decodes received Pause frames and disables its transmitted for a specified time
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFCE_A::ENABLED)
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
///Unicast pause frame detect
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPFD_A {
    ///0: MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
    DISABLED = 0,
    ///1: MAC additionally detects Pause frames with the station's unicast address
    ENABLED = 1,
}
impl From<UPFD_A> for bool {
    #[inline(always)]
    fn from(variant: UPFD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `UPFD`
pub type UPFD_R = crate::R<bool, UPFD_A>;
impl UPFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPFD_A {
        match self.bits {
            false => UPFD_A::DISABLED,
            true => UPFD_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPFD_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPFD_A::ENABLED
    }
}
///Write proxy for field `UPFD`
pub struct UPFD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPFD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UPFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPFD_A::DISABLED)
    }
    ///MAC additionally detects Pause frames with the station's unicast address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPFD_A::ENABLED)
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
///Pause low threshold
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLT_A {
    ///0: Pause time minus 4 slot times
    PLT4 = 0,
    ///1: Pause time minus 28 slot times
    PLT28 = 1,
    ///2: Pause time minus 144 slot times
    PLT144 = 2,
    ///3: Pause time minus 256 slot times
    PLT256 = 3,
}
impl From<PLT_A> for u8 {
    #[inline(always)]
    fn from(variant: PLT_A) -> Self {
        variant as _
    }
}
///Reader of field `PLT`
pub type PLT_R = crate::R<u8, PLT_A>;
impl PLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLT_A {
        match self.bits {
            0 => PLT_A::PLT4,
            1 => PLT_A::PLT28,
            2 => PLT_A::PLT144,
            3 => PLT_A::PLT256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PLT4`
    #[inline(always)]
    pub fn is_plt4(&self) -> bool {
        *self == PLT_A::PLT4
    }
    ///Checks if the value of the field is `PLT28`
    #[inline(always)]
    pub fn is_plt28(&self) -> bool {
        *self == PLT_A::PLT28
    }
    ///Checks if the value of the field is `PLT144`
    #[inline(always)]
    pub fn is_plt144(&self) -> bool {
        *self == PLT_A::PLT144
    }
    ///Checks if the value of the field is `PLT256`
    #[inline(always)]
    pub fn is_plt256(&self) -> bool {
        *self == PLT_A::PLT256
    }
}
///Write proxy for field `PLT`
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Pause time minus 4 slot times
    #[inline(always)]
    pub fn plt4(self) -> &'a mut W {
        self.variant(PLT_A::PLT4)
    }
    ///Pause time minus 28 slot times
    #[inline(always)]
    pub fn plt28(self) -> &'a mut W {
        self.variant(PLT_A::PLT28)
    }
    ///Pause time minus 144 slot times
    #[inline(always)]
    pub fn plt144(self) -> &'a mut W {
        self.variant(PLT_A::PLT144)
    }
    ///Pause time minus 256 slot times
    #[inline(always)]
    pub fn plt256(self) -> &'a mut W {
        self.variant(PLT_A::PLT256)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///Zero-quanta pause disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZQPD_A {
    ///0: Normal operation with automatic zero-quanta pause control frame generation
    ENABLED = 0,
    ///1: Automatic generation of zero-quanta pause control frames is disabled
    DISABLED = 1,
}
impl From<ZQPD_A> for bool {
    #[inline(always)]
    fn from(variant: ZQPD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ZQPD`
pub type ZQPD_R = crate::R<bool, ZQPD_A>;
impl ZQPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ZQPD_A {
        match self.bits {
            false => ZQPD_A::ENABLED,
            true => ZQPD_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ZQPD_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ZQPD_A::DISABLED
    }
}
///Write proxy for field `ZQPD`
pub struct ZQPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQPD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ZQPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Normal operation with automatic zero-quanta pause control frame generation
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ZQPD_A::ENABLED)
    }
    ///Automatic generation of zero-quanta pause control frames is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ZQPD_A::DISABLED)
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
///Reader of field `PT`
pub type PT_R = crate::R<u16, u16>;
///Write proxy for field `PT`
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Flow control busy/back pressure activate
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Transmit flow control enable
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Receive flow control enable
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Unicast pause frame detect
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:5 - Pause low threshold
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 7 - Zero-quanta pause disable
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 16:31 - Pause time
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - Flow control busy/back pressure activate
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W {
        FCB_W { w: self }
    }
    ///Bit 1 - Transmit flow control enable
    #[inline(always)]
    pub fn tfce(&mut self) -> TFCE_W {
        TFCE_W { w: self }
    }
    ///Bit 2 - Receive flow control enable
    #[inline(always)]
    pub fn rfce(&mut self) -> RFCE_W {
        RFCE_W { w: self }
    }
    ///Bit 3 - Unicast pause frame detect
    #[inline(always)]
    pub fn upfd(&mut self) -> UPFD_W {
        UPFD_W { w: self }
    }
    ///Bits 4:5 - Pause low threshold
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W {
        PLT_W { w: self }
    }
    ///Bit 7 - Zero-quanta pause disable
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W {
        ZQPD_W { w: self }
    }
    ///Bits 16:31 - Pause time
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
}
