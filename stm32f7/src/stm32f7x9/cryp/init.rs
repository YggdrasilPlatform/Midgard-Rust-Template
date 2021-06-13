///initialization vector registers
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivlr](ivlr) module
pub type IVLR = crate::Reg<u32, _IVLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVLR;
///`read()` method returns [ivlr::R](ivlr::R) reader structure
impl crate::Readable for IVLR {}
///`write(|w| ..)` method takes [ivlr::W](ivlr::W) writer structure
impl crate::Writable for IVLR {}
///initialization vector registers
pub mod ivlr;
///initialization vector registers
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivrr](ivrr) module
pub type IVRR = crate::Reg<u32, _IVRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVRR;
///`read()` method returns [ivrr::R](ivrr::R) reader structure
impl crate::Readable for IVRR {}
///`write(|w| ..)` method takes [ivrr::W](ivrr::W) writer structure
impl crate::Writable for IVRR {}
///initialization vector registers
pub mod ivrr;
