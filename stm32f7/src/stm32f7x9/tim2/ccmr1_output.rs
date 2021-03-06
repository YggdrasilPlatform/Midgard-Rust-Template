///Reader of register CCMR1_Output
pub type R = crate::R<u32, super::CCMR1_OUTPUT>;
///Writer for register CCMR1_Output
pub type W = crate::W<u32, super::CCMR1_OUTPUT>;
///Register CCMR1_Output `reset()`'s with value 0
impl crate::ResetValue for super::CCMR1_OUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OC2CE`
pub type OC2CE_R = crate::R<bool, bool>;
///Write proxy for field `OC2CE`
pub struct OC2CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2CE_W<'a> {
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
///OC2M
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OC2M_A {
    ///0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    FROZEN = 0,
    ///1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    ACTIVEONMATCH = 1,
    ///2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    INACTIVEONMATCH = 2,
    ///3: OCyREF toggles when TIMx_CNT=TIMx_CCRy
    TOGGLE = 3,
    ///4: OCyREF is forced low
    FORCEINACTIVE = 4,
    ///5: OCyREF is forced high
    FORCEACTIVE = 5,
    ///6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    PWMMODE1 = 6,
    ///7: Inversely to PwmMode1
    PWMMODE2 = 7,
    ///8: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    OPMMODE1 = 8,
    ///9: Inversely to OpmMode1
    OPMMODE2 = 9,
    ///12: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    COMBINEDPWMMODE1 = 12,
    ///13: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    COMBINEDPWMMODE2 = 13,
    ///14: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    ASYMMETRICPWMMODE1 = 14,
    ///15: OCyREF has the same behavior as in PWM mode 2. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    ASYMMETRICPWMMODE2 = 15,
}
impl From<OC2M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC2M_A) -> Self {
        variant as _
    }
}
///Reader of field `OC2M`
pub type OC2M_R = crate::R<u8, OC2M_A>;
impl OC2M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OC2M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OC2M_A::FROZEN),
            1 => Val(OC2M_A::ACTIVEONMATCH),
            2 => Val(OC2M_A::INACTIVEONMATCH),
            3 => Val(OC2M_A::TOGGLE),
            4 => Val(OC2M_A::FORCEINACTIVE),
            5 => Val(OC2M_A::FORCEACTIVE),
            6 => Val(OC2M_A::PWMMODE1),
            7 => Val(OC2M_A::PWMMODE2),
            8 => Val(OC2M_A::OPMMODE1),
            9 => Val(OC2M_A::OPMMODE2),
            12 => Val(OC2M_A::COMBINEDPWMMODE1),
            13 => Val(OC2M_A::COMBINEDPWMMODE2),
            14 => Val(OC2M_A::ASYMMETRICPWMMODE1),
            15 => Val(OC2M_A::ASYMMETRICPWMMODE2),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `FROZEN`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC2M_A::FROZEN
    }
    ///Checks if the value of the field is `ACTIVEONMATCH`
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC2M_A::ACTIVEONMATCH
    }
    ///Checks if the value of the field is `INACTIVEONMATCH`
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC2M_A::INACTIVEONMATCH
    }
    ///Checks if the value of the field is `TOGGLE`
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC2M_A::TOGGLE
    }
    ///Checks if the value of the field is `FORCEINACTIVE`
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC2M_A::FORCEINACTIVE
    }
    ///Checks if the value of the field is `FORCEACTIVE`
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC2M_A::FORCEACTIVE
    }
    ///Checks if the value of the field is `PWMMODE1`
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC2M_A::PWMMODE1
    }
    ///Checks if the value of the field is `PWMMODE2`
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC2M_A::PWMMODE2
    }
    ///Checks if the value of the field is `OPMMODE1`
    #[inline(always)]
    pub fn is_opm_mode1(&self) -> bool {
        *self == OC2M_A::OPMMODE1
    }
    ///Checks if the value of the field is `OPMMODE2`
    #[inline(always)]
    pub fn is_opm_mode2(&self) -> bool {
        *self == OC2M_A::OPMMODE2
    }
    ///Checks if the value of the field is `COMBINEDPWMMODE1`
    #[inline(always)]
    pub fn is_combined_pwm_mode1(&self) -> bool {
        *self == OC2M_A::COMBINEDPWMMODE1
    }
    ///Checks if the value of the field is `COMBINEDPWMMODE2`
    #[inline(always)]
    pub fn is_combined_pwm_mode2(&self) -> bool {
        *self == OC2M_A::COMBINEDPWMMODE2
    }
    ///Checks if the value of the field is `ASYMMETRICPWMMODE1`
    #[inline(always)]
    pub fn is_asymmetric_pwm_mode1(&self) -> bool {
        *self == OC2M_A::ASYMMETRICPWMMODE1
    }
    ///Checks if the value of the field is `ASYMMETRICPWMMODE2`
    #[inline(always)]
    pub fn is_asymmetric_pwm_mode2(&self) -> bool {
        *self == OC2M_A::ASYMMETRICPWMMODE2
    }
}
///Write proxy for field `OC2M`
pub struct OC2M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2M_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC2M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC2M_A::FROZEN)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::ACTIVEONMATCH)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::INACTIVEONMATCH)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC2M_A::TOGGLE)
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC2M_A::FORCEINACTIVE)
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC2M_A::FORCEACTIVE)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::PWMMODE1)
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::PWMMODE2)
    }
    ///Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn opm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::OPMMODE1)
    }
    ///Inversely to OpmMode1
    #[inline(always)]
    pub fn opm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::OPMMODE2)
    }
    ///OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn combined_pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::COMBINEDPWMMODE1)
    }
    ///OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn combined_pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::COMBINEDPWMMODE2)
    }
    ///OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn asymmetric_pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::ASYMMETRICPWMMODE1)
    }
    ///OCyREF has the same behavior as in PWM mode 2. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn asymmetric_pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::ASYMMETRICPWMMODE2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
///OC2PE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC2PE_A {
    ///0: Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately
    DISABLED = 0,
    ///1: Preload register on CCR2 enabled. Preload value is loaded into active register on each update event
    ENABLED = 1,
}
impl From<OC2PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC2PE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OC2PE`
pub type OC2PE_R = crate::R<bool, OC2PE_A>;
impl OC2PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC2PE_A {
        match self.bits {
            false => OC2PE_A::DISABLED,
            true => OC2PE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC2PE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC2PE_A::ENABLED
    }
}
///Write proxy for field `OC2PE`
pub struct OC2PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC2PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC2PE_A::DISABLED)
    }
    ///Preload register on CCR2 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC2PE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `OC2FE`
pub type OC2FE_R = crate::R<bool, bool>;
///Write proxy for field `OC2FE`
pub struct OC2FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2FE_W<'a> {
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
///CC2S
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC2S_A {
    ///0: CC2 channel is configured as output
    OUTPUT = 0,
}
impl From<CC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC2S_A) -> Self {
        variant as _
    }
}
///Reader of field `CC2S`
pub type CC2S_R = crate::R<u8, CC2S_A>;
impl CC2S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC2S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC2S_A::OUTPUT),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC2S_A::OUTPUT
    }
}
///Write proxy for field `CC2S`
pub struct CC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///CC2 channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC2S_A::OUTPUT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `OC1CE`
pub type OC1CE_R = crate::R<bool, bool>;
///Write proxy for field `OC1CE`
pub struct OC1CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1CE_W<'a> {
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
///OC1M
pub type OC1M_A = OC2M_A;
///Reader of field `OC1M`
pub type OC1M_R = crate::R<u8, OC2M_A>;
///Write proxy for field `OC1M`
pub struct OC1M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1M_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC1M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC2M_A::FROZEN)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::ACTIVEONMATCH)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::INACTIVEONMATCH)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC2M_A::TOGGLE)
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC2M_A::FORCEINACTIVE)
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC2M_A::FORCEACTIVE)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::PWMMODE1)
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::PWMMODE2)
    }
    ///Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn opm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::OPMMODE1)
    }
    ///Inversely to OpmMode1
    #[inline(always)]
    pub fn opm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::OPMMODE2)
    }
    ///OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn combined_pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::COMBINEDPWMMODE1)
    }
    ///OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn combined_pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::COMBINEDPWMMODE2)
    }
    ///OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn asymmetric_pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::ASYMMETRICPWMMODE1)
    }
    ///OCyREF has the same behavior as in PWM mode 2. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn asymmetric_pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::ASYMMETRICPWMMODE2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///OC1PE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1PE_A {
    ///0: Preload register on CCR1 disabled. New values written to CCR1 are taken into account immediately
    DISABLED = 0,
    ///1: Preload register on CCR1 enabled. Preload value is loaded into active register on each update event
    ENABLED = 1,
}
impl From<OC1PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1PE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OC1PE`
pub type OC1PE_R = crate::R<bool, OC1PE_A>;
impl OC1PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC1PE_A {
        match self.bits {
            false => OC1PE_A::DISABLED,
            true => OC1PE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC1PE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC1PE_A::ENABLED
    }
}
///Write proxy for field `OC1PE`
pub struct OC1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC1PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Preload register on CCR1 disabled. New values written to CCR1 are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC1PE_A::DISABLED)
    }
    ///Preload register on CCR1 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC1PE_A::ENABLED)
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
///Reader of field `OC1FE`
pub type OC1FE_R = crate::R<bool, bool>;
///Write proxy for field `OC1FE`
pub struct OC1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1FE_W<'a> {
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
///CC1S
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    ///0: CC1 channel is configured as output
    OUTPUT = 0,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
///Reader of field `CC1S`
pub type CC1S_R = crate::R<u8, CC1S_A>;
impl CC1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC1S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC1S_A::OUTPUT),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC1S_A::OUTPUT
    }
}
///Write proxy for field `CC1S`
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///CC1 channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC1S_A::OUTPUT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bit 15 - OC2CE
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 12:14 - OC2M
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bit 11 - OC2PE
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - OC2FE
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 8:9 - CC2S
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 7 - OC1CE
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 4:6 - OC1M
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - OC1PE
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - OC1FE
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bit 15 - OC2CE
    #[inline(always)]
    pub fn oc2ce(&mut self) -> OC2CE_W {
        OC2CE_W { w: self }
    }
    ///Bits 12:14 - OC2M
    #[inline(always)]
    pub fn oc2m(&mut self) -> OC2M_W {
        OC2M_W { w: self }
    }
    ///Bit 11 - OC2PE
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OC2PE_W {
        OC2PE_W { w: self }
    }
    ///Bit 10 - OC2FE
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OC2FE_W {
        OC2FE_W { w: self }
    }
    ///Bits 8:9 - CC2S
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W { w: self }
    }
    ///Bit 7 - OC1CE
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OC1CE_W {
        OC1CE_W { w: self }
    }
    ///Bits 4:6 - OC1M
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W {
        OC1M_W { w: self }
    }
    ///Bit 3 - OC1PE
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W {
        OC1PE_W { w: self }
    }
    ///Bit 2 - OC1FE
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W {
        OC1FE_W { w: self }
    }
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
}
