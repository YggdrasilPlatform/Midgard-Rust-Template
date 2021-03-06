///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0x8000_0000
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
///Programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PG_A {
    ///1: Flash programming activated
    PROGRAM = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PG`
pub type PG_R = crate::R<bool, PG_A>;
impl PG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PG_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PG_A::PROGRAM),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `PROGRAM`
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == PG_A::PROGRAM
    }
}
///Write proxy for field `PG`
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Flash programming activated
    #[inline(always)]
    pub fn program(self) -> &'a mut W {
        self.variant(PG_A::PROGRAM)
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
///Sector Erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SER_A {
    ///1: Erase activated for selected sector
    SECTORERASE = 1,
}
impl From<SER_A> for bool {
    #[inline(always)]
    fn from(variant: SER_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SER`
pub type SER_R = crate::R<bool, SER_A>;
impl SER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SER_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SER_A::SECTORERASE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `SECTORERASE`
    #[inline(always)]
    pub fn is_sector_erase(&self) -> bool {
        *self == SER_A::SECTORERASE
    }
}
///Write proxy for field `SER`
pub struct SER_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Erase activated for selected sector
    #[inline(always)]
    pub fn sector_erase(self) -> &'a mut W {
        self.variant(SER_A::SECTORERASE)
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
///Mass Erase of sectors 0 to 11
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER1_A {
    ///1: Erase activated for all user sectors or bank 1 in dual bank mode
    MASSERASE = 1,
}
impl From<MER1_A> for bool {
    #[inline(always)]
    fn from(variant: MER1_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MER1`
pub type MER1_R = crate::R<bool, MER1_A>;
impl MER1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MER1_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MER1_A::MASSERASE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `MASSERASE`
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER1_A::MASSERASE
    }
}
///Write proxy for field `MER1`
pub struct MER1_W<'a> {
    w: &'a mut W,
}
impl<'a> MER1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MER1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Erase activated for all user sectors or bank 1 in dual bank mode
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER1_A::MASSERASE)
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
///Reader of field `SNB`
pub type SNB_R = crate::R<u8, u8>;
///Write proxy for field `SNB`
pub struct SNB_W<'a> {
    w: &'a mut W,
}
impl<'a> SNB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
///Program size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSIZE_A {
    ///0: Program x8
    PSIZE8 = 0,
    ///1: Program x16
    PSIZE16 = 1,
    ///2: Program x32
    PSIZE32 = 2,
    ///3: Program x64
    PSIZE64 = 3,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
///Reader of field `PSIZE`
pub type PSIZE_R = crate::R<u8, PSIZE_A>;
impl PSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PSIZE_A {
        match self.bits {
            0 => PSIZE_A::PSIZE8,
            1 => PSIZE_A::PSIZE16,
            2 => PSIZE_A::PSIZE32,
            3 => PSIZE_A::PSIZE64,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PSIZE8`
    #[inline(always)]
    pub fn is_psize8(&self) -> bool {
        *self == PSIZE_A::PSIZE8
    }
    ///Checks if the value of the field is `PSIZE16`
    #[inline(always)]
    pub fn is_psize16(&self) -> bool {
        *self == PSIZE_A::PSIZE16
    }
    ///Checks if the value of the field is `PSIZE32`
    #[inline(always)]
    pub fn is_psize32(&self) -> bool {
        *self == PSIZE_A::PSIZE32
    }
    ///Checks if the value of the field is `PSIZE64`
    #[inline(always)]
    pub fn is_psize64(&self) -> bool {
        *self == PSIZE_A::PSIZE64
    }
}
///Write proxy for field `PSIZE`
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Program x8
    #[inline(always)]
    pub fn psize8(self) -> &'a mut W {
        self.variant(PSIZE_A::PSIZE8)
    }
    ///Program x16
    #[inline(always)]
    pub fn psize16(self) -> &'a mut W {
        self.variant(PSIZE_A::PSIZE16)
    }
    ///Program x32
    #[inline(always)]
    pub fn psize32(self) -> &'a mut W {
        self.variant(PSIZE_A::PSIZE32)
    }
    ///Program x64
    #[inline(always)]
    pub fn psize64(self) -> &'a mut W {
        self.variant(PSIZE_A::PSIZE64)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Mass Erase of sectors 12 to 23
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER2_A {
    ///1: Erase activated for bank 2 in dual bank mode
    MASSERASE = 1,
}
impl From<MER2_A> for bool {
    #[inline(always)]
    fn from(variant: MER2_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MER2`
pub type MER2_R = crate::R<bool, MER2_A>;
impl MER2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MER2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MER2_A::MASSERASE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `MASSERASE`
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER2_A::MASSERASE
    }
}
///Write proxy for field `MER2`
pub struct MER2_W<'a> {
    w: &'a mut W,
}
impl<'a> MER2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MER2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Erase activated for bank 2 in dual bank mode
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER2_A::MASSERASE)
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
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_A {
    ///1: Trigger an erase operation
    START = 1,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `STRT`
pub type STRT_R = crate::R<bool, STRT_A>;
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, STRT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(STRT_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STRT_A::START
    }
}
///Write proxy for field `STRT`
pub struct STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STRT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Trigger an erase operation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STRT_A::START)
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
///End of operation interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPIE_A {
    ///0: End of operation interrupt disabled
    DISABLED = 0,
    ///1: End of operation interrupt enabled
    ENABLED = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOPIE`
pub type EOPIE_R = crate::R<bool, EOPIE_A>;
impl EOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::DISABLED,
            true => EOPIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE_A::ENABLED
    }
}
///Write proxy for field `EOPIE`
pub struct EOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///End of operation interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::DISABLED)
    }
    ///End of operation interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIE_A::ENABLED)
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
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    ///0: Error interrupt generation disabled
    DISABLED = 0,
    ///1: Error interrupt generation enabled
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ERRIE`
pub type ERRIE_R = crate::R<bool, ERRIE_A>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
///Write proxy for field `ERRIE`
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Error interrupt generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    ///Error interrupt generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
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
///Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    ///0: FLASH_CR register is unlocked
    UNLOCKED = 0,
    ///1: FLASH_CR register is locked
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LOCK`
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
///Write proxy for field `LOCK`
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///FLASH_CR register is unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    ///FLASH_CR register is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Sector Erase
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Mass Erase of sectors 0 to 11
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 3:7 - Sector number
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:9 - Program size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 15 - Mass Erase of sectors 12 to 23
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 31 - Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
    ///Bit 1 - Sector Erase
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W {
        SER_W { w: self }
    }
    ///Bit 2 - Mass Erase of sectors 0 to 11
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W {
        MER1_W { w: self }
    }
    ///Bits 3:7 - Sector number
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W {
        SNB_W { w: self }
    }
    ///Bits 8:9 - Program size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    ///Bit 15 - Mass Erase of sectors 12 to 23
    #[inline(always)]
    pub fn mer2(&mut self) -> MER2_W {
        MER2_W { w: self }
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W {
        STRT_W { w: self }
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W { w: self }
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    ///Bit 31 - Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
