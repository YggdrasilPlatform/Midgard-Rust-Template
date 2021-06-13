///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Flash access control register
    pub acr: ACR,
    ///0x04 - Flash key register
    pub keyr: KEYR,
    ///0x08 - Flash option key register
    pub optkeyr: OPTKEYR,
    ///0x0c - Status register
    pub sr: SR,
    ///0x10 - Control register
    pub cr: CR,
    ///0x14 - Flash option control register
    pub optcr: OPTCR,
    ///0x18 - Flash option control register 1
    pub optcr1: OPTCR1,
}
///Flash access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [acr](acr) module
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
///`read()` method returns [acr::R](acr::R) reader structure
impl crate::Readable for ACR {}
///`write(|w| ..)` method takes [acr::W](acr::W) writer structure
impl crate::Writable for ACR {}
///Flash access control register
pub mod acr;
///Flash key register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr](keyr) module
pub type KEYR = crate::Reg<u32, _KEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR;
///`write(|w| ..)` method takes [keyr::W](keyr::W) writer structure
impl crate::Writable for KEYR {}
///Flash key register
pub mod keyr;
///Flash option key register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optkeyr](optkeyr) module
pub type OPTKEYR = crate::Reg<u32, _OPTKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR;
///`write(|w| ..)` method takes [optkeyr::W](optkeyr::W) writer structure
impl crate::Writable for OPTKEYR {}
///Flash option key register
pub mod optkeyr;
///Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///`write(|w| ..)` method takes [sr::W](sr::W) writer structure
impl crate::Writable for SR {}
///Status register
pub mod sr;
///Control register
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
///Control register
pub mod cr;
///Flash option control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optcr](optcr) module
pub type OPTCR = crate::Reg<u32, _OPTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTCR;
///`read()` method returns [optcr::R](optcr::R) reader structure
impl crate::Readable for OPTCR {}
///`write(|w| ..)` method takes [optcr::W](optcr::W) writer structure
impl crate::Writable for OPTCR {}
///Flash option control register
pub mod optcr;
///Flash option control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optcr1](optcr1) module
pub type OPTCR1 = crate::Reg<u32, _OPTCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTCR1;
///`read()` method returns [optcr1::R](optcr1::R) reader structure
impl crate::Readable for OPTCR1 {}
///`write(|w| ..)` method takes [optcr1::W](optcr1::W) writer structure
impl crate::Writable for OPTCR1 {}
///Flash option control register 1
pub mod optcr1;
