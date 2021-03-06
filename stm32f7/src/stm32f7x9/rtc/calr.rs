///Reader of register CALR
pub type R = crate::R<u32, super::CALR>;
///Writer for register CALR
pub type W = crate::W<u32, super::CALR>;
///Register CALR `reset()`'s with value 0
impl crate::ResetValue for super::CALR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Increase frequency of RTC by 488.5 ppm
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALP_A {
    ///0: No RTCCLK pulses are added
    NOCHANGE = 0,
    ///1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    INCREASEFREQ = 1,
}
impl From<CALP_A> for bool {
    #[inline(always)]
    fn from(variant: CALP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CALP`
pub type CALP_R = crate::R<bool, CALP_A>;
impl CALP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALP_A {
        match self.bits {
            false => CALP_A::NOCHANGE,
            true => CALP_A::INCREASEFREQ,
        }
    }
    ///Checks if the value of the field is `NOCHANGE`
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CALP_A::NOCHANGE
    }
    ///Checks if the value of the field is `INCREASEFREQ`
    #[inline(always)]
    pub fn is_increase_freq(&self) -> bool {
        *self == CALP_A::INCREASEFREQ
    }
}
///Write proxy for field `CALP`
pub struct CALP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No RTCCLK pulses are added
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CALP_A::NOCHANGE)
    }
    ///One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    #[inline(always)]
    pub fn increase_freq(self) -> &'a mut W {
        self.variant(CALP_A::INCREASEFREQ)
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
///Use an 8-second calibration cycle period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALW8_A {
    ///1: When CALW8 is set to ???1???, the 8-second calibration cycle period is selected
    EIGHT_SECOND = 1,
}
impl From<CALW8_A> for bool {
    #[inline(always)]
    fn from(variant: CALW8_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CALW8`
pub type CALW8_R = crate::R<bool, CALW8_A>;
impl CALW8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CALW8_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CALW8_A::EIGHT_SECOND),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `EIGHT_SECOND`
    #[inline(always)]
    pub fn is_eight_second(&self) -> bool {
        *self == CALW8_A::EIGHT_SECOND
    }
}
///Write proxy for field `CALW8`
pub struct CALW8_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALW8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///When CALW8 is set to ???1???, the 8-second calibration cycle period is selected
    #[inline(always)]
    pub fn eight_second(self) -> &'a mut W {
        self.variant(CALW8_A::EIGHT_SECOND)
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
///Use a 16-second calibration cycle period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALW16_A {
    ///1: When CALW16 is set to ???1???, the 16-second calibration cycle period is selected.This bit must not be set to ???1??? if CALW8=1
    SIXTEEN_SECOND = 1,
}
impl From<CALW16_A> for bool {
    #[inline(always)]
    fn from(variant: CALW16_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CALW16`
pub type CALW16_R = crate::R<bool, CALW16_A>;
impl CALW16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CALW16_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CALW16_A::SIXTEEN_SECOND),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `SIXTEEN_SECOND`
    #[inline(always)]
    pub fn is_sixteen_second(&self) -> bool {
        *self == CALW16_A::SIXTEEN_SECOND
    }
}
///Write proxy for field `CALW16`
pub struct CALW16_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALW16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///When CALW16 is set to ???1???, the 16-second calibration cycle period is selected.This bit must not be set to ???1??? if CALW8=1
    #[inline(always)]
    pub fn sixteen_second(self) -> &'a mut W {
        self.variant(CALW16_A::SIXTEEN_SECOND)
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
///Reader of field `CALM`
pub type CALM_R = crate::R<u16, u16>;
///Write proxy for field `CALM`
pub struct CALM_W<'a> {
    w: &'a mut W,
}
impl<'a> CALM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    ///Bit 15 - Increase frequency of RTC by 488.5 ppm
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bit 15 - Increase frequency of RTC by 488.5 ppm
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W {
        CALP_W { w: self }
    }
    ///Bit 14 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W {
        CALW8_W { w: self }
    }
    ///Bit 13 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W {
        CALW16_W { w: self }
    }
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W {
        CALM_W { w: self }
    }
}
