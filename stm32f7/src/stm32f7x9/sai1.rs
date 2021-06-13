///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Global configuration register
    pub gcr: GCR,
    ///0x04 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    pub cha: CH,
    ///0x24 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    pub chb: CH,
}
///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - AConfiguration register 1
    pub cr1: self::ch::CR1,
    ///0x04 - AConfiguration register 2
    pub cr2: self::ch::CR2,
    ///0x08 - AFRCR
    pub frcr: self::ch::FRCR,
    ///0x0c - ASlot register
    pub slotr: self::ch::SLOTR,
    ///0x10 - AInterrupt mask register2
    pub im: self::ch::IM,
    ///0x14 - AStatus register
    pub sr: self::ch::SR,
    ///0x18 - AClear flag register
    pub clrfr: self::ch::CLRFR,
    ///0x1c - AData register
    pub dr: self::ch::DR,
}
///Register block
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub mod ch;
///Global configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gcr](gcr) module
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
///`read()` method returns [gcr::R](gcr::R) reader structure
impl crate::Readable for GCR {}
///`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure
impl crate::Writable for GCR {}
///Global configuration register
pub mod gcr;
