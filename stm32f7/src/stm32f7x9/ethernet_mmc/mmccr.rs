///Reader of register MMCCR
pub type R = crate::R<u32, super::MMCCR>;
///Writer for register MMCCR
pub type W = crate::W<u32, super::MMCCR>;
///Register MMCCR `reset()`'s with value 0
impl crate::ResetValue for super::MMCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Counter reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CR_A {
    ///1: Reset all counters. Cleared automatically
    RESET = 1,
}
impl From<CR_A> for bool {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CR`
pub type CR_R = crate::R<bool, CR_A>;
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CR_A::RESET),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CR_A::RESET
    }
}
///Write proxy for field `CR`
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset all counters. Cleared automatically
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CR_A::RESET)
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
///Counter stop rollover
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR_A {
    ///0: Counters roll over to zero after reaching the maximum value
    DISABLED = 0,
    ///1: Counters do not roll over to zero after reaching the maximum value
    ENABLED = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CSR`
pub type CSR_R = crate::R<bool, CSR_A>;
impl CSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::DISABLED,
            true => CSR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSR_A::ENABLED
    }
}
///Write proxy for field `CSR`
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Counters roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSR_A::DISABLED)
    }
    ///Counters do not roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSR_A::ENABLED)
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
///Reset on read
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROR_A {
    ///0: MMC counters do not reset on read
    DISABLED = 0,
    ///1: MMC counters reset to zero after read
    ENABLED = 1,
}
impl From<ROR_A> for bool {
    #[inline(always)]
    fn from(variant: ROR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ROR`
pub type ROR_R = crate::R<bool, ROR_A>;
impl ROR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ROR_A {
        match self.bits {
            false => ROR_A::DISABLED,
            true => ROR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROR_A::ENABLED
    }
}
///Write proxy for field `ROR`
pub struct ROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ROR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ROR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MMC counters do not reset on read
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROR_A::DISABLED)
    }
    ///MMC counters reset to zero after read
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROR_A::ENABLED)
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
///MMC counter freeze
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCF_A {
    ///0: All MMC counters update normally
    UNFROZEN = 0,
    ///1: All MMC counters frozen to their current value
    FROZEN = 1,
}
impl From<MCF_A> for bool {
    #[inline(always)]
    fn from(variant: MCF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MCF`
pub type MCF_R = crate::R<bool, MCF_A>;
impl MCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCF_A {
        match self.bits {
            false => MCF_A::UNFROZEN,
            true => MCF_A::FROZEN,
        }
    }
    ///Checks if the value of the field is `UNFROZEN`
    #[inline(always)]
    pub fn is_unfrozen(&self) -> bool {
        *self == MCF_A::UNFROZEN
    }
    ///Checks if the value of the field is `FROZEN`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == MCF_A::FROZEN
    }
}
///Write proxy for field `MCF`
pub struct MCF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///All MMC counters update normally
    #[inline(always)]
    pub fn unfrozen(self) -> &'a mut W {
        self.variant(MCF_A::UNFROZEN)
    }
    ///All MMC counters frozen to their current value
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(MCF_A::FROZEN)
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
///MMC counter preset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCP_A {
    ///1: MMC counters will be preset to almost full or almost half. Cleared automatically
    PRESET = 1,
}
impl From<MCP_A> for bool {
    #[inline(always)]
    fn from(variant: MCP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MCP`
pub type MCP_R = crate::R<bool, MCP_A>;
impl MCP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MCP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MCP_A::PRESET),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `PRESET`
    #[inline(always)]
    pub fn is_preset(&self) -> bool {
        *self == MCP_A::PRESET
    }
}
///Write proxy for field `MCP`
pub struct MCP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MMC counters will be preset to almost full or almost half. Cleared automatically
    #[inline(always)]
    pub fn preset(self) -> &'a mut W {
        self.variant(MCP_A::PRESET)
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
///MMC counter Full-Half preset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCFHP_A {
    ///0: When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0
    ALMOSTHALF = 0,
    ///1: When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0
    ALMOSTFULL = 1,
}
impl From<MCFHP_A> for bool {
    #[inline(always)]
    fn from(variant: MCFHP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MCFHP`
pub type MCFHP_R = crate::R<bool, MCFHP_A>;
impl MCFHP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCFHP_A {
        match self.bits {
            false => MCFHP_A::ALMOSTHALF,
            true => MCFHP_A::ALMOSTFULL,
        }
    }
    ///Checks if the value of the field is `ALMOSTHALF`
    #[inline(always)]
    pub fn is_almost_half(&self) -> bool {
        *self == MCFHP_A::ALMOSTHALF
    }
    ///Checks if the value of the field is `ALMOSTFULL`
    #[inline(always)]
    pub fn is_almost_full(&self) -> bool {
        *self == MCFHP_A::ALMOSTFULL
    }
}
///Write proxy for field `MCFHP`
pub struct MCFHP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCFHP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCFHP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0
    #[inline(always)]
    pub fn almost_half(self) -> &'a mut W {
        self.variant(MCFHP_A::ALMOSTHALF)
    }
    ///When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0
    #[inline(always)]
    pub fn almost_full(self) -> &'a mut W {
        self.variant(MCFHP_A::ALMOSTFULL)
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
impl R {
    ///Bit 0 - Counter reset
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - MMC counter freeze
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - MMC counter preset
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - MMC counter Full-Half preset
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Counter reset
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    pub fn ror(&mut self) -> ROR_W {
        ROR_W { w: self }
    }
    ///Bit 3 - MMC counter freeze
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W {
        MCF_W { w: self }
    }
    ///Bit 4 - MMC counter preset
    #[inline(always)]
    pub fn mcp(&mut self) -> MCP_W {
        MCP_W { w: self }
    }
    ///Bit 5 - MMC counter Full-Half preset
    #[inline(always)]
    pub fn mcfhp(&mut self) -> MCFHP_W {
        MCFHP_W { w: self }
    }
}
