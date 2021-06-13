///Reader of register ALRMBR
pub type R = crate::R<u32, super::ALRMBR>;
///Writer for register ALRMBR
pub type W = crate::W<u32, super::ALRMBR>;
///Register ALRMBR `reset()`'s with value 0
impl crate::ResetValue for super::ALRMBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Alarm B date mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSK4_A {
    ///0: Alarm set if the date/day match
    MASK = 0,
    ///1: Date/day don’t care in Alarm comparison
    NOTMASK = 1,
}
impl From<MSK4_A> for bool {
    #[inline(always)]
    fn from(variant: MSK4_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MSK4`
pub type MSK4_R = crate::R<bool, MSK4_A>;
impl MSK4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSK4_A {
        match self.bits {
            false => MSK4_A::MASK,
            true => MSK4_A::NOTMASK,
        }
    }
    ///Checks if the value of the field is `MASK`
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MSK4_A::MASK
    }
    ///Checks if the value of the field is `NOTMASK`
    #[inline(always)]
    pub fn is_not_mask(&self) -> bool {
        *self == MSK4_A::NOTMASK
    }
}
///Write proxy for field `MSK4`
pub struct MSK4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSK4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Alarm set if the date/day match
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MSK4_A::MASK)
    }
    ///Date/day don’t care in Alarm comparison
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut W {
        self.variant(MSK4_A::NOTMASK)
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
///Week day selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDSEL_A {
    ///0: DU\[3:0\]
    ///represents the date units
    DATEUNITS = 0,
    ///1: DU\[3:0\]
    ///represents the week day. DT\[1:0\]
    ///is don’t care.
    WEEKDAY = 1,
}
impl From<WDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WDSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WDSEL`
pub type WDSEL_R = crate::R<bool, WDSEL_A>;
impl WDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDSEL_A {
        match self.bits {
            false => WDSEL_A::DATEUNITS,
            true => WDSEL_A::WEEKDAY,
        }
    }
    ///Checks if the value of the field is `DATEUNITS`
    #[inline(always)]
    pub fn is_date_units(&self) -> bool {
        *self == WDSEL_A::DATEUNITS
    }
    ///Checks if the value of the field is `WEEKDAY`
    #[inline(always)]
    pub fn is_week_day(&self) -> bool {
        *self == WDSEL_A::WEEKDAY
    }
}
///Write proxy for field `WDSEL`
pub struct WDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DU\[3:0\]
    ///represents the date units
    #[inline(always)]
    pub fn date_units(self) -> &'a mut W {
        self.variant(WDSEL_A::DATEUNITS)
    }
    ///DU\[3:0\]
    ///represents the week day. DT\[1:0\]
    ///is don’t care.
    #[inline(always)]
    pub fn week_day(self) -> &'a mut W {
        self.variant(WDSEL_A::WEEKDAY)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///Reader of field `DT`
pub type DT_R = crate::R<u8, u8>;
///Write proxy for field `DT`
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///Reader of field `DU`
pub type DU_R = crate::R<u8, u8>;
///Write proxy for field `DU`
pub struct DU_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
///Alarm B hours mask
pub type MSK3_A = MSK4_A;
///Reader of field `MSK3`
pub type MSK3_R = crate::R<bool, MSK4_A>;
///Write proxy for field `MSK3`
pub struct MSK3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSK3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Alarm set if the date/day match
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MSK4_A::MASK)
    }
    ///Date/day don’t care in Alarm comparison
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut W {
        self.variant(MSK4_A::NOTMASK)
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
///AM/PM notation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    ///0: AM or 24-hour format
    AM = 0,
    ///1: PM
    PM = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PM`
pub type PM_R = crate::R<bool, PM_A>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::AM,
            true => PM_A::PM,
        }
    }
    ///Checks if the value of the field is `AM`
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == PM_A::AM
    }
    ///Checks if the value of the field is `PM`
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == PM_A::PM
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
            self.bit(variant.into())
        }
    }
    ///AM or 24-hour format
    #[inline(always)]
    pub fn am(self) -> &'a mut W {
        self.variant(PM_A::AM)
    }
    ///PM
    #[inline(always)]
    pub fn pm(self) -> &'a mut W {
        self.variant(PM_A::PM)
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
///Reader of field `HT`
pub type HT_R = crate::R<u8, u8>;
///Write proxy for field `HT`
pub struct HT_W<'a> {
    w: &'a mut W,
}
impl<'a> HT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
///Reader of field `HU`
pub type HU_R = crate::R<u8, u8>;
///Write proxy for field `HU`
pub struct HU_W<'a> {
    w: &'a mut W,
}
impl<'a> HU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
///Alarm B minutes mask
pub type MSK2_A = MSK4_A;
///Reader of field `MSK2`
pub type MSK2_R = crate::R<bool, MSK4_A>;
///Write proxy for field `MSK2`
pub struct MSK2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSK2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Alarm set if the date/day match
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MSK4_A::MASK)
    }
    ///Date/day don’t care in Alarm comparison
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut W {
        self.variant(MSK4_A::NOTMASK)
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
///Reader of field `MNT`
pub type MNT_R = crate::R<u8, u8>;
///Write proxy for field `MNT`
pub struct MNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
///Reader of field `MNU`
pub type MNU_R = crate::R<u8, u8>;
///Write proxy for field `MNU`
pub struct MNU_W<'a> {
    w: &'a mut W,
}
impl<'a> MNU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Alarm B seconds mask
pub type MSK1_A = MSK4_A;
///Reader of field `MSK1`
pub type MSK1_R = crate::R<bool, MSK4_A>;
///Write proxy for field `MSK1`
pub struct MSK1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSK1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Alarm set if the date/day match
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MSK4_A::MASK)
    }
    ///Date/day don’t care in Alarm comparison
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut W {
        self.variant(MSK4_A::NOTMASK)
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
///Reader of field `ST`
pub type ST_R = crate::R<u8, u8>;
///Write proxy for field `ST`
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///Reader of field `SU`
pub type SU_R = crate::R<u8, u8>;
///Write proxy for field `SU`
pub struct SU_W<'a> {
    w: &'a mut W,
}
impl<'a> SU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    ///Bit 31 - Alarm B date mask
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Week day selection
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bits 28:29 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 24:27 - Date units or day in BCD format
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 23 - Alarm B hours mask
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 15 - Alarm B minutes mask
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 7 - Alarm B seconds mask
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bit 31 - Alarm B date mask
    #[inline(always)]
    pub fn msk4(&mut self) -> MSK4_W {
        MSK4_W { w: self }
    }
    ///Bit 30 - Week day selection
    #[inline(always)]
    pub fn wdsel(&mut self) -> WDSEL_W {
        WDSEL_W { w: self }
    }
    ///Bits 28:29 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    ///Bits 24:27 - Date units or day in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W { w: self }
    }
    ///Bit 23 - Alarm B hours mask
    #[inline(always)]
    pub fn msk3(&mut self) -> MSK3_W {
        MSK3_W { w: self }
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W {
        HT_W { w: self }
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W {
        HU_W { w: self }
    }
    ///Bit 15 - Alarm B minutes mask
    #[inline(always)]
    pub fn msk2(&mut self) -> MSK2_W {
        MSK2_W { w: self }
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W {
        MNT_W { w: self }
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W {
        MNU_W { w: self }
    }
    ///Bit 7 - Alarm B seconds mask
    #[inline(always)]
    pub fn msk1(&mut self) -> MSK1_W {
        MSK1_W { w: self }
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    pub fn su(&mut self) -> SU_W {
        SU_W { w: self }
    }
}
