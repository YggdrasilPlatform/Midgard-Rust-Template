///Reader of register GCR
pub type R = crate::R<u32, super::GCR>;
///Writer for register GCR
pub type W = crate::W<u32, super::GCR>;
///Register GCR `reset()`'s with value 0x2220
impl crate::ResetValue for super::GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2220
    }
}
///Horizontal Synchronization Polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPOL_A {
    ///0: Horizontal synchronization polarity is active low
    ACTIVELOW = 0,
    ///1: Horizontal synchronization polarity is active high
    ACTIVEHIGH = 1,
}
impl From<HSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HSPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HSPOL`
pub type HSPOL_R = crate::R<bool, HSPOL_A>;
impl HSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSPOL_A {
        match self.bits {
            false => HSPOL_A::ACTIVELOW,
            true => HSPOL_A::ACTIVEHIGH,
        }
    }
    ///Checks if the value of the field is `ACTIVELOW`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == HSPOL_A::ACTIVELOW
    }
    ///Checks if the value of the field is `ACTIVEHIGH`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == HSPOL_A::ACTIVEHIGH
    }
}
///Write proxy for field `HSPOL`
pub struct HSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Horizontal synchronization polarity is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(HSPOL_A::ACTIVELOW)
    }
    ///Horizontal synchronization polarity is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(HSPOL_A::ACTIVEHIGH)
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
///Vertical Synchronization Polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSPOL_A {
    ///0: Vertical synchronization polarity is active low
    ACTIVELOW = 0,
    ///1: Vertical synchronization polarity is active high
    ACTIVEHIGH = 1,
}
impl From<VSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: VSPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `VSPOL`
pub type VSPOL_R = crate::R<bool, VSPOL_A>;
impl VSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VSPOL_A {
        match self.bits {
            false => VSPOL_A::ACTIVELOW,
            true => VSPOL_A::ACTIVEHIGH,
        }
    }
    ///Checks if the value of the field is `ACTIVELOW`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == VSPOL_A::ACTIVELOW
    }
    ///Checks if the value of the field is `ACTIVEHIGH`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == VSPOL_A::ACTIVEHIGH
    }
}
///Write proxy for field `VSPOL`
pub struct VSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VSPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Vertical synchronization polarity is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(VSPOL_A::ACTIVELOW)
    }
    ///Vertical synchronization polarity is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(VSPOL_A::ACTIVEHIGH)
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
///Data Enable Polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEPOL_A {
    ///0: Data enable polarity is active low
    ACTIVELOW = 0,
    ///1: Data enable polarity is active high
    ACTIVEHIGH = 1,
}
impl From<DEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DEPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DEPOL`
pub type DEPOL_R = crate::R<bool, DEPOL_A>;
impl DEPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEPOL_A {
        match self.bits {
            false => DEPOL_A::ACTIVELOW,
            true => DEPOL_A::ACTIVEHIGH,
        }
    }
    ///Checks if the value of the field is `ACTIVELOW`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DEPOL_A::ACTIVELOW
    }
    ///Checks if the value of the field is `ACTIVEHIGH`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DEPOL_A::ACTIVEHIGH
    }
}
///Write proxy for field `DEPOL`
pub struct DEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Data enable polarity is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(DEPOL_A::ACTIVELOW)
    }
    ///Data enable polarity is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(DEPOL_A::ACTIVEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
///Pixel Clock Polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCPOL_A {
    ///0: Pixel clock on rising edge
    RISINGEDGE = 0,
    ///1: Pixel clock on falling edge
    FALLINGEDGE = 1,
}
impl From<PCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PCPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PCPOL`
pub type PCPOL_R = crate::R<bool, PCPOL_A>;
impl PCPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCPOL_A {
        match self.bits {
            false => PCPOL_A::RISINGEDGE,
            true => PCPOL_A::FALLINGEDGE,
        }
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PCPOL_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PCPOL_A::FALLINGEDGE
    }
}
///Write proxy for field `PCPOL`
pub struct PCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PCPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Pixel clock on rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(PCPOL_A::RISINGEDGE)
    }
    ///Pixel clock on falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(PCPOL_A::FALLINGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
///Dither Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN_A {
    ///0: Dither disabled
    DISABLED = 0,
    ///1: Dither enabled
    ENABLED = 1,
}
impl From<DEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DEN`
pub type DEN_R = crate::R<bool, DEN_A>;
impl DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEN_A {
        match self.bits {
            false => DEN_A::DISABLED,
            true => DEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEN_A::ENABLED
    }
}
///Write proxy for field `DEN`
pub struct DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Dither disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEN_A::DISABLED)
    }
    ///Dither enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `DRW`
pub type DRW_R = crate::R<u8, u8>;
///Reader of field `DGW`
pub type DGW_R = crate::R<u8, u8>;
///Reader of field `DBW`
pub type DBW_R = crate::R<u8, u8>;
///LCD-TFT controller enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCEN_A {
    ///0: LCD-TFT controller disabled
    DISABLED = 0,
    ///1: LCD-TFT controller enabled
    ENABLED = 1,
}
impl From<LTDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LTDCEN`
pub type LTDCEN_R = crate::R<bool, LTDCEN_A>;
impl LTDCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LTDCEN_A {
        match self.bits {
            false => LTDCEN_A::DISABLED,
            true => LTDCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN_A::ENABLED
    }
}
///Write proxy for field `LTDCEN`
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LTDCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///LCD-TFT controller disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::DISABLED)
    }
    ///LCD-TFT controller enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::ENABLED)
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
    ///Bit 31 - Horizontal Synchronization Polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Vertical Synchronization Polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - Data Enable Polarity
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Pixel Clock Polarity
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 16 - Dither Enable
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 12:14 - Dither Red Width
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bits 8:10 - Dither Green Width
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 4:6 - Dither Blue Width
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 0 - LCD-TFT controller enable bit
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - Horizontal Synchronization Polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W {
        HSPOL_W { w: self }
    }
    ///Bit 30 - Vertical Synchronization Polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W {
        VSPOL_W { w: self }
    }
    ///Bit 29 - Data Enable Polarity
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W {
        DEPOL_W { w: self }
    }
    ///Bit 28 - Pixel Clock Polarity
    #[inline(always)]
    pub fn pcpol(&mut self) -> PCPOL_W {
        PCPOL_W { w: self }
    }
    ///Bit 16 - Dither Enable
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W {
        DEN_W { w: self }
    }
    ///Bit 0 - LCD-TFT controller enable bit
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
    }
}
