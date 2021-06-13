///Reader of register PTPTSLR
pub type R = crate::R<u32, super::PTPTSLR>;
///Reader of field `STSS`
pub type STSS_R = crate::R<u32, u32>;
///Reader of field `STPNS`
pub type STPNS_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:30 - STSS
    #[inline(always)]
    pub fn stss(&self) -> STSS_R {
        STSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    ///Bit 31 - STPNS
    #[inline(always)]
    pub fn stpns(&self) -> STPNS_R {
        STPNS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
