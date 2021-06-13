///Reader of register MACCR
pub type R = crate::R<u32, super::MACCR>;
///Writer for register MACCR
pub type W = crate::W<u32, super::MACCR>;
///Register MACCR `reset()`'s with value 0x8000
impl crate::ResetValue for super::MACCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
///Receiver enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    ///0: MAC receive state machine is disabled after the completion of the reception of the current frame
    DISABLED = 0,
    ///1: MAC receive state machine is enabled
    ENABLED = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RE`
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::DISABLED,
            true => RE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::ENABLED
    }
}
///Write proxy for field `RE`
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC receive state machine is disabled after the completion of the reception of the current frame
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::DISABLED)
    }
    ///MAC receive state machine is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::ENABLED)
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
///Transmitter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    ///0: MAC transmit state machine is disabled after completion of the transmission of the current frame
    DISABLED = 0,
    ///1: MAC transmit state machine is enabled
    ENABLED = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TE`
pub type TE_R = crate::R<bool, TE_A>;
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::DISABLED,
            true => TE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::ENABLED
    }
}
///Write proxy for field `TE`
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC transmit state machine is disabled after completion of the transmission of the current frame
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::DISABLED)
    }
    ///MAC transmit state machine is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::ENABLED)
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
///Deferral check
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_A {
    ///0: MAC defers until CRS signal goes inactive
    DISABLED = 0,
    ///1: Deferral check function enabled
    ENABLED = 1,
}
impl From<DC_A> for bool {
    #[inline(always)]
    fn from(variant: DC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DC`
pub type DC_R = crate::R<bool, DC_A>;
impl DC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DC_A {
        match self.bits {
            false => DC_A::DISABLED,
            true => DC_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DC_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DC_A::ENABLED
    }
}
///Write proxy for field `DC`
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC defers until CRS signal goes inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DC_A::DISABLED)
    }
    ///Deferral check function enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DC_A::ENABLED)
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
///Back-off limit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BL_A {
    ///0: For retransmission n, wait up to 2^min(n, 10) time slots
    BL10 = 0,
    ///1: For retransmission n, wait up to 2^min(n, 8) time slots
    BL8 = 1,
    ///2: For retransmission n, wait up to 2^min(n, 4) time slots
    BL4 = 2,
    ///3: For retransmission n, wait up to 2^min(n, 1) time slots
    BL1 = 3,
}
impl From<BL_A> for u8 {
    #[inline(always)]
    fn from(variant: BL_A) -> Self {
        variant as _
    }
}
///Reader of field `BL`
pub type BL_R = crate::R<u8, BL_A>;
impl BL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BL_A {
        match self.bits {
            0 => BL_A::BL10,
            1 => BL_A::BL8,
            2 => BL_A::BL4,
            3 => BL_A::BL1,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `BL10`
    #[inline(always)]
    pub fn is_bl10(&self) -> bool {
        *self == BL_A::BL10
    }
    ///Checks if the value of the field is `BL8`
    #[inline(always)]
    pub fn is_bl8(&self) -> bool {
        *self == BL_A::BL8
    }
    ///Checks if the value of the field is `BL4`
    #[inline(always)]
    pub fn is_bl4(&self) -> bool {
        *self == BL_A::BL4
    }
    ///Checks if the value of the field is `BL1`
    #[inline(always)]
    pub fn is_bl1(&self) -> bool {
        *self == BL_A::BL1
    }
}
///Write proxy for field `BL`
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///For retransmission n, wait up to 2^min(n, 10) time slots
    #[inline(always)]
    pub fn bl10(self) -> &'a mut W {
        self.variant(BL_A::BL10)
    }
    ///For retransmission n, wait up to 2^min(n, 8) time slots
    #[inline(always)]
    pub fn bl8(self) -> &'a mut W {
        self.variant(BL_A::BL8)
    }
    ///For retransmission n, wait up to 2^min(n, 4) time slots
    #[inline(always)]
    pub fn bl4(self) -> &'a mut W {
        self.variant(BL_A::BL4)
    }
    ///For retransmission n, wait up to 2^min(n, 1) time slots
    #[inline(always)]
    pub fn bl1(self) -> &'a mut W {
        self.variant(BL_A::BL1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
///Automatic pad/CRC stripping
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APCS_A {
    ///0: MAC passes all incoming frames unmodified
    DISABLED = 0,
    ///1: MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes
    STRIP = 1,
}
impl From<APCS_A> for bool {
    #[inline(always)]
    fn from(variant: APCS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `APCS`
pub type APCS_R = crate::R<bool, APCS_A>;
impl APCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> APCS_A {
        match self.bits {
            false => APCS_A::DISABLED,
            true => APCS_A::STRIP,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APCS_A::DISABLED
    }
    ///Checks if the value of the field is `STRIP`
    #[inline(always)]
    pub fn is_strip(&self) -> bool {
        *self == APCS_A::STRIP
    }
}
///Write proxy for field `APCS`
pub struct APCS_W<'a> {
    w: &'a mut W,
}
impl<'a> APCS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: APCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC passes all incoming frames unmodified
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APCS_A::DISABLED)
    }
    ///MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes
    #[inline(always)]
    pub fn strip(self) -> &'a mut W {
        self.variant(APCS_A::STRIP)
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
///Retry disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_A {
    ///0: MAC attempts retries based on the settings of BL
    ENABLED = 0,
    ///1: MAC attempts only 1 transmission
    DISABLED = 1,
}
impl From<RD_A> for bool {
    #[inline(always)]
    fn from(variant: RD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RD`
pub type RD_R = crate::R<bool, RD_A>;
impl RD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RD_A {
        match self.bits {
            false => RD_A::ENABLED,
            true => RD_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RD_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RD_A::DISABLED
    }
}
///Write proxy for field `RD`
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC attempts retries based on the settings of BL
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RD_A::ENABLED)
    }
    ///MAC attempts only 1 transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RD_A::DISABLED)
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
///IPv4 checksum offload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCO_A {
    ///0: IPv4 checksum offload disabled
    DISABLED = 0,
    ///1: IPv4 checksums are checked in received frames
    OFFLOAD = 1,
}
impl From<IPCO_A> for bool {
    #[inline(always)]
    fn from(variant: IPCO_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IPCO`
pub type IPCO_R = crate::R<bool, IPCO_A>;
impl IPCO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IPCO_A {
        match self.bits {
            false => IPCO_A::DISABLED,
            true => IPCO_A::OFFLOAD,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IPCO_A::DISABLED
    }
    ///Checks if the value of the field is `OFFLOAD`
    #[inline(always)]
    pub fn is_offload(&self) -> bool {
        *self == IPCO_A::OFFLOAD
    }
}
///Write proxy for field `IPCO`
pub struct IPCO_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCO_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IPCO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///IPv4 checksum offload disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IPCO_A::DISABLED)
    }
    ///IPv4 checksums are checked in received frames
    #[inline(always)]
    pub fn offload(self) -> &'a mut W {
        self.variant(IPCO_A::OFFLOAD)
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
///Duplex mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DM_A {
    ///0: MAC operates in half-duplex mode
    HALFDUPLEX = 0,
    ///1: MAC operates in full-duplex mode
    FULLDUPLEX = 1,
}
impl From<DM_A> for bool {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DM`
pub type DM_R = crate::R<bool, DM_A>;
impl DM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DM_A {
        match self.bits {
            false => DM_A::HALFDUPLEX,
            true => DM_A::FULLDUPLEX,
        }
    }
    ///Checks if the value of the field is `HALFDUPLEX`
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == DM_A::HALFDUPLEX
    }
    ///Checks if the value of the field is `FULLDUPLEX`
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == DM_A::FULLDUPLEX
    }
}
///Write proxy for field `DM`
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC operates in half-duplex mode
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(DM_A::HALFDUPLEX)
    }
    ///MAC operates in full-duplex mode
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(DM_A::FULLDUPLEX)
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
///Loopback mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LM_A {
    ///0: Normal mode
    NORMAL = 0,
    ///1: MAC operates in loopback mode at the MII
    LOOPBACK = 1,
}
impl From<LM_A> for bool {
    #[inline(always)]
    fn from(variant: LM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LM`
pub type LM_R = crate::R<bool, LM_A>;
impl LM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LM_A {
        match self.bits {
            false => LM_A::NORMAL,
            true => LM_A::LOOPBACK,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LM_A::NORMAL
    }
    ///Checks if the value of the field is `LOOPBACK`
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == LM_A::LOOPBACK
    }
}
///Write proxy for field `LM`
pub struct LM_W<'a> {
    w: &'a mut W,
}
impl<'a> LM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LM_A::NORMAL)
    }
    ///MAC operates in loopback mode at the MII
    #[inline(always)]
    pub fn loopback(self) -> &'a mut W {
        self.variant(LM_A::LOOPBACK)
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
///Receive own disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROD_A {
    ///0: MAC receives all packets from PHY while transmitting
    ENABLED = 0,
    ///1: MAC disables reception of frames in half-duplex mode
    DISABLED = 1,
}
impl From<ROD_A> for bool {
    #[inline(always)]
    fn from(variant: ROD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ROD`
pub type ROD_R = crate::R<bool, ROD_A>;
impl ROD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ROD_A {
        match self.bits {
            false => ROD_A::ENABLED,
            true => ROD_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROD_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROD_A::DISABLED
    }
}
///Write proxy for field `ROD`
pub struct ROD_W<'a> {
    w: &'a mut W,
}
impl<'a> ROD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ROD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC receives all packets from PHY while transmitting
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROD_A::ENABLED)
    }
    ///MAC disables reception of frames in half-duplex mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROD_A::DISABLED)
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
///Fast Ethernet speed
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FES_A {
    ///0: 10 Mbit/s
    FES10 = 0,
    ///1: 100 Mbit/s
    FES100 = 1,
}
impl From<FES_A> for bool {
    #[inline(always)]
    fn from(variant: FES_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FES`
pub type FES_R = crate::R<bool, FES_A>;
impl FES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FES_A {
        match self.bits {
            false => FES_A::FES10,
            true => FES_A::FES100,
        }
    }
    ///Checks if the value of the field is `FES10`
    #[inline(always)]
    pub fn is_fes10(&self) -> bool {
        *self == FES_A::FES10
    }
    ///Checks if the value of the field is `FES100`
    #[inline(always)]
    pub fn is_fes100(&self) -> bool {
        *self == FES_A::FES100
    }
}
///Write proxy for field `FES`
pub struct FES_W<'a> {
    w: &'a mut W,
}
impl<'a> FES_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///10 Mbit/s
    #[inline(always)]
    pub fn fes10(self) -> &'a mut W {
        self.variant(FES_A::FES10)
    }
    ///100 Mbit/s
    #[inline(always)]
    pub fn fes100(self) -> &'a mut W {
        self.variant(FES_A::FES100)
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
///Carrier sense disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSD_A {
    ///0: Errors generated due to loss of carrier
    ENABLED = 0,
    ///1: No error generated due to loss of carrier
    DISABLED = 1,
}
impl From<CSD_A> for bool {
    #[inline(always)]
    fn from(variant: CSD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CSD`
pub type CSD_R = crate::R<bool, CSD_A>;
impl CSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSD_A {
        match self.bits {
            false => CSD_A::ENABLED,
            true => CSD_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSD_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSD_A::DISABLED
    }
}
///Write proxy for field `CSD`
pub struct CSD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Errors generated due to loss of carrier
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSD_A::ENABLED)
    }
    ///No error generated due to loss of carrier
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSD_A::DISABLED)
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
///Interframe gap
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IFG_A {
    ///0: 96 bit times
    IFG96 = 0,
    ///1: 88 bit times
    IFG88 = 1,
    ///6: 48 bit times
    IFG80 = 6,
    ///7: 40 bit times
    IFG40 = 7,
}
impl From<IFG_A> for u8 {
    #[inline(always)]
    fn from(variant: IFG_A) -> Self {
        variant as _
    }
}
///Reader of field `IFG`
pub type IFG_R = crate::R<u8, IFG_A>;
impl IFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IFG_A::IFG96),
            1 => Val(IFG_A::IFG88),
            6 => Val(IFG_A::IFG80),
            7 => Val(IFG_A::IFG40),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `IFG96`
    #[inline(always)]
    pub fn is_ifg96(&self) -> bool {
        *self == IFG_A::IFG96
    }
    ///Checks if the value of the field is `IFG88`
    #[inline(always)]
    pub fn is_ifg88(&self) -> bool {
        *self == IFG_A::IFG88
    }
    ///Checks if the value of the field is `IFG80`
    #[inline(always)]
    pub fn is_ifg80(&self) -> bool {
        *self == IFG_A::IFG80
    }
    ///Checks if the value of the field is `IFG40`
    #[inline(always)]
    pub fn is_ifg40(&self) -> bool {
        *self == IFG_A::IFG40
    }
}
///Write proxy for field `IFG`
pub struct IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> IFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///96 bit times
    #[inline(always)]
    pub fn ifg96(self) -> &'a mut W {
        self.variant(IFG_A::IFG96)
    }
    ///88 bit times
    #[inline(always)]
    pub fn ifg88(self) -> &'a mut W {
        self.variant(IFG_A::IFG88)
    }
    ///48 bit times
    #[inline(always)]
    pub fn ifg80(self) -> &'a mut W {
        self.variant(IFG_A::IFG80)
    }
    ///40 bit times
    #[inline(always)]
    pub fn ifg40(self) -> &'a mut W {
        self.variant(IFG_A::IFG40)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
///Jabber disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JD_A {
    ///0: Jabber enabled, transmit frames up to 2048 bytes
    ENABLED = 0,
    ///1: Jabber disabled, transmit frames up to 16384 bytes
    DISABLED = 1,
}
impl From<JD_A> for bool {
    #[inline(always)]
    fn from(variant: JD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JD`
pub type JD_R = crate::R<bool, JD_A>;
impl JD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JD_A {
        match self.bits {
            false => JD_A::ENABLED,
            true => JD_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JD_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JD_A::DISABLED
    }
}
///Write proxy for field `JD`
pub struct JD_W<'a> {
    w: &'a mut W,
}
impl<'a> JD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Jabber enabled, transmit frames up to 2048 bytes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JD_A::ENABLED)
    }
    ///Jabber disabled, transmit frames up to 16384 bytes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JD_A::DISABLED)
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
///Watchdog disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WD_A {
    ///0: Watchdog enabled, receive frames limited to 2048 bytes
    ENABLED = 0,
    ///1: Watchdog disabled, receive frames may be up to to 16384 bytes
    DISABLED = 1,
}
impl From<WD_A> for bool {
    #[inline(always)]
    fn from(variant: WD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WD`
pub type WD_R = crate::R<bool, WD_A>;
impl WD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WD_A {
        match self.bits {
            false => WD_A::ENABLED,
            true => WD_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WD_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WD_A::DISABLED
    }
}
///Write proxy for field `WD`
pub struct WD_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Watchdog enabled, receive frames limited to 2048 bytes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WD_A::ENABLED)
    }
    ///Watchdog disabled, receive frames may be up to to 16384 bytes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WD_A::DISABLED)
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
///CRC stripping for type frames
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTF_A {
    ///0: CRC not stripped
    DISABLED = 0,
    ///1: CRC stripped
    ENABLED = 1,
}
impl From<CSTF_A> for bool {
    #[inline(always)]
    fn from(variant: CSTF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CSTF`
pub type CSTF_R = crate::R<bool, CSTF_A>;
impl CSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSTF_A {
        match self.bits {
            false => CSTF_A::DISABLED,
            true => CSTF_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSTF_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSTF_A::ENABLED
    }
}
///Write proxy for field `CSTF`
pub struct CSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CRC not stripped
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSTF_A::DISABLED)
    }
    ///CRC stripped
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSTF_A::ENABLED)
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
impl R {
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Deferral check
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 5:6 - Back-off limit
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bit 7 - Automatic pad/CRC stripping
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 9 - Retry disable
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - IPv4 checksum offload
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Duplex mode
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Loopback mode
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Receive own disable
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Fast Ethernet speed
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - Carrier sense disable
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 17:19 - Interframe gap
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    ///Bit 22 - Jabber disable
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - Watchdog disable
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 25 - CRC stripping for type frames
    #[inline(always)]
    pub fn cstf(&self) -> CSTF_R {
        CSTF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    ///Bit 4 - Deferral check
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    ///Bits 5:6 - Back-off limit
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    ///Bit 7 - Automatic pad/CRC stripping
    #[inline(always)]
    pub fn apcs(&mut self) -> APCS_W {
        APCS_W { w: self }
    }
    ///Bit 9 - Retry disable
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    ///Bit 10 - IPv4 checksum offload
    #[inline(always)]
    pub fn ipco(&mut self) -> IPCO_W {
        IPCO_W { w: self }
    }
    ///Bit 11 - Duplex mode
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
    ///Bit 12 - Loopback mode
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W { w: self }
    }
    ///Bit 13 - Receive own disable
    #[inline(always)]
    pub fn rod(&mut self) -> ROD_W {
        ROD_W { w: self }
    }
    ///Bit 14 - Fast Ethernet speed
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W { w: self }
    }
    ///Bit 16 - Carrier sense disable
    #[inline(always)]
    pub fn csd(&mut self) -> CSD_W {
        CSD_W { w: self }
    }
    ///Bits 17:19 - Interframe gap
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W {
        IFG_W { w: self }
    }
    ///Bit 22 - Jabber disable
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W {
        JD_W { w: self }
    }
    ///Bit 23 - Watchdog disable
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W {
        WD_W { w: self }
    }
    ///Bit 25 - CRC stripping for type frames
    #[inline(always)]
    pub fn cstf(&mut self) -> CSTF_W {
        CSTF_W { w: self }
    }
}
