///Writer for register SWTRIGR
pub type W = crate::W<u32, super::SWTRIGR>;
///Register SWTRIGR `reset()`'s with value 0
impl crate::ResetValue for super::SWTRIGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///DAC channel2 software trigger
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG2_AW {
    ///0: DAC channel X software trigger disabled
    DISABLED = 0,
    ///1: DAC channel X software trigger enabled
    ENABLED = 1,
}
impl From<SWTRIG2_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG2_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `SWTRIG2`
pub struct SWTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DAC channel X software trigger disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::DISABLED)
    }
    ///DAC channel X software trigger enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::ENABLED)
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
///DAC channel1 software trigger
pub type SWTRIG1_AW = SWTRIG2_AW;
///Write proxy for field `SWTRIG1`
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DAC channel X software trigger disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::DISABLED)
    }
    ///DAC channel X software trigger enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::ENABLED)
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
impl W {
    ///Bit 1 - DAC channel2 software trigger
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W { w: self }
    }
    ///Bit 0 - DAC channel1 software trigger
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
}
