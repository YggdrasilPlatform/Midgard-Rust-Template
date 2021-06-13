///Reader of register PFCR
pub type R = crate::R<u32, super::PFCR>;
///Writer for register PFCR
pub type W = crate::W<u32, super::PFCR>;
///Register PFCR `reset()`'s with value 0
impl crate::ResetValue for super::PFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Pixel Format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PF_A {
    ///0: ARGB8888
    ARGB8888 = 0,
    ///1: RGB888
    RGB888 = 1,
    ///2: RGB565
    RGB565 = 2,
    ///3: ARGB1555
    ARGB1555 = 3,
    ///4: ARGB4444
    ARGB4444 = 4,
    ///5: L8 (8-bit luminance)
    L8 = 5,
    ///6: AL44 (4-bit alpha, 4-bit luminance)
    AL44 = 6,
    ///7: AL88 (8-bit alpha, 8-bit luminance)
    AL88 = 7,
}
impl From<PF_A> for u8 {
    #[inline(always)]
    fn from(variant: PF_A) -> Self {
        variant as _
    }
}
///Reader of field `PF`
pub type PF_R = crate::R<u8, PF_A>;
impl PF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PF_A {
        match self.bits {
            0 => PF_A::ARGB8888,
            1 => PF_A::RGB888,
            2 => PF_A::RGB565,
            3 => PF_A::ARGB1555,
            4 => PF_A::ARGB4444,
            5 => PF_A::L8,
            6 => PF_A::AL44,
            7 => PF_A::AL88,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `ARGB8888`
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == PF_A::ARGB8888
    }
    ///Checks if the value of the field is `RGB888`
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == PF_A::RGB888
    }
    ///Checks if the value of the field is `RGB565`
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == PF_A::RGB565
    }
    ///Checks if the value of the field is `ARGB1555`
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == PF_A::ARGB1555
    }
    ///Checks if the value of the field is `ARGB4444`
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == PF_A::ARGB4444
    }
    ///Checks if the value of the field is `L8`
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == PF_A::L8
    }
    ///Checks if the value of the field is `AL44`
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == PF_A::AL44
    }
    ///Checks if the value of the field is `AL88`
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == PF_A::AL88
    }
}
///Write proxy for field `PF`
pub struct PF_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(PF_A::ARGB8888)
    }
    ///RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(PF_A::RGB888)
    }
    ///RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(PF_A::RGB565)
    }
    ///ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(PF_A::ARGB1555)
    }
    ///ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(PF_A::ARGB4444)
    }
    ///L8 (8-bit luminance)
    #[inline(always)]
    pub fn l8(self) -> &'a mut W {
        self.variant(PF_A::L8)
    }
    ///AL44 (4-bit alpha, 4-bit luminance)
    #[inline(always)]
    pub fn al44(self) -> &'a mut W {
        self.variant(PF_A::AL44)
    }
    ///AL88 (8-bit alpha, 8-bit luminance)
    #[inline(always)]
    pub fn al88(self) -> &'a mut W {
        self.variant(PF_A::AL88)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Pixel Format
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Pixel Format
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W {
        PF_W { w: self }
    }
}
