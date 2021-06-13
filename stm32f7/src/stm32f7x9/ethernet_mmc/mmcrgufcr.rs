///Reader of register MMCRGUFCR
pub type R = crate::R<u32, super::MMCRGUFCR>;
///Reader of field `RGUFC`
pub type RGUFC_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - RGUFC
    #[inline(always)]
    pub fn rgufc(&self) -> RGUFC_R {
        RGUFC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
