///Reader of register OR1
pub type R = crate::R<u32, super::OR1>;
///Writer for register OR1
pub type W = crate::W<u32, super::OR1>;
///Register OR1 `reset()`'s with value 0
impl crate::ResetValue for super::OR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TI4_RMP`
pub type TI4_RMP_R = crate::R<u8, u8>;
///Write proxy for field `TI4_RMP`
pub struct TI4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4_RMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `ETR1_RMP`
pub type ETR1_RMP_R = crate::R<bool, bool>;
///Write proxy for field `ETR1_RMP`
pub struct ETR1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR1_RMP_W<'a> {
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
///Reader of field `ITR1_RMP`
pub type ITR1_RMP_R = crate::R<bool, bool>;
///Write proxy for field `ITR1_RMP`
pub struct ITR1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ITR1_RMP_W<'a> {
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
    ///Bits 2:3 - Input Capture 4 remap
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bit 1 - External trigger remap
    #[inline(always)]
    pub fn etr1_rmp(&self) -> ETR1_RMP_R {
        ETR1_RMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Internal trigger 1 remap
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 2:3 - Input Capture 4 remap
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W {
        TI4_RMP_W { w: self }
    }
    ///Bit 1 - External trigger remap
    #[inline(always)]
    pub fn etr1_rmp(&mut self) -> ETR1_RMP_W {
        ETR1_RMP_W { w: self }
    }
    ///Bit 0 - Internal trigger 1 remap
    #[inline(always)]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W {
        ITR1_RMP_W { w: self }
    }
}
