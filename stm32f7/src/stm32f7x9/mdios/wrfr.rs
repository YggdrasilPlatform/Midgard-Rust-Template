///Reader of register WRFR
pub type R = crate::R<u32, super::WRFR>;
///Reader of field `WRF`
pub type WRF_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - Write flags for MDIO registers 0 to 31
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
