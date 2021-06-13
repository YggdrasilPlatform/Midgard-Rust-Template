///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Reader of field `PERF`
pub type PERF_R = crate::R<bool, bool>;
///Reader of field `SERF`
pub type SERF_R = crate::R<bool, bool>;
///Reader of field `TERF`
pub type TERF_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - Preamble error flag
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Start error flag
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Turnaround error flag
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
