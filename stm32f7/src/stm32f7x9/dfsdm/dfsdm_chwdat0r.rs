///Reader of register DFSDM_CHWDAT0R
pub type R = crate::R<u32, super::DFSDM_CHWDAT0R>;
///Reader of field `WDATA`
pub type WDATA_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:15 - Input channel y watchdog data
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
