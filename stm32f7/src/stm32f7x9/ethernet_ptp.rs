///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Ethernet PTP time stamp control register
    pub ptptscr: PTPTSCR,
    ///0x04 - Ethernet PTP subsecond increment register
    pub ptpssir: PTPSSIR,
    ///0x08 - Ethernet PTP time stamp high register
    pub ptptshr: PTPTSHR,
    ///0x0c - Ethernet PTP time stamp low register
    pub ptptslr: PTPTSLR,
    ///0x10 - Ethernet PTP time stamp high update register
    pub ptptshur: PTPTSHUR,
    ///0x14 - Ethernet PTP time stamp low update register
    pub ptptslur: PTPTSLUR,
    ///0x18 - Ethernet PTP time stamp addend register
    pub ptptsar: PTPTSAR,
    ///0x1c - Ethernet PTP target time high register
    pub ptptthr: PTPTTHR,
    ///0x20 - Ethernet PTP target time low register
    pub ptpttlr: PTPTTLR,
    _reserved9: [u8; 4usize],
    ///0x28 - Ethernet PTP time stamp status register
    pub ptptssr: PTPTSSR,
    ///0x2c - Ethernet PTP PPS control register
    pub ptpppscr: PTPPPSCR,
}
///Ethernet PTP time stamp control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptscr](ptptscr) module
pub type PTPTSCR = crate::Reg<u32, _PTPTSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSCR;
///`read()` method returns [ptptscr::R](ptptscr::R) reader structure
impl crate::Readable for PTPTSCR {}
///`write(|w| ..)` method takes [ptptscr::W](ptptscr::W) writer structure
impl crate::Writable for PTPTSCR {}
///Ethernet PTP time stamp control register
pub mod ptptscr;
///Ethernet PTP subsecond increment register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptpssir](ptpssir) module
pub type PTPSSIR = crate::Reg<u32, _PTPSSIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPSSIR;
///`read()` method returns [ptpssir::R](ptpssir::R) reader structure
impl crate::Readable for PTPSSIR {}
///`write(|w| ..)` method takes [ptpssir::W](ptpssir::W) writer structure
impl crate::Writable for PTPSSIR {}
///Ethernet PTP subsecond increment register
pub mod ptpssir;
///Ethernet PTP time stamp high register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptshr](ptptshr) module
pub type PTPTSHR = crate::Reg<u32, _PTPTSHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSHR;
///`read()` method returns [ptptshr::R](ptptshr::R) reader structure
impl crate::Readable for PTPTSHR {}
///Ethernet PTP time stamp high register
pub mod ptptshr;
///Ethernet PTP time stamp low register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptslr](ptptslr) module
pub type PTPTSLR = crate::Reg<u32, _PTPTSLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSLR;
///`read()` method returns [ptptslr::R](ptptslr::R) reader structure
impl crate::Readable for PTPTSLR {}
///Ethernet PTP time stamp low register
pub mod ptptslr;
///Ethernet PTP time stamp high update register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptshur](ptptshur) module
pub type PTPTSHUR = crate::Reg<u32, _PTPTSHUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSHUR;
///`read()` method returns [ptptshur::R](ptptshur::R) reader structure
impl crate::Readable for PTPTSHUR {}
///`write(|w| ..)` method takes [ptptshur::W](ptptshur::W) writer structure
impl crate::Writable for PTPTSHUR {}
///Ethernet PTP time stamp high update register
pub mod ptptshur;
///Ethernet PTP time stamp low update register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptslur](ptptslur) module
pub type PTPTSLUR = crate::Reg<u32, _PTPTSLUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSLUR;
///`read()` method returns [ptptslur::R](ptptslur::R) reader structure
impl crate::Readable for PTPTSLUR {}
///`write(|w| ..)` method takes [ptptslur::W](ptptslur::W) writer structure
impl crate::Writable for PTPTSLUR {}
///Ethernet PTP time stamp low update register
pub mod ptptslur;
///Ethernet PTP time stamp addend register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptsar](ptptsar) module
pub type PTPTSAR = crate::Reg<u32, _PTPTSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSAR;
///`read()` method returns [ptptsar::R](ptptsar::R) reader structure
impl crate::Readable for PTPTSAR {}
///`write(|w| ..)` method takes [ptptsar::W](ptptsar::W) writer structure
impl crate::Writable for PTPTSAR {}
///Ethernet PTP time stamp addend register
pub mod ptptsar;
///Ethernet PTP target time high register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptthr](ptptthr) module
pub type PTPTTHR = crate::Reg<u32, _PTPTTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTTHR;
///`read()` method returns [ptptthr::R](ptptthr::R) reader structure
impl crate::Readable for PTPTTHR {}
///`write(|w| ..)` method takes [ptptthr::W](ptptthr::W) writer structure
impl crate::Writable for PTPTTHR {}
///Ethernet PTP target time high register
pub mod ptptthr;
///Ethernet PTP target time low register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptpttlr](ptpttlr) module
pub type PTPTTLR = crate::Reg<u32, _PTPTTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTTLR;
///`read()` method returns [ptpttlr::R](ptpttlr::R) reader structure
impl crate::Readable for PTPTTLR {}
///`write(|w| ..)` method takes [ptpttlr::W](ptpttlr::W) writer structure
impl crate::Writable for PTPTTLR {}
///Ethernet PTP target time low register
pub mod ptpttlr;
///Ethernet PTP time stamp status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptssr](ptptssr) module
pub type PTPTSSR = crate::Reg<u32, _PTPTSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSSR;
///`read()` method returns [ptptssr::R](ptptssr::R) reader structure
impl crate::Readable for PTPTSSR {}
///Ethernet PTP time stamp status register
pub mod ptptssr;
///Ethernet PTP PPS control register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptpppscr](ptpppscr) module
pub type PTPPPSCR = crate::Reg<u32, _PTPPPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPPPSCR;
///`read()` method returns [ptpppscr::R](ptpppscr::R) reader structure
impl crate::Readable for PTPPPSCR {}
///Ethernet PTP PPS control register
pub mod ptpppscr;
