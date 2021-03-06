///Reader of register AHB1ENR
pub type R = crate::R<u32, super::AHB1ENR>;
///Writer for register AHB1ENR
pub type W = crate::W<u32, super::AHB1ENR>;
///Register AHB1ENR `reset()`'s with value 0x0010_0000
impl crate::ResetValue for super::AHB1ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0000
    }
}
///USB OTG HSULPI clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGHSULPIEN_A {
    ///0: The selected clock is disabled
    DISABLED = 0,
    ///1: The selected clock is enabled
    ENABLED = 1,
}
impl From<OTGHSULPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGHSULPIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OTGHSULPIEN`
pub type OTGHSULPIEN_R = crate::R<bool, OTGHSULPIEN_A>;
impl OTGHSULPIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OTGHSULPIEN_A {
        match self.bits {
            false => OTGHSULPIEN_A::DISABLED,
            true => OTGHSULPIEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGHSULPIEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGHSULPIEN_A::ENABLED
    }
}
///Write proxy for field `OTGHSULPIEN`
pub struct OTGHSULPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGHSULPIEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OTGHSULPIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///USB OTG HS clock enable
pub type OTGHSEN_A = OTGHSULPIEN_A;
///Reader of field `OTGHSEN`
pub type OTGHSEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `OTGHSEN`
pub struct OTGHSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGHSEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OTGHSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///Ethernet PTP clock enable
pub type ETHMACPTPEN_A = OTGHSULPIEN_A;
///Reader of field `ETHMACPTPEN`
pub type ETHMACPTPEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `ETHMACPTPEN`
pub struct ETHMACPTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACPTPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETHMACPTPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///Ethernet Reception clock enable
pub type ETHMACRXEN_A = OTGHSULPIEN_A;
///Reader of field `ETHMACRXEN`
pub type ETHMACRXEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `ETHMACRXEN`
pub struct ETHMACRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACRXEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETHMACRXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///Ethernet Transmission clock enable
pub type ETHMACTXEN_A = OTGHSULPIEN_A;
///Reader of field `ETHMACTXEN`
pub type ETHMACTXEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `ETHMACTXEN`
pub struct ETHMACTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACTXEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETHMACTXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///Ethernet MAC clock enable
pub type ETHMACEN_A = OTGHSULPIEN_A;
///Reader of field `ETHMACEN`
pub type ETHMACEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `ETHMACEN`
pub struct ETHMACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETHMACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///DMA2D clock enable
pub type DMA2DEN_A = OTGHSULPIEN_A;
///Reader of field `DMA2DEN`
pub type DMA2DEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `DMA2DEN`
pub struct DMA2DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2DEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMA2DEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///DMA2 clock enable
pub type DMA2EN_A = OTGHSULPIEN_A;
///Reader of field `DMA2EN`
pub type DMA2EN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `DMA2EN`
pub struct DMA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMA2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///DMA1 clock enable
pub type DMA1EN_A = OTGHSULPIEN_A;
///Reader of field `DMA1EN`
pub type DMA1EN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `DMA1EN`
pub struct DMA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMA1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///CCM data RAM clock enable
pub type CCMDATARAMEN_A = OTGHSULPIEN_A;
///Reader of field `CCMDATARAMEN`
pub type CCMDATARAMEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `CCMDATARAMEN`
pub struct CCMDATARAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMDATARAMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCMDATARAMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///Backup SRAM interface clock enable
pub type BKPSRAMEN_A = OTGHSULPIEN_A;
///Reader of field `BKPSRAMEN`
pub type BKPSRAMEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `BKPSRAMEN`
pub struct BKPSRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPSRAMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKPSRAMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///CRC clock enable
pub type CRCEN_A = OTGHSULPIEN_A;
///Reader of field `CRCEN`
pub type CRCEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `CRCEN`
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///IO port K clock enable
pub type GPIOKEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOKEN`
pub type GPIOKEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOKEN`
pub struct GPIOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOKEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port J clock enable
pub type GPIOJEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOJEN`
pub type GPIOJEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOJEN`
pub struct GPIOJEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOJEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOJEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port I clock enable
pub type GPIOIEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOIEN`
pub type GPIOIEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOIEN`
pub struct GPIOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOIEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port H clock enable
pub type GPIOHEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOHEN`
pub type GPIOHEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOHEN`
pub struct GPIOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port G clock enable
pub type GPIOGEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOGEN`
pub type GPIOGEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOGEN`
pub struct GPIOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///IO port F clock enable
pub type GPIOFEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOFEN`
pub type GPIOFEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOFEN`
pub struct GPIOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port E clock enable
pub type GPIOEEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOEEN`
pub type GPIOEEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOEEN`
pub struct GPIOEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOEEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port D clock enable
pub type GPIODEN_A = OTGHSULPIEN_A;
///Reader of field `GPIODEN`
pub type GPIODEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIODEN`
pub struct GPIODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIODEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port C clock enable
pub type GPIOCEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOCEN`
pub type GPIOCEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOCEN`
pub struct GPIOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port B clock enable
pub type GPIOBEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOBEN`
pub type GPIOBEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOBEN`
pub struct GPIOBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
///IO port A clock enable
pub type GPIOAEN_A = OTGHSULPIEN_A;
///Reader of field `GPIOAEN`
pub type GPIOAEN_R = crate::R<bool, OTGHSULPIEN_A>;
///Write proxy for field `GPIOAEN`
pub struct GPIOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::DISABLED)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIEN_A::ENABLED)
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
    ///Bit 30 - USB OTG HSULPI clock enable
    #[inline(always)]
    pub fn otghsulpien(&self) -> OTGHSULPIEN_R {
        OTGHSULPIEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - USB OTG HS clock enable
    #[inline(always)]
    pub fn otghsen(&self) -> OTGHSEN_R {
        OTGHSEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Ethernet PTP clock enable
    #[inline(always)]
    pub fn ethmacptpen(&self) -> ETHMACPTPEN_R {
        ETHMACPTPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 27 - Ethernet Reception clock enable
    #[inline(always)]
    pub fn ethmacrxen(&self) -> ETHMACRXEN_R {
        ETHMACRXEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - Ethernet Transmission clock enable
    #[inline(always)]
    pub fn ethmactxen(&self) -> ETHMACTXEN_R {
        ETHMACTXEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - Ethernet MAC clock enable
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 23 - DMA2D clock enable
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - CCM data RAM clock enable
    #[inline(always)]
    pub fn ccmdataramen(&self) -> CCMDATARAMEN_R {
        CCMDATARAMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 18 - Backup SRAM interface clock enable
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 10 - IO port K clock enable
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - IO port J clock enable
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 30 - USB OTG HSULPI clock enable
    #[inline(always)]
    pub fn otghsulpien(&mut self) -> OTGHSULPIEN_W {
        OTGHSULPIEN_W { w: self }
    }
    ///Bit 29 - USB OTG HS clock enable
    #[inline(always)]
    pub fn otghsen(&mut self) -> OTGHSEN_W {
        OTGHSEN_W { w: self }
    }
    ///Bit 28 - Ethernet PTP clock enable
    #[inline(always)]
    pub fn ethmacptpen(&mut self) -> ETHMACPTPEN_W {
        ETHMACPTPEN_W { w: self }
    }
    ///Bit 27 - Ethernet Reception clock enable
    #[inline(always)]
    pub fn ethmacrxen(&mut self) -> ETHMACRXEN_W {
        ETHMACRXEN_W { w: self }
    }
    ///Bit 26 - Ethernet Transmission clock enable
    #[inline(always)]
    pub fn ethmactxen(&mut self) -> ETHMACTXEN_W {
        ETHMACTXEN_W { w: self }
    }
    ///Bit 25 - Ethernet MAC clock enable
    #[inline(always)]
    pub fn ethmacen(&mut self) -> ETHMACEN_W {
        ETHMACEN_W { w: self }
    }
    ///Bit 23 - DMA2D clock enable
    #[inline(always)]
    pub fn dma2den(&mut self) -> DMA2DEN_W {
        DMA2DEN_W { w: self }
    }
    ///Bit 22 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W { w: self }
    }
    ///Bit 21 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W { w: self }
    }
    ///Bit 20 - CCM data RAM clock enable
    #[inline(always)]
    pub fn ccmdataramen(&mut self) -> CCMDATARAMEN_W {
        CCMDATARAMEN_W { w: self }
    }
    ///Bit 18 - Backup SRAM interface clock enable
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W {
        BKPSRAMEN_W { w: self }
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    ///Bit 10 - IO port K clock enable
    #[inline(always)]
    pub fn gpioken(&mut self) -> GPIOKEN_W {
        GPIOKEN_W { w: self }
    }
    ///Bit 9 - IO port J clock enable
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GPIOJEN_W {
        GPIOJEN_W { w: self }
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W {
        GPIOIEN_W { w: self }
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W {
        GPIOHEN_W { w: self }
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W {
        GPIOGEN_W { w: self }
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W {
        GPIOFEN_W { w: self }
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W {
        GPIOEEN_W { w: self }
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W {
        GPIODEN_W { w: self }
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W {
        GPIOCEN_W { w: self }
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W {
        GPIOBEN_W { w: self }
    }
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W {
        GPIOAEN_W { w: self }
    }
}
