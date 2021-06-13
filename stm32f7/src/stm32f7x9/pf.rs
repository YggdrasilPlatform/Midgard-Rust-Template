///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Cache Level ID register
    pub clidr: CLIDR,
    ///0x04 - Cache Type register
    pub ctr: CTR,
    ///0x08 - Cache Size ID register
    pub ccsidr: CCSIDR,
}
///Cache Level ID register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clidr](clidr) module
pub type CLIDR = crate::Reg<u32, _CLIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLIDR;
///`read()` method returns [clidr::R](clidr::R) reader structure
impl crate::Readable for CLIDR {}
///Cache Level ID register
pub mod clidr;
///Cache Type register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctr](ctr) module
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
///`read()` method returns [ctr::R](ctr::R) reader structure
impl crate::Readable for CTR {}
///Cache Type register
pub mod ctr;
///Cache Size ID register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccsidr](ccsidr) module
pub type CCSIDR = crate::Reg<u32, _CCSIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCSIDR;
///`read()` method returns [ccsidr::R](ccsidr::R) reader structure
impl crate::Readable for CCSIDR {}
///Cache Size ID register
pub mod ccsidr;
