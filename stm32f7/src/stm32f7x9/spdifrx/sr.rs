///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Reader of field `RXNE`
pub type RXNE_R = crate::R<bool, bool>;
///Reader of field `CSRNE`
pub type CSRNE_R = crate::R<bool, bool>;
///Reader of field `PERR`
pub type PERR_R = crate::R<bool, bool>;
///Reader of field `OVR`
pub type OVR_R = crate::R<bool, bool>;
///Reader of field `SBD`
pub type SBD_R = crate::R<bool, bool>;
///Reader of field `SYNCD`
pub type SYNCD_R = crate::R<bool, bool>;
///Reader of field `FERR`
pub type FERR_R = crate::R<bool, bool>;
///Reader of field `SERR`
pub type SERR_R = crate::R<bool, bool>;
///Reader of field `TERR`
pub type TERR_R = crate::R<bool, bool>;
///Reader of field `WIDTH5`
pub type WIDTH5_R = crate::R<u16, u16>;
impl R {
    ///Bit 0 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Control Buffer register is not empty
    #[inline(always)]
    pub fn csrne(&self) -> CSRNE_R {
        CSRNE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Parity error
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Overrun error
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Synchronization Block Detected
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Synchronization Done
    #[inline(always)]
    pub fn syncd(&self) -> SYNCD_R {
        SYNCD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Framing error
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Synchronization error
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Time-out error
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 16:30 - Duration of 5 symbols counted with SPDIF_CLK
    #[inline(always)]
    pub fn width5(&self) -> WIDTH5_R {
        WIDTH5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
