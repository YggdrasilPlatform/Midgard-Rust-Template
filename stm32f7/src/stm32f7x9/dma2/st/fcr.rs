///Reader of register FCR
pub type R = crate::R<u32, super::FCR>;
///Writer for register FCR
pub type W = crate::W<u32, super::FCR>;
///Register FCR `reset()`'s with value 0x21
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x21
    }
}
///FIFO error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    ///0: FE interrupt disabled
    DISABLED = 0,
    ///1: FE interrupt enabled
    ENABLED = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FEIE`
pub type FEIE_R = crate::R<bool, FEIE_A>;
impl FEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::DISABLED,
            true => FEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE_A::ENABLED
    }
}
///Write proxy for field `FEIE`
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///FE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIE_A::DISABLED)
    }
    ///FE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIE_A::ENABLED)
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
///FIFO status
///
///Value on reset: 4
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FS_A {
    ///0: 0 < fifo_level < 1/4
    QUARTER1 = 0,
    ///1: 1/4 <= fifo_level < 1/2
    QUARTER2 = 1,
    ///2: 1/2 <= fifo_level < 3/4
    QUARTER3 = 2,
    ///3: 3/4 <= fifo_level < full
    QUARTER4 = 3,
    ///4: FIFO is empty
    EMPTY = 4,
    ///5: FIFO is full
    FULL = 5,
}
impl From<FS_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as _
    }
}
///Reader of field `FS`
pub type FS_R = crate::R<u8, FS_A>;
impl FS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FS_A::QUARTER1),
            1 => Val(FS_A::QUARTER2),
            2 => Val(FS_A::QUARTER3),
            3 => Val(FS_A::QUARTER4),
            4 => Val(FS_A::EMPTY),
            5 => Val(FS_A::FULL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `QUARTER1`
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FS_A::QUARTER1
    }
    ///Checks if the value of the field is `QUARTER2`
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FS_A::QUARTER2
    }
    ///Checks if the value of the field is `QUARTER3`
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FS_A::QUARTER3
    }
    ///Checks if the value of the field is `QUARTER4`
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FS_A::QUARTER4
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FS_A::EMPTY
    }
    ///Checks if the value of the field is `FULL`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FS_A::FULL
    }
}
///Direct mode disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMDIS_A {
    ///0: Direct mode is enabled
    ENABLED = 0,
    ///1: Direct mode is disabled
    DISABLED = 1,
}
impl From<DMDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DMDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DMDIS`
pub type DMDIS_R = crate::R<bool, DMDIS_A>;
impl DMDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMDIS_A {
        match self.bits {
            false => DMDIS_A::ENABLED,
            true => DMDIS_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMDIS_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMDIS_A::DISABLED
    }
}
///Write proxy for field `DMDIS`
pub struct DMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Direct mode is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMDIS_A::ENABLED)
    }
    ///Direct mode is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMDIS_A::DISABLED)
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
///FIFO threshold selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTH_A {
    ///0: 1/4 full FIFO
    QUARTER = 0,
    ///1: 1/2 full FIFO
    HALF = 1,
    ///2: 3/4 full FIFO
    THREEQUARTERS = 2,
    ///3: Full FIFO
    FULL = 3,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        variant as _
    }
}
///Reader of field `FTH`
pub type FTH_R = crate::R<u8, FTH_A>;
impl FTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FTH_A {
        match self.bits {
            0 => FTH_A::QUARTER,
            1 => FTH_A::HALF,
            2 => FTH_A::THREEQUARTERS,
            3 => FTH_A::FULL,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `QUARTER`
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FTH_A::QUARTER
    }
    ///Checks if the value of the field is `HALF`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FTH_A::HALF
    }
    ///Checks if the value of the field is `THREEQUARTERS`
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        *self == FTH_A::THREEQUARTERS
    }
    ///Checks if the value of the field is `FULL`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH_A::FULL
    }
}
///Write proxy for field `FTH`
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///1/4 full FIFO
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER)
    }
    ///1/2 full FIFO
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FTH_A::HALF)
    }
    ///3/4 full FIFO
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(FTH_A::THREEQUARTERS)
    }
    ///Full FIFO
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::FULL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bit 7 - FIFO error interrupt enable
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 3:5 - FIFO status
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    ///Bit 2 - Direct mode disable
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 0:1 - FIFO threshold selection
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bit 7 - FIFO error interrupt enable
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    ///Bit 2 - Direct mode disable
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W {
        DMDIS_W { w: self }
    }
    ///Bits 0:1 - FIFO threshold selection
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
}
