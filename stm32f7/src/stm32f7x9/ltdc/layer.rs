///Layerx Control Register
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
///Layerx Control Register
pub mod cr;
///Layerx Window Horizontal Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [whpcr](whpcr) module
pub type WHPCR = crate::Reg<u32, _WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WHPCR;
///`read()` method returns [whpcr::R](whpcr::R) reader structure
impl crate::Readable for WHPCR {}
///`write(|w| ..)` method takes [whpcr::W](whpcr::W) writer structure
impl crate::Writable for WHPCR {}
///Layerx Window Horizontal Position Configuration Register
pub mod whpcr;
///Layerx Window Vertical Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wvpcr](wvpcr) module
pub type WVPCR = crate::Reg<u32, _WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WVPCR;
///`read()` method returns [wvpcr::R](wvpcr::R) reader structure
impl crate::Readable for WVPCR {}
///`write(|w| ..)` method takes [wvpcr::W](wvpcr::W) writer structure
impl crate::Writable for WVPCR {}
///Layerx Window Vertical Position Configuration Register
pub mod wvpcr;
///Layerx Color Keying Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ckcr](ckcr) module
pub type CKCR = crate::Reg<u32, _CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCR;
///`read()` method returns [ckcr::R](ckcr::R) reader structure
impl crate::Readable for CKCR {}
///`write(|w| ..)` method takes [ckcr::W](ckcr::W) writer structure
impl crate::Writable for CKCR {}
///Layerx Color Keying Configuration Register
pub mod ckcr;
///Layerx Pixel Format Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pfcr](pfcr) module
pub type PFCR = crate::Reg<u32, _PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFCR;
///`read()` method returns [pfcr::R](pfcr::R) reader structure
impl crate::Readable for PFCR {}
///`write(|w| ..)` method takes [pfcr::W](pfcr::W) writer structure
impl crate::Writable for PFCR {}
///Layerx Pixel Format Configuration Register
pub mod pfcr;
///Layerx Constant Alpha Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cacr](cacr) module
pub type CACR = crate::Reg<u32, _CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACR;
///`read()` method returns [cacr::R](cacr::R) reader structure
impl crate::Readable for CACR {}
///`write(|w| ..)` method takes [cacr::W](cacr::W) writer structure
impl crate::Writable for CACR {}
///Layerx Constant Alpha Configuration Register
pub mod cacr;
///Layerx Default Color Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dccr](dccr) module
pub type DCCR = crate::Reg<u32, _DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCR;
///`read()` method returns [dccr::R](dccr::R) reader structure
impl crate::Readable for DCCR {}
///`write(|w| ..)` method takes [dccr::W](dccr::W) writer structure
impl crate::Writable for DCCR {}
///Layerx Default Color Configuration Register
pub mod dccr;
///Layerx Blending Factors Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bfcr](bfcr) module
pub type BFCR = crate::Reg<u32, _BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFCR;
///`read()` method returns [bfcr::R](bfcr::R) reader structure
impl crate::Readable for BFCR {}
///`write(|w| ..)` method takes [bfcr::W](bfcr::W) writer structure
impl crate::Writable for BFCR {}
///Layerx Blending Factors Configuration Register
pub mod bfcr;
///Layerx Color Frame Buffer Address Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfbar](cfbar) module
pub type CFBAR = crate::Reg<u32, _CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFBAR;
///`read()` method returns [cfbar::R](cfbar::R) reader structure
impl crate::Readable for CFBAR {}
///`write(|w| ..)` method takes [cfbar::W](cfbar::W) writer structure
impl crate::Writable for CFBAR {}
///Layerx Color Frame Buffer Address Register
pub mod cfbar;
///Layerx Color Frame Buffer Length Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfblr](cfblr) module
pub type CFBLR = crate::Reg<u32, _CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFBLR;
///`read()` method returns [cfblr::R](cfblr::R) reader structure
impl crate::Readable for CFBLR {}
///`write(|w| ..)` method takes [cfblr::W](cfblr::W) writer structure
impl crate::Writable for CFBLR {}
///Layerx Color Frame Buffer Length Register
pub mod cfblr;
///Layerx ColorFrame Buffer Line Number Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfblnr](cfblnr) module
pub type CFBLNR = crate::Reg<u32, _CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFBLNR;
///`read()` method returns [cfblnr::R](cfblnr::R) reader structure
impl crate::Readable for CFBLNR {}
///`write(|w| ..)` method takes [cfblnr::W](cfblnr::W) writer structure
impl crate::Writable for CFBLNR {}
///Layerx ColorFrame Buffer Line Number Register
pub mod cfblnr;
///Layerx CLUT Write Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clutwr](clutwr) module
pub type CLUTWR = crate::Reg<u32, _CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLUTWR;
///`write(|w| ..)` method takes [clutwr::W](clutwr::W) writer structure
impl crate::Writable for CLUTWR {}
///Layerx CLUT Write Register
pub mod clutwr;
