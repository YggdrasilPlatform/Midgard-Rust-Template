///Reader of register MMCTGFSCCR
pub type R = crate::R<u32, super::MMCTGFSCCR>;
///Reader of field `TGFSCC`
pub type TGFSCC_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - Transmitted good frames single collision counter
    #[inline(always)]
    pub fn tgfscc(&self) -> TGFSCC_R {
        TGFSCC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
