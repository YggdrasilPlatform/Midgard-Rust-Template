///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Instruction and Data Tightly-Coupled Memory Control Registers
    pub itcmcr: ITCMCR,
    ///0x04 - Instruction and Data Tightly-Coupled Memory Control Registers
    pub dtcmcr: DTCMCR,
    ///0x08 - AHBP Control register
    pub ahbpcr: AHBPCR,
    ///0x0c - Auxiliary Cache Control register
    pub cacr: CACR,
    ///0x10 - AHB Slave Control register
    pub ahbscr: AHBSCR,
    _reserved5: [u8; 4usize],
    ///0x18 - Auxiliary Bus Fault Status register
    pub abfsr: ABFSR,
}
///Instruction and Data Tightly-Coupled Memory Control Registers
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itcmcr](itcmcr) module
pub type ITCMCR = crate::Reg<u32, _ITCMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITCMCR;
///`read()` method returns [itcmcr::R](itcmcr::R) reader structure
impl crate::Readable for ITCMCR {}
///`write(|w| ..)` method takes [itcmcr::W](itcmcr::W) writer structure
impl crate::Writable for ITCMCR {}
///Instruction and Data Tightly-Coupled Memory Control Registers
pub mod itcmcr;
///Instruction and Data Tightly-Coupled Memory Control Registers
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtcmcr](dtcmcr) module
pub type DTCMCR = crate::Reg<u32, _DTCMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTCMCR;
///`read()` method returns [dtcmcr::R](dtcmcr::R) reader structure
impl crate::Readable for DTCMCR {}
///`write(|w| ..)` method takes [dtcmcr::W](dtcmcr::W) writer structure
impl crate::Writable for DTCMCR {}
///Instruction and Data Tightly-Coupled Memory Control Registers
pub mod dtcmcr;
///AHBP Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbpcr](ahbpcr) module
pub type AHBPCR = crate::Reg<u32, _AHBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBPCR;
///`read()` method returns [ahbpcr::R](ahbpcr::R) reader structure
impl crate::Readable for AHBPCR {}
///`write(|w| ..)` method takes [ahbpcr::W](ahbpcr::W) writer structure
impl crate::Writable for AHBPCR {}
///AHBP Control register
pub mod ahbpcr;
///Auxiliary Cache Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cacr](cacr) module
pub type CACR = crate::Reg<u32, _CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACR;
///`read()` method returns [cacr::R](cacr::R) reader structure
impl crate::Readable for CACR {}
///`write(|w| ..)` method takes [cacr::W](cacr::W) writer structure
impl crate::Writable for CACR {}
///Auxiliary Cache Control register
pub mod cacr;
///AHB Slave Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbscr](ahbscr) module
pub type AHBSCR = crate::Reg<u32, _AHBSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSCR;
///`read()` method returns [ahbscr::R](ahbscr::R) reader structure
impl crate::Readable for AHBSCR {}
///`write(|w| ..)` method takes [ahbscr::W](ahbscr::W) writer structure
impl crate::Writable for AHBSCR {}
///AHB Slave Control register
pub mod ahbscr;
///Auxiliary Bus Fault Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [abfsr](abfsr) module
pub type ABFSR = crate::Reg<u32, _ABFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABFSR;
///`read()` method returns [abfsr::R](abfsr::R) reader structure
impl crate::Readable for ABFSR {}
///`write(|w| ..)` method takes [abfsr::W](abfsr::W) writer structure
impl crate::Writable for ABFSR {}
///Auxiliary Bus Fault Status register
pub mod abfsr;
