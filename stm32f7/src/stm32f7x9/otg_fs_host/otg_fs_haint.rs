///Reader of register OTG_FS_HAINT
pub type R = crate::R<u32, super::OTG_FS_HAINT>;
///Reader of field `HAINT`
pub type HAINT_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:15 - Channel interrupts
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
