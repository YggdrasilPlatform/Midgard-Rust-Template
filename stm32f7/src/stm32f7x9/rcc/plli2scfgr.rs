///Reader of register PLLI2SCFGR
pub type R = crate::R<u32, super::PLLI2SCFGR>;
///Writer for register PLLI2SCFGR
pub type W = crate::W<u32, super::PLLI2SCFGR>;
///Register PLLI2SCFGR `reset()`'s with value 0x2000_3000
impl crate::ResetValue for super::PLLI2SCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_3000
    }
}
///Reader of field `PLLI2SR`
pub type PLLI2SR_R = crate::R<u8, u8>;
///Write proxy for field `PLLI2SR`
pub struct PLLI2SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
///Reader of field `PLLI2SQ`
pub type PLLI2SQ_R = crate::R<u8, u8>;
///Write proxy for field `PLLI2SQ`
pub struct PLLI2SQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SQ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
///Reader of field `PLLI2SN`
pub type PLLI2SN_R = crate::R<u16, u16>;
///Write proxy for field `PLLI2SN`
pub struct PLLI2SN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
///PLLI2S division factor for SPDIFRX clock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLI2SP_A {
    ///0: PLL*P=2
    DIV2 = 0,
    ///1: PLL*P=4
    DIV4 = 1,
    ///2: PLL*P=6
    DIV6 = 2,
    ///3: PLL*P=8
    DIV8 = 3,
}
impl From<PLLI2SP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SP_A) -> Self {
        variant as _
    }
}
///Reader of field `PLLI2SP`
pub type PLLI2SP_R = crate::R<u8, PLLI2SP_A>;
impl PLLI2SP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SP_A {
        match self.bits {
            0 => PLLI2SP_A::DIV2,
            1 => PLLI2SP_A::DIV4,
            2 => PLLI2SP_A::DIV6,
            3 => PLLI2SP_A::DIV8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SP_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SP_A::DIV4
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SP_A::DIV6
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SP_A::DIV8
    }
}
///Write proxy for field `PLLI2SP`
pub struct PLLI2SP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///PLL*P=2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLI2SP_A::DIV2)
    }
    ///PLL*P=4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLI2SP_A::DIV4)
    }
    ///PLL*P=6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLI2SP_A::DIV6)
    }
    ///PLL*P=8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLI2SP_A::DIV8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    ///Bits 28:30 - PLLI2S division factor for I2S clocks
    #[inline(always)]
    pub fn plli2sr(&self) -> PLLI2SR_R {
        PLLI2SR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    ///Bits 24:27 - PLLI2S division factor for SAI1 clock
    #[inline(always)]
    pub fn plli2sq(&self) -> PLLI2SQ_R {
        PLLI2SQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 6:14 - PLLI2S multiplication factor for VCO
    #[inline(always)]
    pub fn plli2sn(&self) -> PLLI2SN_R {
        PLLI2SN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    ///Bits 16:17 - PLLI2S division factor for SPDIFRX clock
    #[inline(always)]
    pub fn plli2sp(&self) -> PLLI2SP_R {
        PLLI2SP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    ///Bits 28:30 - PLLI2S division factor for I2S clocks
    #[inline(always)]
    pub fn plli2sr(&mut self) -> PLLI2SR_W {
        PLLI2SR_W { w: self }
    }
    ///Bits 24:27 - PLLI2S division factor for SAI1 clock
    #[inline(always)]
    pub fn plli2sq(&mut self) -> PLLI2SQ_W {
        PLLI2SQ_W { w: self }
    }
    ///Bits 6:14 - PLLI2S multiplication factor for VCO
    #[inline(always)]
    pub fn plli2sn(&mut self) -> PLLI2SN_W {
        PLLI2SN_W { w: self }
    }
    ///Bits 16:17 - PLLI2S division factor for SPDIFRX clock
    #[inline(always)]
    pub fn plli2sp(&mut self) -> PLLI2SP_W {
        PLLI2SP_W { w: self }
    }
}
