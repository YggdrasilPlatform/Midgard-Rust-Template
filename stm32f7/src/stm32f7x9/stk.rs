///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SysTick control and status register
    pub csr: CSR,
    ///0x04 - SysTick reload value register
    pub rvr: RVR,
    ///0x08 - SysTick current value register
    pub cvr: CVR,
    ///0x0c - SysTick calibration value register
    pub calib: CALIB,
}
///SysTick control and status register
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
///SysTick control and status register
pub mod csr;
///SysTick reload value register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rvr](rvr) module
pub type RVR = crate::Reg<u32, _RVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RVR;
///`read()` method returns [rvr::R](rvr::R) reader structure
impl crate::Readable for RVR {}
///`write(|w| ..)` method takes [rvr::W](rvr::W) writer structure
impl crate::Writable for RVR {}
///SysTick reload value register
pub mod rvr;
///SysTick current value register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cvr](cvr) module
pub type CVR = crate::Reg<u32, _CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVR;
///`read()` method returns [cvr::R](cvr::R) reader structure
impl crate::Readable for CVR {}
///`write(|w| ..)` method takes [cvr::W](cvr::W) writer structure
impl crate::Writable for CVR {}
///SysTick current value register
pub mod cvr;
///SysTick calibration value register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calib](calib) module
pub type CALIB = crate::Reg<u32, _CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALIB;
///`read()` method returns [calib::R](calib::R) reader structure
impl crate::Readable for CALIB {}
///`write(|w| ..)` method takes [calib::W](calib::W) writer structure
impl crate::Writable for CALIB {}
///SysTick calibration value register
pub mod calib;
