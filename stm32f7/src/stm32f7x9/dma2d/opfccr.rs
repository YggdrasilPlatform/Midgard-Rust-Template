///Reader of register OPFCCR
pub type R = crate::R<u32, super::OPFCCR>;
///Writer for register OPFCCR
pub type W = crate::W<u32, super::OPFCCR>;
///Register OPFCCR `reset()`'s with value 0
impl crate::ResetValue for super::OPFCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Color mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
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
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
///Reader of field `CM`
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CM_A::ARGB8888),
            1 => Val(CM_A::RGB888),
            2 => Val(CM_A::RGB565),
            3 => Val(CM_A::ARGB1555),
            4 => Val(CM_A::ARGB4444),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `ARGB8888`
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM_A::ARGB8888
    }
    ///Checks if the value of the field is `RGB888`
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM_A::RGB888
    }
    ///Checks if the value of the field is `RGB565`
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM_A::RGB565
    }
    ///Checks if the value of the field is `ARGB1555`
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM_A::ARGB1555
    }
    ///Checks if the value of the field is `ARGB4444`
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM_A::ARGB4444
    }
}
///Write proxy for field `CM`
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CM_A::ARGB8888)
    }
    ///RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CM_A::RGB888)
    }
    ///RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(CM_A::RGB565)
    }
    ///ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(CM_A::ARGB1555)
    }
    ///ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(CM_A::ARGB4444)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
}
