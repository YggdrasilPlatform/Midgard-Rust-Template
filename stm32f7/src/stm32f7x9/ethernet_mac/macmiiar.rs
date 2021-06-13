///Reader of register MACMIIAR
pub type R = crate::R<u32, super::MACMIIAR>;
///Writer for register MACMIIAR
pub type W = crate::W<u32, super::MACMIIAR>;
///Register MACMIIAR `reset()`'s with value 0
impl crate::ResetValue for super::MACMIIAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///MII busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MB_A {
    ///1: This bit is set to 1 by the application to indicate that a read or write access is in progress
    BUSY = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MB`
pub type MB_R = crate::R<bool, MB_A>;
impl MB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MB_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MB_A::BUSY),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `BUSY`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == MB_A::BUSY
    }
}
///Write proxy for field `MB`
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///This bit is set to 1 by the application to indicate that a read or write access is in progress
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(MB_A::BUSY)
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
///MII write
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MW_A {
    ///0: Read operation
    READ = 0,
    ///1: Write operation
    WRITE = 1,
}
impl From<MW_A> for bool {
    #[inline(always)]
    fn from(variant: MW_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MW`
pub type MW_R = crate::R<bool, MW_A>;
impl MW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MW_A {
        match self.bits {
            false => MW_A::READ,
            true => MW_A::WRITE,
        }
    }
    ///Checks if the value of the field is `READ`
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == MW_A::READ
    }
    ///Checks if the value of the field is `WRITE`
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == MW_A::WRITE
    }
}
///Write proxy for field `MW`
pub struct MW_W<'a> {
    w: &'a mut W,
}
impl<'a> MW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Read operation
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(MW_A::READ)
    }
    ///Write operation
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(MW_A::WRITE)
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
///Clock range
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CR_A {
    ///0: 60-100MHz HCLK/42
    CR_60_100 = 0,
    ///1: 100-150 MHz HCLK/62
    CR_100_150 = 1,
    ///2: 20-35MHz HCLK/16
    CR_20_35 = 2,
    ///3: 35-60MHz HCLK/16
    CR_35_60 = 3,
    ///4: 150-168MHz HCLK/102
    CR_150_168 = 4,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
///Reader of field `CR`
pub type CR_R = crate::R<u8, CR_A>;
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CR_A::CR_60_100),
            1 => Val(CR_A::CR_100_150),
            2 => Val(CR_A::CR_20_35),
            3 => Val(CR_A::CR_35_60),
            4 => Val(CR_A::CR_150_168),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `CR_60_100`
    #[inline(always)]
    pub fn is_cr_60_100(&self) -> bool {
        *self == CR_A::CR_60_100
    }
    ///Checks if the value of the field is `CR_100_150`
    #[inline(always)]
    pub fn is_cr_100_150(&self) -> bool {
        *self == CR_A::CR_100_150
    }
    ///Checks if the value of the field is `CR_20_35`
    #[inline(always)]
    pub fn is_cr_20_35(&self) -> bool {
        *self == CR_A::CR_20_35
    }
    ///Checks if the value of the field is `CR_35_60`
    #[inline(always)]
    pub fn is_cr_35_60(&self) -> bool {
        *self == CR_A::CR_35_60
    }
    ///Checks if the value of the field is `CR_150_168`
    #[inline(always)]
    pub fn is_cr_150_168(&self) -> bool {
        *self == CR_A::CR_150_168
    }
}
///Write proxy for field `CR`
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///60-100MHz HCLK/42
    #[inline(always)]
    pub fn cr_60_100(self) -> &'a mut W {
        self.variant(CR_A::CR_60_100)
    }
    ///100-150 MHz HCLK/62
    #[inline(always)]
    pub fn cr_100_150(self) -> &'a mut W {
        self.variant(CR_A::CR_100_150)
    }
    ///20-35MHz HCLK/16
    #[inline(always)]
    pub fn cr_20_35(self) -> &'a mut W {
        self.variant(CR_A::CR_20_35)
    }
    ///35-60MHz HCLK/16
    #[inline(always)]
    pub fn cr_35_60(self) -> &'a mut W {
        self.variant(CR_A::CR_35_60)
    }
    ///150-168MHz HCLK/102
    #[inline(always)]
    pub fn cr_150_168(self) -> &'a mut W {
        self.variant(CR_A::CR_150_168)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
///Reader of field `MR`
pub type MR_R = crate::R<u8, u8>;
///Write proxy for field `MR`
pub struct MR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
///Reader of field `PA`
pub type PA_R = crate::R<u8, u8>;
///Write proxy for field `PA`
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
impl R {
    ///Bit 0 - MII busy
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - MII write
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:4 - Clock range
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    ///Bits 6:10 - MII register - select the desired MII register in the PHY device
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - MII busy
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    ///Bit 1 - MII write
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W {
        MW_W { w: self }
    }
    ///Bits 2:4 - Clock range
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    ///Bits 6:10 - MII register - select the desired MII register in the PHY device
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W {
        MR_W { w: self }
    }
    ///Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
}
