///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CHSEL`
pub type CHSEL_R = crate::R<u8, u8>;
///Write proxy for field `CHSEL`
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
///Memory burst transfer configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MBURST_A {
    ///0: Single transfer
    SINGLE = 0,
    ///1: Incremental burst of 4 beats
    INCR4 = 1,
    ///2: Incremental burst of 8 beats
    INCR8 = 2,
    ///3: Incremental burst of 16 beats
    INCR16 = 3,
}
impl From<MBURST_A> for u8 {
    #[inline(always)]
    fn from(variant: MBURST_A) -> Self {
        variant as _
    }
}
///Reader of field `MBURST`
pub type MBURST_R = crate::R<u8, MBURST_A>;
impl MBURST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MBURST_A {
        match self.bits {
            0 => MBURST_A::SINGLE,
            1 => MBURST_A::INCR4,
            2 => MBURST_A::INCR8,
            3 => MBURST_A::INCR16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `SINGLE`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MBURST_A::SINGLE
    }
    ///Checks if the value of the field is `INCR4`
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == MBURST_A::INCR4
    }
    ///Checks if the value of the field is `INCR8`
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == MBURST_A::INCR8
    }
    ///Checks if the value of the field is `INCR16`
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == MBURST_A::INCR16
    }
}
///Write proxy for field `MBURST`
pub struct MBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> MBURST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MBURST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Single transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MBURST_A::SINGLE)
    }
    ///Incremental burst of 4 beats
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(MBURST_A::INCR4)
    }
    ///Incremental burst of 8 beats
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(MBURST_A::INCR8)
    }
    ///Incremental burst of 16 beats
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(MBURST_A::INCR16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
///Peripheral burst transfer configuration
pub type PBURST_A = MBURST_A;
///Reader of field `PBURST`
pub type PBURST_R = crate::R<u8, MBURST_A>;
///Write proxy for field `PBURST`
pub struct PBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> PBURST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PBURST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Single transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MBURST_A::SINGLE)
    }
    ///Incremental burst of 4 beats
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(MBURST_A::INCR4)
    }
    ///Incremental burst of 8 beats
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(MBURST_A::INCR8)
    }
    ///Incremental burst of 16 beats
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(MBURST_A::INCR16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
///Current target (only in double buffer mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT_A {
    ///0: The current target memory is Memory 0
    MEMORY0 = 0,
    ///1: The current target memory is Memory 1
    MEMORY1 = 1,
}
impl From<CT_A> for bool {
    #[inline(always)]
    fn from(variant: CT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CT`
pub type CT_R = crate::R<bool, CT_A>;
impl CT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CT_A {
        match self.bits {
            false => CT_A::MEMORY0,
            true => CT_A::MEMORY1,
        }
    }
    ///Checks if the value of the field is `MEMORY0`
    #[inline(always)]
    pub fn is_memory0(&self) -> bool {
        *self == CT_A::MEMORY0
    }
    ///Checks if the value of the field is `MEMORY1`
    #[inline(always)]
    pub fn is_memory1(&self) -> bool {
        *self == CT_A::MEMORY1
    }
}
///Write proxy for field `CT`
pub struct CT_W<'a> {
    w: &'a mut W,
}
impl<'a> CT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The current target memory is Memory 0
    #[inline(always)]
    pub fn memory0(self) -> &'a mut W {
        self.variant(CT_A::MEMORY0)
    }
    ///The current target memory is Memory 1
    #[inline(always)]
    pub fn memory1(self) -> &'a mut W {
        self.variant(CT_A::MEMORY1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Double buffer mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBM_A {
    ///0: No buffer switching at the end of transfer
    DISABLED = 0,
    ///1: Memory target switched at the end of the DMA transfer
    ENABLED = 1,
}
impl From<DBM_A> for bool {
    #[inline(always)]
    fn from(variant: DBM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DBM`
pub type DBM_R = crate::R<bool, DBM_A>;
impl DBM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBM_A {
        match self.bits {
            false => DBM_A::DISABLED,
            true => DBM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBM_A::ENABLED
    }
}
///Write proxy for field `DBM`
pub struct DBM_W<'a> {
    w: &'a mut W,
}
impl<'a> DBM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No buffer switching at the end of transfer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBM_A::DISABLED)
    }
    ///Memory target switched at the end of the DMA transfer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Priority level
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PL_A {
    ///0: Low
    LOW = 0,
    ///1: Medium
    MEDIUM = 1,
    ///2: High
    HIGH = 2,
    ///3: Very high
    VERYHIGH = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
///Reader of field `PL`
pub type PL_R = crate::R<u8, PL_A>;
impl PL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::LOW,
            1 => PL_A::MEDIUM,
            2 => PL_A::HIGH,
            3 => PL_A::VERYHIGH,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL_A::LOW
    }
    ///Checks if the value of the field is `MEDIUM`
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL_A::MEDIUM
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL_A::HIGH
    }
    ///Checks if the value of the field is `VERYHIGH`
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL_A::VERYHIGH
    }
}
///Write proxy for field `PL`
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PL_A::LOW)
    }
    ///Medium
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PL_A::MEDIUM)
    }
    ///High
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PL_A::HIGH)
    }
    ///Very high
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PL_A::VERYHIGH)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Peripheral increment offset size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCOS_A {
    ///0: The offset size for the peripheral address calculation is linked to the PSIZE
    PSIZE = 0,
    ///1: The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)
    FIXED4 = 1,
}
impl From<PINCOS_A> for bool {
    #[inline(always)]
    fn from(variant: PINCOS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PINCOS`
pub type PINCOS_R = crate::R<bool, PINCOS_A>;
impl PINCOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PINCOS_A {
        match self.bits {
            false => PINCOS_A::PSIZE,
            true => PINCOS_A::FIXED4,
        }
    }
    ///Checks if the value of the field is `PSIZE`
    #[inline(always)]
    pub fn is_psize(&self) -> bool {
        *self == PINCOS_A::PSIZE
    }
    ///Checks if the value of the field is `FIXED4`
    #[inline(always)]
    pub fn is_fixed4(&self) -> bool {
        *self == PINCOS_A::FIXED4
    }
}
///Write proxy for field `PINCOS`
pub struct PINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCOS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PINCOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The offset size for the peripheral address calculation is linked to the PSIZE
    #[inline(always)]
    pub fn psize(self) -> &'a mut W {
        self.variant(PINCOS_A::PSIZE)
    }
    ///The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)
    #[inline(always)]
    pub fn fixed4(self) -> &'a mut W {
        self.variant(PINCOS_A::FIXED4)
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
///Memory data size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSIZE_A {
    ///0: Byte (8-bit)
    BITS8 = 0,
    ///1: Half-word (16-bit)
    BITS16 = 1,
    ///2: Word (32-bit)
    BITS32 = 2,
}
impl From<MSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIZE_A) -> Self {
        variant as _
    }
}
///Reader of field `MSIZE`
pub type MSIZE_R = crate::R<u8, MSIZE_A>;
impl MSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MSIZE_A::BITS8),
            1 => Val(MSIZE_A::BITS16),
            2 => Val(MSIZE_A::BITS32),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `BITS8`
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MSIZE_A::BITS8
    }
    ///Checks if the value of the field is `BITS16`
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MSIZE_A::BITS16
    }
    ///Checks if the value of the field is `BITS32`
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MSIZE_A::BITS32
    }
}
///Write proxy for field `MSIZE`
pub struct MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIZE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS8)
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS16)
    }
    ///Word (32-bit)
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS32)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
///Peripheral data size
pub type PSIZE_A = MSIZE_A;
///Reader of field `PSIZE`
pub type PSIZE_R = crate::R<u8, MSIZE_A>;
///Write proxy for field `PSIZE`
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS8)
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS16)
    }
    ///Word (32-bit)
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS32)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
///Memory increment mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MINC_A {
    ///0: Address pointer is fixed
    FIXED = 0,
    ///1: Address pointer is incremented after each data transfer
    INCREMENTED = 1,
}
impl From<MINC_A> for bool {
    #[inline(always)]
    fn from(variant: MINC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MINC`
pub type MINC_R = crate::R<bool, MINC_A>;
impl MINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MINC_A {
        match self.bits {
            false => MINC_A::FIXED,
            true => MINC_A::INCREMENTED,
        }
    }
    ///Checks if the value of the field is `FIXED`
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == MINC_A::FIXED
    }
    ///Checks if the value of the field is `INCREMENTED`
    #[inline(always)]
    pub fn is_incremented(&self) -> bool {
        *self == MINC_A::INCREMENTED
    }
}
///Write proxy for field `MINC`
pub struct MINC_W<'a> {
    w: &'a mut W,
}
impl<'a> MINC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Address pointer is fixed
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(MINC_A::FIXED)
    }
    ///Address pointer is incremented after each data transfer
    #[inline(always)]
    pub fn incremented(self) -> &'a mut W {
        self.variant(MINC_A::INCREMENTED)
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
///Peripheral increment mode
pub type PINC_A = MINC_A;
///Reader of field `PINC`
pub type PINC_R = crate::R<bool, MINC_A>;
///Write proxy for field `PINC`
pub struct PINC_W<'a> {
    w: &'a mut W,
}
impl<'a> PINC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Address pointer is fixed
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(MINC_A::FIXED)
    }
    ///Address pointer is incremented after each data transfer
    #[inline(always)]
    pub fn incremented(self) -> &'a mut W {
        self.variant(MINC_A::INCREMENTED)
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
///Circular mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRC_A {
    ///0: Circular mode disabled
    DISABLED = 0,
    ///1: Circular mode enabled
    ENABLED = 1,
}
impl From<CIRC_A> for bool {
    #[inline(always)]
    fn from(variant: CIRC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CIRC`
pub type CIRC_R = crate::R<bool, CIRC_A>;
impl CIRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CIRC_A {
        match self.bits {
            false => CIRC_A::DISABLED,
            true => CIRC_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC_A::ENABLED
    }
}
///Write proxy for field `CIRC`
pub struct CIRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CIRC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CIRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Circular mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRC_A::DISABLED)
    }
    ///Circular mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRC_A::ENABLED)
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
///Data transfer direction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIR_A {
    ///0: Peripheral-to-memory
    PERIPHERALTOMEMORY = 0,
    ///1: Memory-to-peripheral
    MEMORYTOPERIPHERAL = 1,
    ///2: Memory-to-memory
    MEMORYTOMEMORY = 2,
}
impl From<DIR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as _
    }
}
///Reader of field `DIR`
pub type DIR_R = crate::R<u8, DIR_A>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIR_A::PERIPHERALTOMEMORY),
            1 => Val(DIR_A::MEMORYTOPERIPHERAL),
            2 => Val(DIR_A::MEMORYTOMEMORY),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `PERIPHERALTOMEMORY`
    #[inline(always)]
    pub fn is_peripheral_to_memory(&self) -> bool {
        *self == DIR_A::PERIPHERALTOMEMORY
    }
    ///Checks if the value of the field is `MEMORYTOPERIPHERAL`
    #[inline(always)]
    pub fn is_memory_to_peripheral(&self) -> bool {
        *self == DIR_A::MEMORYTOPERIPHERAL
    }
    ///Checks if the value of the field is `MEMORYTOMEMORY`
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == DIR_A::MEMORYTOMEMORY
    }
}
///Write proxy for field `DIR`
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Peripheral-to-memory
    #[inline(always)]
    pub fn peripheral_to_memory(self) -> &'a mut W {
        self.variant(DIR_A::PERIPHERALTOMEMORY)
    }
    ///Memory-to-peripheral
    #[inline(always)]
    pub fn memory_to_peripheral(self) -> &'a mut W {
        self.variant(DIR_A::MEMORYTOPERIPHERAL)
    }
    ///Memory-to-memory
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(DIR_A::MEMORYTOMEMORY)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
///Peripheral flow controller
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFCTRL_A {
    ///0: The DMA is the flow controller
    DMA = 0,
    ///1: The peripheral is the flow controller
    PERIPHERAL = 1,
}
impl From<PFCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: PFCTRL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PFCTRL`
pub type PFCTRL_R = crate::R<bool, PFCTRL_A>;
impl PFCTRL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PFCTRL_A {
        match self.bits {
            false => PFCTRL_A::DMA,
            true => PFCTRL_A::PERIPHERAL,
        }
    }
    ///Checks if the value of the field is `DMA`
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == PFCTRL_A::DMA
    }
    ///Checks if the value of the field is `PERIPHERAL`
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        *self == PFCTRL_A::PERIPHERAL
    }
}
///Write proxy for field `PFCTRL`
pub struct PFCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFCTRL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PFCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The DMA is the flow controller
    #[inline(always)]
    pub fn dma(self) -> &'a mut W {
        self.variant(PFCTRL_A::DMA)
    }
    ///The peripheral is the flow controller
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut W {
        self.variant(PFCTRL_A::PERIPHERAL)
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
///Transfer complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    ///0: TC interrupt disabled
    DISABLED = 0,
    ///1: TC interrupt enabled
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TCIE`
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::ENABLED
    }
}
///Write proxy for field `TCIE`
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///TC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
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
///Half transfer interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIE_A {
    ///0: HT interrupt disabled
    DISABLED = 0,
    ///1: HT interrupt enabled
    ENABLED = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HTIE`
pub type HTIE_R = crate::R<bool, HTIE_A>;
impl HTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::DISABLED,
            true => HTIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE_A::ENABLED
    }
}
///Write proxy for field `HTIE`
pub struct HTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///HT interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIE_A::DISABLED)
    }
    ///HT interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIE_A::ENABLED)
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
///Transfer error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIE_A {
    ///0: TE interrupt disabled
    DISABLED = 0,
    ///1: TE interrupt enabled
    ENABLED = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TEIE`
pub type TEIE_R = crate::R<bool, TEIE_A>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::DISABLED,
            true => TEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE_A::ENABLED
    }
}
///Write proxy for field `TEIE`
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///TE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::DISABLED)
    }
    ///TE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::ENABLED)
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
///Direct mode error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIE_A {
    ///0: DME interrupt disabled
    DISABLED = 0,
    ///1: DME interrupt enabled
    ENABLED = 1,
}
impl From<DMEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DMEIE`
pub type DMEIE_R = crate::R<bool, DMEIE_A>;
impl DMEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMEIE_A {
        match self.bits {
            false => DMEIE_A::DISABLED,
            true => DMEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMEIE_A::ENABLED
    }
}
///Write proxy for field `DMEIE`
pub struct DMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DME interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMEIE_A::DISABLED)
    }
    ///DME interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMEIE_A::ENABLED)
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
///Stream enable / flag stream ready when read low
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    ///0: Stream disabled
    DISABLED = 0,
    ///1: Stream enabled
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EN`
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::ENABLED
    }
}
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Stream disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    ///Stream enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
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
    ///Bits 25:28 - Channel selection
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bits 23:24 - Memory burst transfer configuration
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    ///Bits 21:22 - Peripheral burst transfer configuration
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    ///Bit 19 - Current target (only in double buffer mode)
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bits 16:17 - Priority level
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bit 15 - Peripheral increment offset size
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 13:14 - Memory data size
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    ///Bits 11:12 - Peripheral data size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    ///Bit 10 - Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Circular mode
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 6:7 - Data transfer direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bit 5 - Peripheral flow controller
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Direct mode error interrupt enable
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Stream enable / flag stream ready when read low
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 25:28 - Channel selection
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    ///Bits 23:24 - Memory burst transfer configuration
    #[inline(always)]
    pub fn mburst(&mut self) -> MBURST_W {
        MBURST_W { w: self }
    }
    ///Bits 21:22 - Peripheral burst transfer configuration
    #[inline(always)]
    pub fn pburst(&mut self) -> PBURST_W {
        PBURST_W { w: self }
    }
    ///Bit 19 - Current target (only in double buffer mode)
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W {
        CT_W { w: self }
    }
    ///Bit 18 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W {
        DBM_W { w: self }
    }
    ///Bits 16:17 - Priority level
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
    ///Bit 15 - Peripheral increment offset size
    #[inline(always)]
    pub fn pincos(&mut self) -> PINCOS_W {
        PINCOS_W { w: self }
    }
    ///Bits 13:14 - Memory data size
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W {
        MSIZE_W { w: self }
    }
    ///Bits 11:12 - Peripheral data size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    ///Bit 10 - Memory increment mode
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W {
        MINC_W { w: self }
    }
    ///Bit 9 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W {
        PINC_W { w: self }
    }
    ///Bit 8 - Circular mode
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W {
        CIRC_W { w: self }
    }
    ///Bits 6:7 - Data transfer direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    ///Bit 5 - Peripheral flow controller
    #[inline(always)]
    pub fn pfctrl(&mut self) -> PFCTRL_W {
        PFCTRL_W { w: self }
    }
    ///Bit 4 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    ///Bit 3 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W {
        HTIE_W { w: self }
    }
    ///Bit 2 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    ///Bit 1 - Direct mode error interrupt enable
    #[inline(always)]
    pub fn dmeie(&mut self) -> DMEIE_W {
        DMEIE_W { w: self }
    }
    ///Bit 0 - Stream enable / flag stream ready when read low
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
