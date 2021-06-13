///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - data input register
    pub din: DIN,
    ///0x08 - start register
    pub str: STR,
    ///0x0c - digest registers
    pub hr: [HR; 5],
    ///0x20 - interrupt enable register
    pub imr: IMR,
    ///0x24 - status register
    pub sr: SR,
    _reserved6: [u8; 208usize],
    ///0xf8 - context swap registers
    pub csr: [CSR; 54],
    _reserved7: [u8; 320usize],
    ///0x310 - HASH digest register
    pub hash_hr: [HASH_HR; 8],
}
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
///start register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [str](str) module
pub type STR = crate::Reg<u32, _STR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STR;
///`read()` method returns [str::R](str::R) reader structure
impl crate::Readable for STR {}
///`write(|w| ..)` method takes [str::W](str::W) writer structure
impl crate::Writable for STR {}
///start register
pub mod str;
///digest registers
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hr](hr) module
pub type HR = crate::Reg<u32, _HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HR;
///`read()` method returns [hr::R](hr::R) reader structure
impl crate::Readable for HR {}
///digest registers
pub mod hr;
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr](imr) module
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
///`read()` method returns [imr::R](imr::R) reader structure
impl crate::Readable for IMR {}
///`write(|w| ..)` method takes [imr::W](imr::W) writer structure
impl crate::Writable for IMR {}
///interrupt enable register
pub mod imr;
///status register
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
///status register
pub mod sr;
///context swap registers
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](csr) module
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
///`read()` method returns [csr::R](csr::R) reader structure
impl crate::Readable for CSR {}
///`write(|w| ..)` method takes [csr::W](csr::W) writer structure
impl crate::Writable for CSR {}
///context swap registers
pub mod csr;
///HASH digest register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hash_hr](hash_hr) module
pub type HASH_HR = crate::Reg<u32, _HASH_HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR;
///`read()` method returns [hash_hr::R](hash_hr::R) reader structure
impl crate::Readable for HASH_HR {}
///HASH digest register
pub mod hash_hr;
