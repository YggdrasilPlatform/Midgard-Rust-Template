///Reader of register DFSDM2_EXMIN
pub type R = crate::R<u32, super::DFSDM2_EXMIN>;
///Reader of field `EXMINCH`
pub type EXMINCH_R = crate::R<u8, u8>;
///Reader of field `EXMIN`
pub type EXMIN_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:2 - Extremes detector minimum data channel
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 8:31 - Extremes detector minimum value
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
