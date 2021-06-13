///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - power control register
    pub cr1: CR1,
    ///0x04 - power control/status register
    pub csr1: CSR1,
    ///0x08 - power control register
    pub cr2: CR2,
    ///0x0c - power control/status register
    pub csr2: CSR2,
}
///power control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](cr1) module
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
///`read()` method returns [cr1::R](cr1::R) reader structure
impl crate::Readable for CR1 {}
///`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure
impl crate::Writable for CR1 {}
///power control register
pub mod cr1;
///power control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr1](csr1) module
pub type CSR1 = crate::Reg<u32, _CSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR1;
///`read()` method returns [csr1::R](csr1::R) reader structure
impl crate::Readable for CSR1 {}
///`write(|w| ..)` method takes [csr1::W](csr1::W) writer structure
impl crate::Writable for CSR1 {}
///power control/status register
pub mod csr1;
///power control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](cr2) module
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
///`read()` method returns [cr2::R](cr2::R) reader structure
impl crate::Readable for CR2 {}
///`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure
impl crate::Writable for CR2 {}
///power control register
pub mod cr2;
///power control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr2](csr2) module
pub type CSR2 = crate::Reg<u32, _CSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR2;
///`read()` method returns [csr2::R](csr2::R) reader structure
impl crate::Readable for CSR2 {}
///`write(|w| ..)` method takes [csr2::W](csr2::W) writer structure
impl crate::Writable for CSR2 {}
///power control/status register
pub mod csr2;
