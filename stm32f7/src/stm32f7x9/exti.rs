///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt mask register (EXTI_IMR)
    pub imr: IMR,
    ///0x04 - Event mask register (EXTI_EMR)
    pub emr: EMR,
    ///0x08 - Rising Trigger selection register (EXTI_RTSR)
    pub rtsr: RTSR,
    ///0x0c - Falling Trigger selection register (EXTI_FTSR)
    pub ftsr: FTSR,
    ///0x10 - Software interrupt event register (EXTI_SWIER)
    pub swier: SWIER,
    ///0x14 - Pending register (EXTI_PR)
    pub pr: PR,
}
///Interrupt mask register (EXTI_IMR)
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
///Interrupt mask register (EXTI_IMR)
pub mod imr;
///Event mask register (EXTI_EMR)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emr](emr) module
pub type EMR = crate::Reg<u32, _EMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR;
///`read()` method returns [emr::R](emr::R) reader structure
impl crate::Readable for EMR {}
///`write(|w| ..)` method takes [emr::W](emr::W) writer structure
impl crate::Writable for EMR {}
///Event mask register (EXTI_EMR)
pub mod emr;
///Rising Trigger selection register (EXTI_RTSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr](rtsr) module
pub type RTSR = crate::Reg<u32, _RTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR;
///`read()` method returns [rtsr::R](rtsr::R) reader structure
impl crate::Readable for RTSR {}
///`write(|w| ..)` method takes [rtsr::W](rtsr::W) writer structure
impl crate::Writable for RTSR {}
///Rising Trigger selection register (EXTI_RTSR)
pub mod rtsr;
///Falling Trigger selection register (EXTI_FTSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr](ftsr) module
pub type FTSR = crate::Reg<u32, _FTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR;
///`read()` method returns [ftsr::R](ftsr::R) reader structure
impl crate::Readable for FTSR {}
///`write(|w| ..)` method takes [ftsr::W](ftsr::W) writer structure
impl crate::Writable for FTSR {}
///Falling Trigger selection register (EXTI_FTSR)
pub mod ftsr;
///Software interrupt event register (EXTI_SWIER)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier](swier) module
pub type SWIER = crate::Reg<u32, _SWIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER;
///`read()` method returns [swier::R](swier::R) reader structure
impl crate::Readable for SWIER {}
///`write(|w| ..)` method takes [swier::W](swier::W) writer structure
impl crate::Writable for SWIER {}
///Software interrupt event register (EXTI_SWIER)
pub mod swier;
///Pending register (EXTI_PR)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr](pr) module
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
///`read()` method returns [pr::R](pr::R) reader structure
impl crate::Readable for PR {}
///`write(|w| ..)` method takes [pr::W](pr::W) writer structure
impl crate::Writable for PR {}
///Pending register (EXTI_PR)
pub mod pr;
