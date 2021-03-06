///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    ///0: No overrun occurred
    NOOVERRUN = 0,
    ///1: Overrun occurred
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVR`
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NOOVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    ///Checks if the value of the field is `NOOVERRUN`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NOOVERRUN
    }
    ///Checks if the value of the field is `OVERRUN`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::OVERRUN
    }
}
///Write proxy for field `OVR`
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVR_A::NOOVERRUN)
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVR_A::OVERRUN)
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
///Regular channel start flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_A {
    ///0: No regular channel conversion started
    NOTSTARTED = 0,
    ///1: Regular channel conversion has started
    STARTED = 1,
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
    pub fn variant(&self) -> STRT_A {
        match self.bits {
            false => STRT_A::NOTSTARTED,
            true => STRT_A::STARTED,
        }
    }
    ///Checks if the value of the field is `NOTSTARTED`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT_A::NOTSTARTED
    }
    ///Checks if the value of the field is `STARTED`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT_A::STARTED
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
    ///No regular channel conversion started
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(STRT_A::NOTSTARTED)
    }
    ///Regular channel conversion has started
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(STRT_A::STARTED)
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
///Injected channel start flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT_A {
    ///0: No injected channel conversion started
    NOTSTARTED = 0,
    ///1: Injected channel conversion has started
    STARTED = 1,
}
impl From<JSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JSTRT`
pub type JSTRT_R = crate::R<bool, JSTRT_A>;
impl JSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JSTRT_A {
        match self.bits {
            false => JSTRT_A::NOTSTARTED,
            true => JSTRT_A::STARTED,
        }
    }
    ///Checks if the value of the field is `NOTSTARTED`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT_A::NOTSTARTED
    }
    ///Checks if the value of the field is `STARTED`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT_A::STARTED
    }
}
///Write proxy for field `JSTRT`
pub struct JSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> JSTRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JSTRT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No injected channel conversion started
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(JSTRT_A::NOTSTARTED)
    }
    ///Injected channel conversion has started
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(JSTRT_A::STARTED)
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
///Injected channel end of conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC_A {
    ///0: Conversion is not complete
    NOTCOMPLETE = 0,
    ///1: Conversion complete
    COMPLETE = 1,
}
impl From<JEOC_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JEOC`
pub type JEOC_R = crate::R<bool, JEOC_A>;
impl JEOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOC_A {
        match self.bits {
            false => JEOC_A::NOTCOMPLETE,
            true => JEOC_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_A::COMPLETE
    }
}
///Write proxy for field `JEOC`
pub struct JEOC_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JEOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Conversion is not complete
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(JEOC_A::NOTCOMPLETE)
    }
    ///Conversion complete
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(JEOC_A::COMPLETE)
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
///Regular channel end of conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    ///0: Conversion is not complete
    NOTCOMPLETE = 0,
    ///1: Conversion complete
    COMPLETE = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOC`
pub type EOC_R = crate::R<bool, EOC_A>;
impl EOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NOTCOMPLETE,
            true => EOC_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_A::COMPLETE
    }
}
///Write proxy for field `EOC`
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Conversion is not complete
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(EOC_A::NOTCOMPLETE)
    }
    ///Conversion complete
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(EOC_A::COMPLETE)
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
///Analog watchdog flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD_A {
    ///0: No analog watchdog event occurred
    NOEVENT = 0,
    ///1: Analog watchdog event occurred
    EVENT = 1,
}
impl From<AWD_A> for bool {
    #[inline(always)]
    fn from(variant: AWD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AWD`
pub type AWD_R = crate::R<bool, AWD_A>;
impl AWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD_A {
        match self.bits {
            false => AWD_A::NOEVENT,
            true => AWD_A::EVENT,
        }
    }
    ///Checks if the value of the field is `NOEVENT`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD_A::NOEVENT
    }
    ///Checks if the value of the field is `EVENT`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD_A::EVENT
    }
}
///Write proxy for field `AWD`
pub struct AWD_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No analog watchdog event occurred
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(AWD_A::NOEVENT)
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(AWD_A::EVENT)
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
    ///Bit 5 - Overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 5 - Overrun
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W {
        STRT_W { w: self }
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&mut self) -> JSTRT_W {
        JSTRT_W { w: self }
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W {
        JEOC_W { w: self }
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W {
        AWD_W { w: self }
    }
}
