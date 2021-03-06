///Reader of register CSR
pub type R = crate::R<u32, super::CSR>;
///Reader of field `USR`
pub type USR_R = crate::R<u16, u16>;
///Reader of field `CS`
pub type CS_R = crate::R<u8, u8>;
///Reader of field `SOB`
pub type SOB_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:15 - User data information
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Channel A status information
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Start Of Block
    #[inline(always)]
    pub fn sob(&self) -> SOB_R {
        SOB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
