///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - status register
    pub sr: SR,
    ///0x08 - data input register
    pub din: DIN,
    ///0x0c - data output register
    pub dout: DOUT,
    ///0x10 - DMA control register
    pub dmacr: DMACR,
    ///0x14 - interrupt mask set/clear register
    pub imscr: IMSCR,
    ///0x18 - raw interrupt status register
    pub risr: RISR,
    ///0x1c - masked interrupt status register
    pub misr: MISR,
    ///0x20 - Cluster KEY%s, containing K?LR, K?RR
    pub key: [KEY; 4],
    ///0x40 - Cluster INIT%s, containing IV?LR, IV?RR
    pub init: [INIT; 2],
    ///0x50 - context swap register
    pub csgcmccmr: [CSGCMCCMR; 8],
    ///0x70 - context swap register
    pub csgcmr: [CSGCMR; 8],
}
///Register block
#[repr(C)]
pub struct KEY {
    ///0x00 - key registers
    pub klr: self::key::KLR,
    ///0x04 - key registers
    pub krr: self::key::KRR,
}
///Register block
///Cluster KEY%s, containing K?LR, K?RR
pub mod key;
///Register block
#[repr(C)]
pub struct INIT {
    ///0x00 - initialization vector registers
    pub ivlr: self::init::IVLR,
    ///0x04 - initialization vector registers
    pub ivrr: self::init::IVRR,
}
///Register block
///Cluster INIT%s, containing IV?LR, IV?RR
pub mod init;
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](cr) module
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
///`read()` method returns [cr::R](cr::R) reader structure
impl crate::Readable for CR {}
///`write(|w| ..)` method takes [cr::W](cr::W) writer structure
impl crate::Writable for CR {}
///control register
pub mod cr;
///status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///status register
pub mod sr;
///data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [din](din) module
pub type DIN = crate::Reg<u32, _DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN;
///`read()` method returns [din::R](din::R) reader structure
impl crate::Readable for DIN {}
///`write(|w| ..)` method takes [din::W](din::W) writer structure
impl crate::Writable for DIN {}
///data input register
pub mod din;
///data output register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dout](dout) module
pub type DOUT = crate::Reg<u32, _DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT;
///`read()` method returns [dout::R](dout::R) reader structure
impl crate::Readable for DOUT {}
///data output register
pub mod dout;
///DMA control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmacr](dmacr) module
pub type DMACR = crate::Reg<u32, _DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACR;
///`read()` method returns [dmacr::R](dmacr::R) reader structure
impl crate::Readable for DMACR {}
///`write(|w| ..)` method takes [dmacr::W](dmacr::W) writer structure
impl crate::Writable for DMACR {}
///DMA control register
pub mod dmacr;
///interrupt mask set/clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imscr](imscr) module
pub type IMSCR = crate::Reg<u32, _IMSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMSCR;
///`read()` method returns [imscr::R](imscr::R) reader structure
impl crate::Readable for IMSCR {}
///`write(|w| ..)` method takes [imscr::W](imscr::W) writer structure
impl crate::Writable for IMSCR {}
///interrupt mask set/clear register
pub mod imscr;
///raw interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [risr](risr) module
pub type RISR = crate::Reg<u32, _RISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISR;
///`read()` method returns [risr::R](risr::R) reader structure
impl crate::Readable for RISR {}
///raw interrupt status register
pub mod risr;
///masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [misr](misr) module
pub type MISR = crate::Reg<u32, _MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR;
///`read()` method returns [misr::R](misr::R) reader structure
impl crate::Readable for MISR {}
///masked interrupt status register
pub mod misr;
///context swap register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csgcmccmr](csgcmccmr) module
pub type CSGCMCCMR = crate::Reg<u32, _CSGCMCCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCMR;
///`read()` method returns [csgcmccmr::R](csgcmccmr::R) reader structure
impl crate::Readable for CSGCMCCMR {}
///`write(|w| ..)` method takes [csgcmccmr::W](csgcmccmr::W) writer structure
impl crate::Writable for CSGCMCCMR {}
///context swap register
pub mod csgcmccmr;
///context swap register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csgcmr](csgcmr) module
pub type CSGCMR = crate::Reg<u32, _CSGCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMR;
///`read()` method returns [csgcmr::R](csgcmr::R) reader structure
impl crate::Readable for CSGCMR {}
///`write(|w| ..)` method takes [csgcmr::W](csgcmr::W) writer structure
impl crate::Writable for CSGCMR {}
///context swap register
pub mod csgcmr;
