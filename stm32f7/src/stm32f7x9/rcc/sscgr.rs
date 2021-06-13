///Reader of register SSCGR
pub type R = crate::R<u32, super::SSCGR>;
///Writer for register SSCGR
pub type W = crate::W<u32, super::SSCGR>;
///Register SSCGR `reset()`'s with value 0
impl crate::ResetValue for super::SSCGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Spread spectrum modulation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSCGEN_A {
    ///0: Spread spectrum modulation disabled
    DISABLED = 0,
    ///1: Spread spectrum modulation enabled
    ENABLED = 1,
}
impl From<SSCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSCGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SSCGEN`
pub type SSCGEN_R = crate::R<bool, SSCGEN_A>;
impl SSCGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSCGEN_A {
        match self.bits {
            false => SSCGEN_A::DISABLED,
            true => SSCGEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSCGEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSCGEN_A::ENABLED
    }
}
///Write proxy for field `SSCGEN`
pub struct SSCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SSCGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Spread spectrum modulation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSCGEN_A::DISABLED)
    }
    ///Spread spectrum modulation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSCGEN_A::ENABLED)
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
///Spread Select
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPREADSEL_A {
    ///0: Center spread
    CENTER = 0,
    ///1: Down spread
    DOWN = 1,
}
impl From<SPREADSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SPREADSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SPREADSEL`
pub type SPREADSEL_R = crate::R<bool, SPREADSEL_A>;
impl SPREADSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SPREADSEL_A {
        match self.bits {
            false => SPREADSEL_A::CENTER,
            true => SPREADSEL_A::DOWN,
        }
    }
    ///Checks if the value of the field is `CENTER`
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == SPREADSEL_A::CENTER
    }
    ///Checks if the value of the field is `DOWN`
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == SPREADSEL_A::DOWN
    }
}
///Write proxy for field `SPREADSEL`
pub struct SPREADSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPREADSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPREADSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Center spread
    #[inline(always)]
    pub fn center(self) -> &'a mut W {
        self.variant(SPREADSEL_A::CENTER)
    }
    ///Down spread
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(SPREADSEL_A::DOWN)
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
///Reader of field `INCSTEP`
pub type INCSTEP_R = crate::R<u16, u16>;
///Write proxy for field `INCSTEP`
pub struct INCSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INCSTEP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 13)) | (((value as u32) & 0x7fff) << 13);
        self.w
    }
}
///Reader of field `MODPER`
pub type MODPER_R = crate::R<u16, u16>;
///Write proxy for field `MODPER`
pub struct MODPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MODPER_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    ///Bit 31 - Spread spectrum modulation enable
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Spread Select
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bits 13:27 - Incrementation step
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    ///Bits 0:12 - Modulation period
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    ///Bit 31 - Spread spectrum modulation enable
    #[inline(always)]
    pub fn sscgen(&mut self) -> SSCGEN_W {
        SSCGEN_W { w: self }
    }
    ///Bit 30 - Spread Select
    #[inline(always)]
    pub fn spreadsel(&mut self) -> SPREADSEL_W {
        SPREADSEL_W { w: self }
    }
    ///Bits 13:27 - Incrementation step
    #[inline(always)]
    pub fn incstep(&mut self) -> INCSTEP_W {
        INCSTEP_W { w: self }
    }
    ///Bits 0:12 - Modulation period
    #[inline(always)]
    pub fn modper(&mut self) -> MODPER_W {
        MODPER_W { w: self }
    }
}
