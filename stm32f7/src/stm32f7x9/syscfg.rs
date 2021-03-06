///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - memory remap register
    pub memrm: MEMRM,
    ///0x04 - peripheral mode configuration register
    pub pmc: PMC,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
    _reserved6: [u8; 8usize],
    ///0x20 - Compensation cell control register
    pub cmpcr: CMPCR,
}
///memory remap register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [memrm](memrm) module
pub type MEMRM = crate::Reg<u32, _MEMRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMRM;
///`read()` method returns [memrm::R](memrm::R) reader structure
impl crate::Readable for MEMRM {}
///`write(|w| ..)` method takes [memrm::W](memrm::W) writer structure
impl crate::Writable for MEMRM {}
///memory remap register
pub mod memrm;
///peripheral mode configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pmc](pmc) module
pub type PMC = crate::Reg<u32, _PMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC;
///`read()` method returns [pmc::R](pmc::R) reader structure
impl crate::Readable for PMC {}
///`write(|w| ..)` method takes [pmc::W](pmc::W) writer structure
impl crate::Writable for PMC {}
///peripheral mode configuration register
pub mod pmc;
///external interrupt configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr1](exticr1) module
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
///`read()` method returns [exticr1::R](exticr1::R) reader structure
impl crate::Readable for EXTICR1 {}
///`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure
impl crate::Writable for EXTICR1 {}
///external interrupt configuration register 1
pub mod exticr1;
///external interrupt configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr2](exticr2) module
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
///`read()` method returns [exticr2::R](exticr2::R) reader structure
impl crate::Readable for EXTICR2 {}
///`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure
impl crate::Writable for EXTICR2 {}
///external interrupt configuration register 2
pub mod exticr2;
///external interrupt configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr3](exticr3) module
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
///`read()` method returns [exticr3::R](exticr3::R) reader structure
impl crate::Readable for EXTICR3 {}
///`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure
impl crate::Writable for EXTICR3 {}
///external interrupt configuration register 3
pub mod exticr3;
///external interrupt configuration register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr4](exticr4) module
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
///`read()` method returns [exticr4::R](exticr4::R) reader structure
impl crate::Readable for EXTICR4 {}
///`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure
impl crate::Writable for EXTICR4 {}
///external interrupt configuration register 4
pub mod exticr4;
///Compensation cell control register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmpcr](cmpcr) module
pub type CMPCR = crate::Reg<u32, _CMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPCR;
///`read()` method returns [cmpcr::R](cmpcr::R) reader structure
impl crate::Readable for CMPCR {}
///Compensation cell control register
pub mod cmpcr;
