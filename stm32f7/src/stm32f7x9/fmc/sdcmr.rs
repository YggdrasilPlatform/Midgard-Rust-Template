///Reader of register SDCMR
pub type R = crate::R<u32, super::SDCMR>;
///Writer for register SDCMR
pub type W = crate::W<u32, super::SDCMR>;
///Register SDCMR `reset()`'s with value 0
impl crate::ResetValue for super::SDCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Command mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_AW {
    ///0: Normal Mode
    NORMAL = 0,
    ///1: Clock Configuration Enable
    CLOCKCONFIGURATIONENABLE = 1,
    ///2: PALL (All Bank Precharge) command
    PALL = 2,
    ///3: Auto-refresh command
    AUTOREFRESHCOMMAND = 3,
    ///4: Load Mode Resgier
    LOADMODEREGISTER = 4,
    ///5: Self-refresh command
    SELFREFRESHCOMMAND = 5,
    ///6: Power-down command
    POWERDOWNCOMMAND = 6,
}
impl From<MODE_AW> for u8 {
    #[inline(always)]
    fn from(variant: MODE_AW) -> Self {
        variant as _
    }
}
///Write proxy for field `MODE`
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Normal Mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_AW::NORMAL)
    }
    ///Clock Configuration Enable
    #[inline(always)]
    pub fn clock_configuration_enable(self) -> &'a mut W {
        self.variant(MODE_AW::CLOCKCONFIGURATIONENABLE)
    }
    ///PALL (All Bank Precharge) command
    #[inline(always)]
    pub fn pall(self) -> &'a mut W {
        self.variant(MODE_AW::PALL)
    }
    ///Auto-refresh command
    #[inline(always)]
    pub fn auto_refresh_command(self) -> &'a mut W {
        self.variant(MODE_AW::AUTOREFRESHCOMMAND)
    }
    ///Load Mode Resgier
    #[inline(always)]
    pub fn load_mode_register(self) -> &'a mut W {
        self.variant(MODE_AW::LOADMODEREGISTER)
    }
    ///Self-refresh command
    #[inline(always)]
    pub fn self_refresh_command(self) -> &'a mut W {
        self.variant(MODE_AW::SELFREFRESHCOMMAND)
    }
    ///Power-down command
    #[inline(always)]
    pub fn power_down_command(self) -> &'a mut W {
        self.variant(MODE_AW::POWERDOWNCOMMAND)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Command target bank 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTB2_AW {
    ///0: Command not issued to SDRAM Bank 1
    NOTISSUED = 0,
    ///1: Command issued to SDRAM Bank 1
    ISSUED = 1,
}
impl From<CTB2_AW> for bool {
    #[inline(always)]
    fn from(variant: CTB2_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `CTB2`
pub struct CTB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTB2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Command not issued to SDRAM Bank 1
    #[inline(always)]
    pub fn not_issued(self) -> &'a mut W {
        self.variant(CTB2_AW::NOTISSUED)
    }
    ///Command issued to SDRAM Bank 1
    #[inline(always)]
    pub fn issued(self) -> &'a mut W {
        self.variant(CTB2_AW::ISSUED)
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
///Command target bank 1
pub type CTB1_AW = CTB2_AW;
///Write proxy for field `CTB1`
pub struct CTB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTB1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Command not issued to SDRAM Bank 1
    #[inline(always)]
    pub fn not_issued(self) -> &'a mut W {
        self.variant(CTB2_AW::NOTISSUED)
    }
    ///Command issued to SDRAM Bank 1
    #[inline(always)]
    pub fn issued(self) -> &'a mut W {
        self.variant(CTB2_AW::ISSUED)
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
///Reader of field `NRFS`
pub type NRFS_R = crate::R<u8, u8>;
///Write proxy for field `NRFS`
pub struct NRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> NRFS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
///Reader of field `MRD`
pub type MRD_R = crate::R<u16, u16>;
///Write proxy for field `MRD`
pub struct MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 9)) | (((value as u32) & 0x1fff) << 9);
        self.w
    }
}
impl R {
    ///Bits 5:8 - Number of Auto-refresh
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:21 - Mode Register definition
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 0:2 - Command mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    ///Bit 3 - Command target bank 2
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W {
        CTB2_W { w: self }
    }
    ///Bit 4 - Command target bank 1
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W {
        CTB1_W { w: self }
    }
    ///Bits 5:8 - Number of Auto-refresh
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W {
        NRFS_W { w: self }
    }
    ///Bits 9:21 - Mode Register definition
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W {
        MRD_W { w: self }
    }
}
