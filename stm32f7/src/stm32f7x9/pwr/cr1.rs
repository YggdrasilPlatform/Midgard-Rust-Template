///Reader of register CR1
pub type R = crate::R<u32, super::CR1>;
///Writer for register CR1
pub type W = crate::W<u32, super::CR1>;
///Register CR1 `reset()`'s with value 0xc000
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000
    }
}
///Reader of field `LPDS`
pub type LPDS_R = crate::R<bool, bool>;
///Write proxy for field `LPDS`
pub struct LPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDS_W<'a> {
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
///Power down deepsleep
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDS_A {
    ///0: Enter Stop mode when the CPU enters deepsleep
    STOP_MODE = 0,
    ///1: Enter Standby mode when the CPU enters deepsleep
    STANDBY_MODE = 1,
}
impl From<PDDS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PDDS`
pub type PDDS_R = crate::R<bool, PDDS_A>;
impl PDDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PDDS_A {
        match self.bits {
            false => PDDS_A::STOP_MODE,
            true => PDDS_A::STANDBY_MODE,
        }
    }
    ///Checks if the value of the field is `STOP_MODE`
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS_A::STOP_MODE
    }
    ///Checks if the value of the field is `STANDBY_MODE`
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS_A::STANDBY_MODE
    }
}
///Write proxy for field `PDDS`
pub struct PDDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PDDS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Enter Stop mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDS_A::STOP_MODE)
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDS_A::STANDBY_MODE)
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
///Reader of field `CSBF`
pub type CSBF_R = crate::R<bool, bool>;
///Write proxy for field `CSBF`
pub struct CSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSBF_W<'a> {
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
///Reader of field `PVDE`
pub type PVDE_R = crate::R<bool, bool>;
///Write proxy for field `PVDE`
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
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
///Reader of field `PLS`
pub type PLS_R = crate::R<u8, u8>;
///Write proxy for field `PLS`
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
///Reader of field `DBP`
pub type DBP_R = crate::R<bool, bool>;
///Write proxy for field `DBP`
pub struct DBP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBP_W<'a> {
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
///Reader of field `FPDS`
pub type FPDS_R = crate::R<bool, bool>;
///Write proxy for field `FPDS`
pub struct FPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDS_W<'a> {
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
///Reader of field `LPUDS`
pub type LPUDS_R = crate::R<bool, bool>;
///Write proxy for field `LPUDS`
pub struct LPUDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUDS_W<'a> {
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
///Reader of field `MRUDS`
pub type MRUDS_R = crate::R<bool, bool>;
///Write proxy for field `MRUDS`
pub struct MRUDS_W<'a> {
    w: &'a mut W,
}
impl<'a> MRUDS_W<'a> {
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
///Reader of field `ADCDC1`
pub type ADCDC1_R = crate::R<bool, bool>;
///Write proxy for field `ADCDC1`
pub struct ADCDC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCDC1_W<'a> {
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
///Regulator voltage scaling output selection
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VOS_A {
    ///3: Scale 1 mode (reset value)
    SCALE1 = 3,
    ///2: Scale 2 mode
    SCALE2 = 2,
    ///1: Scale 3 mode
    SCALE3 = 1,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
///Reader of field `VOS`
pub type VOS_R = crate::R<u8, VOS_A>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VOS_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(VOS_A::SCALE1),
            2 => Val(VOS_A::SCALE2),
            1 => Val(VOS_A::SCALE3),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `SCALE1`
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == VOS_A::SCALE1
    }
    ///Checks if the value of the field is `SCALE2`
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == VOS_A::SCALE2
    }
    ///Checks if the value of the field is `SCALE3`
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == VOS_A::SCALE3
    }
}
///Write proxy for field `VOS`
pub struct VOS_W<'a> {
    w: &'a mut W,
}
impl<'a> VOS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Scale 1 mode (reset value)
    #[inline(always)]
    pub fn scale1(self) -> &'a mut W {
        self.variant(VOS_A::SCALE1)
    }
    ///Scale 2 mode
    #[inline(always)]
    pub fn scale2(self) -> &'a mut W {
        self.variant(VOS_A::SCALE2)
    }
    ///Scale 3 mode
    #[inline(always)]
    pub fn scale3(self) -> &'a mut W {
        self.variant(VOS_A::SCALE3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Reader of field `ODEN`
pub type ODEN_R = crate::R<bool, bool>;
///Write proxy for field `ODEN`
pub struct ODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODEN_W<'a> {
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
///Reader of field `ODSWEN`
pub type ODSWEN_R = crate::R<bool, bool>;
///Write proxy for field `ODSWEN`
pub struct ODSWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODSWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Reader of field `UDEN`
pub type UDEN_R = crate::R<u8, u8>;
///Write proxy for field `UDEN`
pub struct UDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UDEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Low-power regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn lpuds(&self) -> LPUDS_R {
        LPUDS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Main regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn mruds(&self) -> MRUDS_R {
        MRUDS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 13 - ADCDC1
    #[inline(always)]
    pub fn adcdc1(&self) -> ADCDC1_R {
        ADCDC1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    pub fn odswen(&self) -> ODSWEN_R {
        ODSWEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W {
        LPDS_W { w: self }
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W {
        PDDS_W { w: self }
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W {
        CSBF_W { w: self }
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W {
        DBP_W { w: self }
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W {
        FPDS_W { w: self }
    }
    ///Bit 10 - Low-power regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn lpuds(&mut self) -> LPUDS_W {
        LPUDS_W { w: self }
    }
    ///Bit 11 - Main regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn mruds(&mut self) -> MRUDS_W {
        MRUDS_W { w: self }
    }
    ///Bit 13 - ADCDC1
    #[inline(always)]
    pub fn adcdc1(&mut self) -> ADCDC1_W {
        ADCDC1_W { w: self }
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W {
        VOS_W { w: self }
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W {
        ODEN_W { w: self }
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    pub fn odswen(&mut self) -> ODSWEN_W {
        ODSWEN_W { w: self }
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    pub fn uden(&mut self) -> UDEN_W {
        UDEN_W { w: self }
    }
}
