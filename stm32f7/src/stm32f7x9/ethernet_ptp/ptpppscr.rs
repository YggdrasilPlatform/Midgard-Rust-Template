///Reader of register PTPPPSCR
pub type R = crate::R<u32, super::PTPPPSCR>;
///Reader of field `TSSO`
pub type TSSO_R = crate::R<bool, bool>;
///Reader of field `TSTTR`
pub type TSTTR_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - TSSO
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TSTTR
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
