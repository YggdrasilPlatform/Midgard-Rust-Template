///Reader of register OTG_HS_HPTXFSIZ
pub type R = crate::R<u32, super::OTG_HS_HPTXFSIZ>;
///Writer for register OTG_HS_HPTXFSIZ
pub type W = crate::W<u32, super::OTG_HS_HPTXFSIZ>;
///Register OTG_HS_HPTXFSIZ `reset()`'s with value 0x0200_0600
impl crate::ResetValue for super::OTG_HS_HPTXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0600
    }
}
///Reader of field `PTXSA`
pub type PTXSA_R = crate::R<u16, u16>;
///Write proxy for field `PTXSA`
pub struct PTXSA_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `PTXFD`
pub type PTXFD_R = crate::R<u16, u16>;
///Write proxy for field `PTXFD`
pub struct PTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Host periodic TxFIFO start address
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Host periodic TxFIFO depth
    #[inline(always)]
    pub fn ptxfd(&self) -> PTXFD_R {
        PTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Host periodic TxFIFO start address
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W {
        PTXSA_W { w: self }
    }
    ///Bits 16:31 - Host periodic TxFIFO depth
    #[inline(always)]
    pub fn ptxfd(&mut self) -> PTXFD_W {
        PTXFD_W { w: self }
    }
}
