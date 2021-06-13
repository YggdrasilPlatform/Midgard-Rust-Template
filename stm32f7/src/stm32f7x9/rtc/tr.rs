///Reader of register TR
pub type R = crate::R<u32, super::TR>;
///Writer for register TR
pub type W = crate::W<u32, super::TR>;
///Register TR `reset()`'s with value 0
impl crate::ResetValue for super::TR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
