///Reader of register DSI_PCTLR
pub type R = crate::R<u32, super::DSI_PCTLR>;
///Writer for register DSI_PCTLR
pub type W = crate::W<u32, super::DSI_PCTLR>;
///Register DSI_PCTLR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_PCTLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DEN`
pub type DEN_R = crate::R<bool, bool>;
///Write proxy for field `DEN`
pub struct DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN_W<'a> {
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
///Reader of field `CKE`
pub type CKE_R = crate::R<bool, bool>;
///Write proxy for field `CKE`
pub struct CKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKE_W<'a> {
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
impl R {
    ///Bit 1 - Digital Enable
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Clock Enable
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Digital Enable
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W {
        DEN_W { w: self }
    }
    ///Bit 2 - Clock Enable
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W {
        CKE_W { w: self }
    }
}
