///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MDIOS configuration register
    pub cr: CR,
    ///0x04 - MDIOS write flag register
    pub wrfr: WRFR,
    ///0x08 - MDIOS clear write flag register
    pub cwrfr: CWRFR,
    ///0x0c - MDIOS read flag register
    pub rdfr: RDFR,
    ///0x10 - MDIOS clear read flag register
    pub crdfr: CRDFR,
    ///0x14 - MDIOS status register
    pub sr: SR,
    ///0x18 - MDIOS clear flag register
    pub clrfr: CLRFR,
    ///0x1c - MDIOS input data register %s
    pub dinr: [DINR; 32],
    ///0x9c - MDIOS output data register %s
    pub doutr: [DOUTR; 32],
}
///MDIOS configuration register
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
///MDIOS configuration register
pub mod cr;
///MDIOS write flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrfr](wrfr) module
pub type WRFR = crate::Reg<u32, _WRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRFR;
///`read()` method returns [wrfr::R](wrfr::R) reader structure
impl crate::Readable for WRFR {}
///MDIOS write flag register
pub mod wrfr;
///MDIOS clear write flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cwrfr](cwrfr) module
pub type CWRFR = crate::Reg<u32, _CWRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWRFR;
///`read()` method returns [cwrfr::R](cwrfr::R) reader structure
impl crate::Readable for CWRFR {}
///`write(|w| ..)` method takes [cwrfr::W](cwrfr::W) writer structure
impl crate::Writable for CWRFR {}
///MDIOS clear write flag register
pub mod cwrfr;
///MDIOS read flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdfr](rdfr) module
pub type RDFR = crate::Reg<u32, _RDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDFR;
///`read()` method returns [rdfr::R](rdfr::R) reader structure
impl crate::Readable for RDFR {}
///MDIOS read flag register
pub mod rdfr;
///MDIOS clear read flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crdfr](crdfr) module
pub type CRDFR = crate::Reg<u32, _CRDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRDFR;
///`read()` method returns [crdfr::R](crdfr::R) reader structure
impl crate::Readable for CRDFR {}
///`write(|w| ..)` method takes [crdfr::W](crdfr::W) writer structure
impl crate::Writable for CRDFR {}
///MDIOS clear read flag register
pub mod crdfr;
///MDIOS status register
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
///MDIOS status register
pub mod sr;
///MDIOS clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clrfr](clrfr) module
pub type CLRFR = crate::Reg<u32, _CLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRFR;
///`read()` method returns [clrfr::R](clrfr::R) reader structure
impl crate::Readable for CLRFR {}
///`write(|w| ..)` method takes [clrfr::W](clrfr::W) writer structure
impl crate::Writable for CLRFR {}
///MDIOS clear flag register
pub mod clrfr;
///MDIOS input data register %s
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr](dinr) module
pub type DINR = crate::Reg<u32, _DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR;
///`read()` method returns [dinr::R](dinr::R) reader structure
impl crate::Readable for DINR {}
///MDIOS input data register %s
pub mod dinr;
///MDIOS output data register %s
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr](doutr) module
pub type DOUTR = crate::Reg<u32, _DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR;
///`read()` method returns [doutr::R](doutr::R) reader structure
impl crate::Readable for DOUTR {}
///`write(|w| ..)` method takes [doutr::W](doutr::W) writer structure
impl crate::Writable for DOUTR {}
///MDIOS output data register %s
pub mod doutr;
