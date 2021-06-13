///Reader of register MISR
pub type R = crate::R<u32, super::MISR>;
///Reader of field `OUTMIS`
pub type OUTMIS_R = crate::R<bool, bool>;
///Reader of field `INMIS`
pub type INMIS_R = crate::R<bool, bool>;
impl R {
    ///Bit 1 - Output FIFO service masked interrupt status
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Input FIFO service masked interrupt status
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 0x01) != 0)
    }
}
