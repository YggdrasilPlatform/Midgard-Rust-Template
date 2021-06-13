///Reader of register DMABMR
pub type R = crate::R<u32, super::DMABMR>;
///Writer for register DMABMR
pub type W = crate::W<u32, super::DMABMR>;
///Register DMABMR `reset()`'s with value 0x2101
impl crate::ResetValue for super::DMABMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2101
    }
}
///Software reset
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    ///1: Reset all MAC subsystem internal registers and logic. Cleared automatically
    RESET = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SR`
pub type SR_R = crate::R<bool, SR_A>;
impl SR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR_A::RESET),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SR_A::RESET
    }
}
///Write proxy for field `SR`
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset all MAC subsystem internal registers and logic. Cleared automatically
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SR_A::RESET)
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
///DMA arbitration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DA_A {
    ///0: Round-robin with Rx:Tx priority given by PM
    ROUNDROBIN = 0,
    ///1: Rx has priority over Tx
    RXPRIORITY = 1,
}
impl From<DA_A> for bool {
    #[inline(always)]
    fn from(variant: DA_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DA`
pub type DA_R = crate::R<bool, DA_A>;
impl DA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DA_A {
        match self.bits {
            false => DA_A::ROUNDROBIN,
            true => DA_A::RXPRIORITY,
        }
    }
    ///Checks if the value of the field is `ROUNDROBIN`
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == DA_A::ROUNDROBIN
    }
    ///Checks if the value of the field is `RXPRIORITY`
    #[inline(always)]
    pub fn is_rx_priority(&self) -> bool {
        *self == DA_A::RXPRIORITY
    }
}
///Write proxy for field `DA`
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Round-robin with Rx:Tx priority given by PM
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(DA_A::ROUNDROBIN)
    }
    ///Rx has priority over Tx
    #[inline(always)]
    pub fn rx_priority(self) -> &'a mut W {
        self.variant(DA_A::RXPRIORITY)
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
///Reader of field `DSL`
pub type DSL_R = crate::R<u8, u8>;
///Write proxy for field `DSL`
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
///Enhanced descriptor format enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFE_A {
    ///0: Normal descriptor format
    DISABLED = 0,
    ///1: Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload
    ENABLED = 1,
}
impl From<EDFE_A> for bool {
    #[inline(always)]
    fn from(variant: EDFE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EDFE`
pub type EDFE_R = crate::R<bool, EDFE_A>;
impl EDFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EDFE_A {
        match self.bits {
            false => EDFE_A::DISABLED,
            true => EDFE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDFE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDFE_A::ENABLED
    }
}
///Write proxy for field `EDFE`
pub struct EDFE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EDFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Normal descriptor format
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDFE_A::DISABLED)
    }
    ///Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDFE_A::ENABLED)
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
///Programmable burst length
///
///Value on reset: 33
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PBL_A {
    ///1: Maximum of 1 beat per DMA transaction
    PBL1 = 1,
    ///2: Maximum of 2 beats per DMA transaction
    PBL2 = 2,
    ///4: Maximum of 4 beats per DMA transaction
    PBL4 = 4,
    ///8: Maximum of 8 beats per DMA transaction
    PBL8 = 8,
    ///16: Maximum of 16 beats per DMA transaction
    PBL16 = 16,
    ///32: Maximum of 32 beats per DMA transaction
    PBL32 = 32,
}
impl From<PBL_A> for u8 {
    #[inline(always)]
    fn from(variant: PBL_A) -> Self {
        variant as _
    }
}
///Reader of field `PBL`
pub type PBL_R = crate::R<u8, PBL_A>;
impl PBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PBL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PBL_A::PBL1),
            2 => Val(PBL_A::PBL2),
            4 => Val(PBL_A::PBL4),
            8 => Val(PBL_A::PBL8),
            16 => Val(PBL_A::PBL16),
            32 => Val(PBL_A::PBL32),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `PBL1`
    #[inline(always)]
    pub fn is_pbl1(&self) -> bool {
        *self == PBL_A::PBL1
    }
    ///Checks if the value of the field is `PBL2`
    #[inline(always)]
    pub fn is_pbl2(&self) -> bool {
        *self == PBL_A::PBL2
    }
    ///Checks if the value of the field is `PBL4`
    #[inline(always)]
    pub fn is_pbl4(&self) -> bool {
        *self == PBL_A::PBL4
    }
    ///Checks if the value of the field is `PBL8`
    #[inline(always)]
    pub fn is_pbl8(&self) -> bool {
        *self == PBL_A::PBL8
    }
    ///Checks if the value of the field is `PBL16`
    #[inline(always)]
    pub fn is_pbl16(&self) -> bool {
        *self == PBL_A::PBL16
    }
    ///Checks if the value of the field is `PBL32`
    #[inline(always)]
    pub fn is_pbl32(&self) -> bool {
        *self == PBL_A::PBL32
    }
}
///Write proxy for field `PBL`
pub struct PBL_W<'a> {
    w: &'a mut W,
}
impl<'a> PBL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PBL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Maximum of 1 beat per DMA transaction
    #[inline(always)]
    pub fn pbl1(self) -> &'a mut W {
        self.variant(PBL_A::PBL1)
    }
    ///Maximum of 2 beats per DMA transaction
    #[inline(always)]
    pub fn pbl2(self) -> &'a mut W {
        self.variant(PBL_A::PBL2)
    }
    ///Maximum of 4 beats per DMA transaction
    #[inline(always)]
    pub fn pbl4(self) -> &'a mut W {
        self.variant(PBL_A::PBL4)
    }
    ///Maximum of 8 beats per DMA transaction
    #[inline(always)]
    pub fn pbl8(self) -> &'a mut W {
        self.variant(PBL_A::PBL8)
    }
    ///Maximum of 16 beats per DMA transaction
    #[inline(always)]
    pub fn pbl16(self) -> &'a mut W {
        self.variant(PBL_A::PBL16)
    }
    ///Maximum of 32 beats per DMA transaction
    #[inline(always)]
    pub fn pbl32(self) -> &'a mut W {
        self.variant(PBL_A::PBL32)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
///Rx-Tx priority ratio
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PM_A {
    ///0: RxDMA priority over TxDMA is 1:1
    ONETOONE = 0,
    ///1: RxDMA priority over TxDMA is 2:1
    TWOTOONE = 1,
    ///2: RxDMA priority over TxDMA is 3:1
    THREETOONE = 2,
    ///3: RxDMA priority over TxDMA is 4:1
    FOURTOONE = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
///Reader of field `PM`
pub type PM_R = crate::R<u8, PM_A>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            0 => PM_A::ONETOONE,
            1 => PM_A::TWOTOONE,
            2 => PM_A::THREETOONE,
            3 => PM_A::FOURTOONE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `ONETOONE`
    #[inline(always)]
    pub fn is_one_to_one(&self) -> bool {
        *self == PM_A::ONETOONE
    }
    ///Checks if the value of the field is `TWOTOONE`
    #[inline(always)]
    pub fn is_two_to_one(&self) -> bool {
        *self == PM_A::TWOTOONE
    }
    ///Checks if the value of the field is `THREETOONE`
    #[inline(always)]
    pub fn is_three_to_one(&self) -> bool {
        *self == PM_A::THREETOONE
    }
    ///Checks if the value of the field is `FOURTOONE`
    #[inline(always)]
    pub fn is_four_to_one(&self) -> bool {
        *self == PM_A::FOURTOONE
    }
}
///Write proxy for field `PM`
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///RxDMA priority over TxDMA is 1:1
    #[inline(always)]
    pub fn one_to_one(self) -> &'a mut W {
        self.variant(PM_A::ONETOONE)
    }
    ///RxDMA priority over TxDMA is 2:1
    #[inline(always)]
    pub fn two_to_one(self) -> &'a mut W {
        self.variant(PM_A::TWOTOONE)
    }
    ///RxDMA priority over TxDMA is 3:1
    #[inline(always)]
    pub fn three_to_one(self) -> &'a mut W {
        self.variant(PM_A::THREETOONE)
    }
    ///RxDMA priority over TxDMA is 4:1
    #[inline(always)]
    pub fn four_to_one(self) -> &'a mut W {
        self.variant(PM_A::FOURTOONE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Fixed burst
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB_A {
    ///0: AHB uses SINGLE and INCR burst transfers
    VARIABLE = 0,
    ///1: AHB uses only fixed burst transfers
    FIXED = 1,
}
impl From<FB_A> for bool {
    #[inline(always)]
    fn from(variant: FB_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FB`
pub type FB_R = crate::R<bool, FB_A>;
impl FB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FB_A {
        match self.bits {
            false => FB_A::VARIABLE,
            true => FB_A::FIXED,
        }
    }
    ///Checks if the value of the field is `VARIABLE`
    #[inline(always)]
    pub fn is_variable(&self) -> bool {
        *self == FB_A::VARIABLE
    }
    ///Checks if the value of the field is `FIXED`
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == FB_A::FIXED
    }
}
///Write proxy for field `FB`
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///AHB uses SINGLE and INCR burst transfers
    #[inline(always)]
    pub fn variable(self) -> &'a mut W {
        self.variant(FB_A::VARIABLE)
    }
    ///AHB uses only fixed burst transfers
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(FB_A::FIXED)
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
///Rx DMA PBL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDP_A {
    ///1: 1 beat per RxDMA transaction
    RDP1 = 1,
    ///2: 2 beats per RxDMA transaction
    RDP2 = 2,
    ///4: 4 beats per RxDMA transaction
    RDP4 = 4,
    ///8: 8 beats per RxDMA transaction
    RDP8 = 8,
    ///16: 16 beats per RxDMA transaction
    RDP16 = 16,
    ///32: 32 beats per RxDMA transaction
    RDP32 = 32,
}
impl From<RDP_A> for u8 {
    #[inline(always)]
    fn from(variant: RDP_A) -> Self {
        variant as _
    }
}
///Reader of field `RDP`
pub type RDP_R = crate::R<u8, RDP_A>;
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDP_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RDP_A::RDP1),
            2 => Val(RDP_A::RDP2),
            4 => Val(RDP_A::RDP4),
            8 => Val(RDP_A::RDP8),
            16 => Val(RDP_A::RDP16),
            32 => Val(RDP_A::RDP32),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RDP1`
    #[inline(always)]
    pub fn is_rdp1(&self) -> bool {
        *self == RDP_A::RDP1
    }
    ///Checks if the value of the field is `RDP2`
    #[inline(always)]
    pub fn is_rdp2(&self) -> bool {
        *self == RDP_A::RDP2
    }
    ///Checks if the value of the field is `RDP4`
    #[inline(always)]
    pub fn is_rdp4(&self) -> bool {
        *self == RDP_A::RDP4
    }
    ///Checks if the value of the field is `RDP8`
    #[inline(always)]
    pub fn is_rdp8(&self) -> bool {
        *self == RDP_A::RDP8
    }
    ///Checks if the value of the field is `RDP16`
    #[inline(always)]
    pub fn is_rdp16(&self) -> bool {
        *self == RDP_A::RDP16
    }
    ///Checks if the value of the field is `RDP32`
    #[inline(always)]
    pub fn is_rdp32(&self) -> bool {
        *self == RDP_A::RDP32
    }
}
///Write proxy for field `RDP`
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///1 beat per RxDMA transaction
    #[inline(always)]
    pub fn rdp1(self) -> &'a mut W {
        self.variant(RDP_A::RDP1)
    }
    ///2 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp2(self) -> &'a mut W {
        self.variant(RDP_A::RDP2)
    }
    ///4 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp4(self) -> &'a mut W {
        self.variant(RDP_A::RDP4)
    }
    ///8 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp8(self) -> &'a mut W {
        self.variant(RDP_A::RDP8)
    }
    ///16 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp16(self) -> &'a mut W {
        self.variant(RDP_A::RDP16)
    }
    ///32 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp32(self) -> &'a mut W {
        self.variant(RDP_A::RDP32)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | (((value as u32) & 0x3f) << 17);
        self.w
    }
}
///Use separate PBL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USP_A {
    ///0: PBL value used for both Rx and Tx DMA
    COMBINED = 0,
    ///1: RxDMA uses RDP value, TxDMA uses PBL value
    SEPARATE = 1,
}
impl From<USP_A> for bool {
    #[inline(always)]
    fn from(variant: USP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `USP`
pub type USP_R = crate::R<bool, USP_A>;
impl USP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USP_A {
        match self.bits {
            false => USP_A::COMBINED,
            true => USP_A::SEPARATE,
        }
    }
    ///Checks if the value of the field is `COMBINED`
    #[inline(always)]
    pub fn is_combined(&self) -> bool {
        *self == USP_A::COMBINED
    }
    ///Checks if the value of the field is `SEPARATE`
    #[inline(always)]
    pub fn is_separate(&self) -> bool {
        *self == USP_A::SEPARATE
    }
}
///Write proxy for field `USP`
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///PBL value used for both Rx and Tx DMA
    #[inline(always)]
    pub fn combined(self) -> &'a mut W {
        self.variant(USP_A::COMBINED)
    }
    ///RxDMA uses RDP value, TxDMA uses PBL value
    #[inline(always)]
    pub fn separate(self) -> &'a mut W {
        self.variant(USP_A::SEPARATE)
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
///4xPBL mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPM_A {
    ///0: PBL values used as-is
    X1 = 0,
    ///1: PBL values multiplied by 4
    X4 = 1,
}
impl From<FPM_A> for bool {
    #[inline(always)]
    fn from(variant: FPM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FPM`
pub type FPM_R = crate::R<bool, FPM_A>;
impl FPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPM_A {
        match self.bits {
            false => FPM_A::X1,
            true => FPM_A::X4,
        }
    }
    ///Checks if the value of the field is `X1`
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == FPM_A::X1
    }
    ///Checks if the value of the field is `X4`
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == FPM_A::X4
    }
}
///Write proxy for field `FPM`
pub struct FPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FPM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///PBL values used as-is
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(FPM_A::X1)
    }
    ///PBL values multiplied by 4
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(FPM_A::X4)
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
///Address-aligned beats
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAB_A {
    ///0: Bursts are not aligned
    UNALIGNED = 0,
    ///1: Align bursts to start address LS bits. First burst alignment depends on FB bit
    ALIGNED = 1,
}
impl From<AAB_A> for bool {
    #[inline(always)]
    fn from(variant: AAB_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AAB`
pub type AAB_R = crate::R<bool, AAB_A>;
impl AAB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AAB_A {
        match self.bits {
            false => AAB_A::UNALIGNED,
            true => AAB_A::ALIGNED,
        }
    }
    ///Checks if the value of the field is `UNALIGNED`
    #[inline(always)]
    pub fn is_unaligned(&self) -> bool {
        *self == AAB_A::UNALIGNED
    }
    ///Checks if the value of the field is `ALIGNED`
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        *self == AAB_A::ALIGNED
    }
}
///Write proxy for field `AAB`
pub struct AAB_W<'a> {
    w: &'a mut W,
}
impl<'a> AAB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AAB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Bursts are not aligned
    #[inline(always)]
    pub fn unaligned(self) -> &'a mut W {
        self.variant(AAB_A::UNALIGNED)
    }
    ///Align bursts to start address LS bits. First burst alignment depends on FB bit
    #[inline(always)]
    pub fn aligned(self) -> &'a mut W {
        self.variant(AAB_A::ALIGNED)
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
///Mixed burst
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MB_A {
    ///0: Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below
    NORMAL = 0,
    ///1: If FB is low, start all bursts greater than 16 with INCR (undefined burst)
    MIXED = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MB`
pub type MB_R = crate::R<bool, MB_A>;
impl MB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MB_A {
        match self.bits {
            false => MB_A::NORMAL,
            true => MB_A::MIXED,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MB_A::NORMAL
    }
    ///Checks if the value of the field is `MIXED`
    #[inline(always)]
    pub fn is_mixed(&self) -> bool {
        *self == MB_A::MIXED
    }
}
///Write proxy for field `MB`
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MB_A::NORMAL)
    }
    ///If FB is low, start all bursts greater than 16 with INCR (undefined burst)
    #[inline(always)]
    pub fn mixed(self) -> &'a mut W {
        self.variant(MB_A::MIXED)
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
impl R {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - DMA arbitration
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - Rx-Tx priority ratio
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - Mixed burst
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    ///Bit 1 - DMA arbitration
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    pub fn edfe(&mut self) -> EDFE_W {
        EDFE_W { w: self }
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W {
        PBL_W { w: self }
    }
    ///Bits 14:15 - Rx-Tx priority ratio
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W {
        FPM_W { w: self }
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W {
        AAB_W { w: self }
    }
    ///Bit 26 - Mixed burst
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
}
