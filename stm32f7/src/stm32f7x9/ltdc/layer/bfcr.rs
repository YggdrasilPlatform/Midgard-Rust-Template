///Reader of register BFCR
pub type R = crate::R<u32, super::BFCR>;
///Writer for register BFCR
pub type W = crate::W<u32, super::BFCR>;
///Register BFCR `reset()`'s with value 0x0607
impl crate::ResetValue for super::BFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0607
    }
}
///Blending Factor 1
///
///Value on reset: 6
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BF1_A {
    ///4: BF1 = constant alpha
    CONSTANT = 4,
    ///6: BF1 = pixel alpha * constant alpha
    PIXEL = 6,
}
impl From<BF1_A> for u8 {
    #[inline(always)]
    fn from(variant: BF1_A) -> Self {
        variant as _
    }
}
///Reader of field `BF1`
pub type BF1_R = crate::R<u8, BF1_A>;
impl BF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BF1_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(BF1_A::CONSTANT),
            6 => Val(BF1_A::PIXEL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `CONSTANT`
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF1_A::CONSTANT
    }
    ///Checks if the value of the field is `PIXEL`
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF1_A::PIXEL
    }
}
///Write proxy for field `BF1`
pub struct BF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BF1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BF1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///BF1 = constant alpha
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(BF1_A::CONSTANT)
    }
    ///BF1 = pixel alpha * constant alpha
    #[inline(always)]
    pub fn pixel(self) -> &'a mut W {
        self.variant(BF1_A::PIXEL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
///Blending Factor 2
///
///Value on reset: 7
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BF2_A {
    ///5: BF2 = 1 - constant alpha
    CONSTANT = 5,
    ///7: BF2 = 1 - pixel alpha * constant alpha
    PIXEL = 7,
}
impl From<BF2_A> for u8 {
    #[inline(always)]
    fn from(variant: BF2_A) -> Self {
        variant as _
    }
}
///Reader of field `BF2`
pub type BF2_R = crate::R<u8, BF2_A>;
impl BF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BF2_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(BF2_A::CONSTANT),
            7 => Val(BF2_A::PIXEL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `CONSTANT`
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF2_A::CONSTANT
    }
    ///Checks if the value of the field is `PIXEL`
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF2_A::PIXEL
    }
}
///Write proxy for field `BF2`
pub struct BF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BF2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///BF2 = 1 - constant alpha
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(BF2_A::CONSTANT)
    }
    ///BF2 = 1 - pixel alpha * constant alpha
    #[inline(always)]
    pub fn pixel(self) -> &'a mut W {
        self.variant(BF2_A::PIXEL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 8:10 - Blending Factor 1
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 0:2 - Blending Factor 2
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 8:10 - Blending Factor 1
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W {
        BF1_W { w: self }
    }
    ///Bits 0:2 - Blending Factor 2
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W {
        BF2_W { w: self }
    }
}
