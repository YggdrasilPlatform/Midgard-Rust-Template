///Reader of register RXDR
pub type R = crate::R<u32, super::RXDR>;
///Reader of field `RXDR`
pub type RXDR_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:7 - CEC Rx Data Register
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new((self.bits & 0xff) as u8)
    }
}
