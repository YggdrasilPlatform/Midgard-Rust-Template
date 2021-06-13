///stream x configuration register
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
///stream x configuration register
pub mod cr;
///stream x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ndtr](ndtr) module
pub type NDTR = crate::Reg<u32, _NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDTR;
///`read()` method returns [ndtr::R](ndtr::R) reader structure
impl crate::Readable for NDTR {}
///`write(|w| ..)` method takes [ndtr::W](ndtr::W) writer structure
impl crate::Writable for NDTR {}
///stream x number of data register
pub mod ndtr;
///stream x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [par](par) module
pub type PAR = crate::Reg<u32, _PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAR;
///`read()` method returns [par::R](par::R) reader structure
impl crate::Readable for PAR {}
///`write(|w| ..)` method takes [par::W](par::W) writer structure
impl crate::Writable for PAR {}
///stream x peripheral address register
pub mod par;
///stream x memory 0 address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m0ar](m0ar) module
pub type M0AR = crate::Reg<u32, _M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M0AR;
///`read()` method returns [m0ar::R](m0ar::R) reader structure
impl crate::Readable for M0AR {}
///`write(|w| ..)` method takes [m0ar::W](m0ar::W) writer structure
impl crate::Writable for M0AR {}
///stream x memory 0 address register
pub mod m0ar;
///stream x memory 1 address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m1ar](m1ar) module
pub type M1AR = crate::Reg<u32, _M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1AR;
///`read()` method returns [m1ar::R](m1ar::R) reader structure
impl crate::Readable for M1AR {}
///`write(|w| ..)` method takes [m1ar::W](m1ar::W) writer structure
impl crate::Writable for M1AR {}
///stream x memory 1 address register
pub mod m1ar;
///stream x FIFO control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](fcr) module
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
///`read()` method returns [fcr::R](fcr::R) reader structure
impl crate::Readable for FCR {}
///`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure
impl crate::Writable for FCR {}
///stream x FIFO control register
pub mod fcr;
