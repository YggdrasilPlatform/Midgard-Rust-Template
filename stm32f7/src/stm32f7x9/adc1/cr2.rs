///Reader of register CR2
pub type R = crate::R<u32, super::CR2>;
///Writer for register CR2
pub type W = crate::W<u32, super::CR2>;
///Register CR2 `reset()`'s with value 0
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Start conversion of regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSTART_A {
    ///1: Starts conversion of regular channels
    START = 1,
}
impl From<SWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: SWSTART_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SWSTART`
pub type SWSTART_R = crate::R<bool, SWSTART_A>;
impl SWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SWSTART_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SWSTART_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SWSTART_A::START
    }
}
///Write proxy for field `SWSTART`
pub struct SWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSTART_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Starts conversion of regular channels
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTART_A::START)
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
///External trigger enable for regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEN_A {
    ///0: Trigger detection disabled
    DISABLED = 0,
    ///1: Trigger detection on the rising edge
    RISINGEDGE = 1,
    ///2: Trigger detection on the falling edge
    FALLINGEDGE = 2,
    ///3: Trigger detection on both the rising and falling edges
    BOTHEDGES = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
///Reader of field `EXTEN`
pub type EXTEN_R = crate::R<u8, EXTEN_A>;
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::DISABLED,
            1 => EXTEN_A::RISINGEDGE,
            2 => EXTEN_A::FALLINGEDGE,
            3 => EXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN_A::DISABLED
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN_A::FALLINGEDGE
    }
    ///Checks if the value of the field is `BOTHEDGES`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN_A::BOTHEDGES
    }
}
///Write proxy for field `EXTEN`
pub struct EXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::DISABLED)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RISINGEDGE)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FALLINGEDGE)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BOTHEDGES)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///External event select for regular group
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTSEL_A {
    ///0: Timer 1 CC1 event
    TIM1CC1 = 0,
    ///1: Timer 1 CC2 event
    TIM1CC2 = 1,
    ///2: Timer 1 CC3 event
    TIM1CC3 = 2,
    ///3: Timer 2 CC2 event
    TIM2CC2 = 3,
    ///4: Timer 2 CC3 event
    TIM2CC3 = 4,
    ///5: Timer 2 CC4 event
    TIM2CC4 = 5,
    ///6: Timer 2 TRGO event
    TIM2TRGO = 6,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
///Reader of field `EXTSEL`
pub type EXTSEL_R = crate::R<u8, EXTSEL_A>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTSEL_A::TIM1CC1),
            1 => Val(EXTSEL_A::TIM1CC2),
            2 => Val(EXTSEL_A::TIM1CC3),
            3 => Val(EXTSEL_A::TIM2CC2),
            4 => Val(EXTSEL_A::TIM2CC3),
            5 => Val(EXTSEL_A::TIM2CC4),
            6 => Val(EXTSEL_A::TIM2TRGO),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `TIM1CC1`
    #[inline(always)]
    pub fn is_tim1cc1(&self) -> bool {
        *self == EXTSEL_A::TIM1CC1
    }
    ///Checks if the value of the field is `TIM1CC2`
    #[inline(always)]
    pub fn is_tim1cc2(&self) -> bool {
        *self == EXTSEL_A::TIM1CC2
    }
    ///Checks if the value of the field is `TIM1CC3`
    #[inline(always)]
    pub fn is_tim1cc3(&self) -> bool {
        *self == EXTSEL_A::TIM1CC3
    }
    ///Checks if the value of the field is `TIM2CC2`
    #[inline(always)]
    pub fn is_tim2cc2(&self) -> bool {
        *self == EXTSEL_A::TIM2CC2
    }
    ///Checks if the value of the field is `TIM2CC3`
    #[inline(always)]
    pub fn is_tim2cc3(&self) -> bool {
        *self == EXTSEL_A::TIM2CC3
    }
    ///Checks if the value of the field is `TIM2CC4`
    #[inline(always)]
    pub fn is_tim2cc4(&self) -> bool {
        *self == EXTSEL_A::TIM2CC4
    }
    ///Checks if the value of the field is `TIM2TRGO`
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == EXTSEL_A::TIM2TRGO
    }
}
///Write proxy for field `EXTSEL`
pub struct EXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn tim1cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC1)
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn tim1cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC2)
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn tim1cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC3)
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn tim2cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2CC2)
    }
    ///Timer 2 CC3 event
    #[inline(always)]
    pub fn tim2cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2CC3)
    }
    ///Timer 2 CC4 event
    #[inline(always)]
    pub fn tim2cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2CC4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2TRGO)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
///Start conversion of injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSWSTART_A {
    ///1: Starts conversion of injected channels
    START = 1,
}
impl From<JSWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTART_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JSWSTART`
pub type JSWSTART_R = crate::R<bool, JSWSTART_A>;
impl JSWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, JSWSTART_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(JSWSTART_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == JSWSTART_A::START
    }
}
///Write proxy for field `JSWSTART`
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JSWSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Starts conversion of injected channels
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTART_A::START)
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
///External trigger enable for injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTEN_A {
    ///0: Trigger detection disabled
    DISABLED = 0,
    ///1: Trigger detection on the rising edge
    RISINGEDGE = 1,
    ///2: Trigger detection on the falling edge
    FALLINGEDGE = 2,
    ///3: Trigger detection on both the rising and falling edges
    BOTHEDGES = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
///Reader of field `JEXTEN`
pub type JEXTEN_R = crate::R<u8, JEXTEN_A>;
impl JEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::DISABLED,
            1 => JEXTEN_A::RISINGEDGE,
            2 => JEXTEN_A::FALLINGEDGE,
            3 => JEXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN_A::DISABLED
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN_A::FALLINGEDGE
    }
    ///Checks if the value of the field is `BOTHEDGES`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN_A::BOTHEDGES
    }
}
///Write proxy for field `JEXTEN`
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JEXTEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTEN_A::DISABLED)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::RISINGEDGE)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::FALLINGEDGE)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTEN_A::BOTHEDGES)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
///External event select for injected group
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    ///0: Timer 1 TRGO event
    TIM1TRGO = 0,
    ///1: Timer 1 CC4 event
    TIM1CC4 = 1,
    ///2: Timer 2 TRGO event
    TIM2TRGO = 2,
    ///3: Timer 2 CC1 event
    TIM2CC1 = 3,
    ///4: Timer 3 CC4 event
    TIM3CC4 = 4,
    ///5: Timer 4 TRGO event
    TIM4TRGO = 5,
    ///7: Timer 8 CC4 event
    TIM8CC4 = 7,
    ///8: Timer 1 TRGO(2) event
    TIM1TRGO2 = 8,
    ///9: Timer 8 TRGO event
    TIM8TRGO = 9,
    ///10: Timer 8 TRGO(2) event
    TIM8TRGO2 = 10,
    ///11: Timer 3 CC3 event
    TIM3CC3 = 11,
    ///12: Timer 5 TRGO event
    TIM5TRGO = 12,
    ///13: Timer 3 CC1 event
    TIM3CC1 = 13,
    ///14: Timer 6 TRGO event
    TIM6TRGO = 14,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
///Reader of field `JEXTSEL`
pub type JEXTSEL_R = crate::R<u8, JEXTSEL_A>;
impl JEXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, JEXTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(JEXTSEL_A::TIM1TRGO),
            1 => Val(JEXTSEL_A::TIM1CC4),
            2 => Val(JEXTSEL_A::TIM2TRGO),
            3 => Val(JEXTSEL_A::TIM2CC1),
            4 => Val(JEXTSEL_A::TIM3CC4),
            5 => Val(JEXTSEL_A::TIM4TRGO),
            7 => Val(JEXTSEL_A::TIM8CC4),
            8 => Val(JEXTSEL_A::TIM1TRGO2),
            9 => Val(JEXTSEL_A::TIM8TRGO),
            10 => Val(JEXTSEL_A::TIM8TRGO2),
            11 => Val(JEXTSEL_A::TIM3CC3),
            12 => Val(JEXTSEL_A::TIM5TRGO),
            13 => Val(JEXTSEL_A::TIM3CC1),
            14 => Val(JEXTSEL_A::TIM6TRGO),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `TIM1TRGO`
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM1TRGO
    }
    ///Checks if the value of the field is `TIM1CC4`
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSEL_A::TIM1CC4
    }
    ///Checks if the value of the field is `TIM2TRGO`
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM2TRGO
    }
    ///Checks if the value of the field is `TIM2CC1`
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        *self == JEXTSEL_A::TIM2CC1
    }
    ///Checks if the value of the field is `TIM3CC4`
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        *self == JEXTSEL_A::TIM3CC4
    }
    ///Checks if the value of the field is `TIM4TRGO`
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM4TRGO
    }
    ///Checks if the value of the field is `TIM8CC4`
    #[inline(always)]
    pub fn is_tim8cc4(&self) -> bool {
        *self == JEXTSEL_A::TIM8CC4
    }
    ///Checks if the value of the field is `TIM1TRGO2`
    #[inline(always)]
    pub fn is_tim1trgo2(&self) -> bool {
        *self == JEXTSEL_A::TIM1TRGO2
    }
    ///Checks if the value of the field is `TIM8TRGO`
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM8TRGO
    }
    ///Checks if the value of the field is `TIM8TRGO2`
    #[inline(always)]
    pub fn is_tim8trgo2(&self) -> bool {
        *self == JEXTSEL_A::TIM8TRGO2
    }
    ///Checks if the value of the field is `TIM3CC3`
    #[inline(always)]
    pub fn is_tim3cc3(&self) -> bool {
        *self == JEXTSEL_A::TIM3CC3
    }
    ///Checks if the value of the field is `TIM5TRGO`
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM5TRGO
    }
    ///Checks if the value of the field is `TIM3CC1`
    #[inline(always)]
    pub fn is_tim3cc1(&self) -> bool {
        *self == JEXTSEL_A::TIM3CC1
    }
    ///Checks if the value of the field is `TIM6TRGO`
    #[inline(always)]
    pub fn is_tim6trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM6TRGO
    }
}
///Write proxy for field `JEXTSEL`
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JEXTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1TRGO)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1CC4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2TRGO)
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2CC1)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3CC4)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM4TRGO)
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn tim8cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM8CC4)
    }
    ///Timer 1 TRGO(2) event
    #[inline(always)]
    pub fn tim1trgo2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1TRGO2)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM8TRGO)
    }
    ///Timer 8 TRGO(2) event
    #[inline(always)]
    pub fn tim8trgo2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM8TRGO2)
    }
    ///Timer 3 CC3 event
    #[inline(always)]
    pub fn tim3cc3(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3CC3)
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM5TRGO)
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn tim3cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3CC1)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM6TRGO)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
///Data alignment
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_A {
    ///0: Right alignment
    RIGHT = 0,
    ///1: Left alignment
    LEFT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ALIGN`
pub type ALIGN_R = crate::R<bool, ALIGN_A>;
impl ALIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::RIGHT,
            true => ALIGN_A::LEFT,
        }
    }
    ///Checks if the value of the field is `RIGHT`
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::RIGHT
    }
    ///Checks if the value of the field is `LEFT`
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::LEFT
    }
}
///Write proxy for field `ALIGN`
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Right alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
    }
    ///Left alignment
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
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
///End of conversion selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCS_A {
    ///0: The EOC bit is set at the end of each sequence of regular conversions
    EACHSEQUENCE = 0,
    ///1: The EOC bit is set at the end of each regular conversion
    EACHCONVERSION = 1,
}
impl From<EOCS_A> for bool {
    #[inline(always)]
    fn from(variant: EOCS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOCS`
pub type EOCS_R = crate::R<bool, EOCS_A>;
impl EOCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCS_A {
        match self.bits {
            false => EOCS_A::EACHSEQUENCE,
            true => EOCS_A::EACHCONVERSION,
        }
    }
    ///Checks if the value of the field is `EACHSEQUENCE`
    #[inline(always)]
    pub fn is_each_sequence(&self) -> bool {
        *self == EOCS_A::EACHSEQUENCE
    }
    ///Checks if the value of the field is `EACHCONVERSION`
    #[inline(always)]
    pub fn is_each_conversion(&self) -> bool {
        *self == EOCS_A::EACHCONVERSION
    }
}
///Write proxy for field `EOCS`
pub struct EOCS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The EOC bit is set at the end of each sequence of regular conversions
    #[inline(always)]
    pub fn each_sequence(self) -> &'a mut W {
        self.variant(EOCS_A::EACHSEQUENCE)
    }
    ///The EOC bit is set at the end of each regular conversion
    #[inline(always)]
    pub fn each_conversion(self) -> &'a mut W {
        self.variant(EOCS_A::EACHCONVERSION)
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
///DMA disable selection (for single ADC mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDS_A {
    ///0: No new DMA request is issued after the last transfer
    SINGLE = 0,
    ///1: DMA requests are issued as long as data are converted and DMA=1
    CONTINUOUS = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DDS`
pub type DDS_R = crate::R<bool, DDS_A>;
impl DDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::SINGLE,
            true => DDS_A::CONTINUOUS,
        }
    }
    ///Checks if the value of the field is `SINGLE`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS_A::SINGLE
    }
    ///Checks if the value of the field is `CONTINUOUS`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS_A::CONTINUOUS
    }
}
///Write proxy for field `DDS`
pub struct DDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DDS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No new DMA request is issued after the last transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDS_A::SINGLE)
    }
    ///DMA requests are issued as long as data are converted and DMA=1
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDS_A::CONTINUOUS)
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
///Direct memory access mode (for single ADC mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    ///0: DMA mode disabled
    DISABLED = 0,
    ///1: DMA mode enabled
    ENABLED = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DMA`
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DISABLED,
            true => DMA_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_A::ENABLED
    }
}
///Write proxy for field `DMA`
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::DISABLED)
    }
    ///DMA mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_A::ENABLED)
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
///Continuous conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    ///0: Single conversion mode
    SINGLE = 0,
    ///1: Continuous conversion mode
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CONT`
pub type CONT_R = crate::R<bool, CONT_A>;
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLE,
            true => CONT_A::CONTINUOUS,
        }
    }
    ///Checks if the value of the field is `SINGLE`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT_A::SINGLE
    }
    ///Checks if the value of the field is `CONTINUOUS`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::CONTINUOUS
    }
}
///Write proxy for field `CONT`
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::SINGLE)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
///A/D Converter ON / OFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADON_A {
    ///0: Disable ADC conversion and go to power down mode
    DISABLED = 0,
    ///1: Enable ADC
    ENABLED = 1,
}
impl From<ADON_A> for bool {
    #[inline(always)]
    fn from(variant: ADON_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADON`
pub type ADON_R = crate::R<bool, ADON_A>;
impl ADON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADON_A {
        match self.bits {
            false => ADON_A::DISABLED,
            true => ADON_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADON_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADON_A::ENABLED
    }
}
///Write proxy for field `ADON`
pub struct ADON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Disable ADC conversion and go to power down mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADON_A::DISABLED)
    }
    ///Enable ADC
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADON_A::ENABLED)
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
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&self) -> EOCS_R {
        EOCS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - DMA disable selection (for single ADC mode)
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Direct memory access mode (for single ADC mode)
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W {
        SWSTART_W { w: self }
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W {
        EXTEN_W { w: self }
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W { w: self }
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&mut self) -> EOCS_W {
        EOCS_W { w: self }
    }
    ///Bit 9 - DMA disable selection (for single ADC mode)
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W {
        DDS_W { w: self }
    }
    ///Bit 8 - Direct memory access mode (for single ADC mode)
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W {
        ADON_W { w: self }
    }
}
