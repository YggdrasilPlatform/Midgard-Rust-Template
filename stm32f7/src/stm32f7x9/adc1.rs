///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - status register
    pub sr: SR,
    ///0x04 - control register 1
    pub cr1: CR1,
    ///0x08 - control register 2
    pub cr2: CR2,
    ///0x0c - sample time register 1
    pub smpr1: SMPR1,
    ///0x10 - sample time register 2
    pub smpr2: SMPR2,
    ///0x14 - injected channel data offset register x
    pub jofr1: JOFR,
    ///0x18 - injected channel data offset register x
    pub jofr2: JOFR,
    ///0x1c - injected channel data offset register x
    pub jofr3: JOFR,
    ///0x20 - injected channel data offset register x
    pub jofr4: JOFR,
    ///0x24 - watchdog higher threshold register
    pub htr: HTR,
    ///0x28 - watchdog lower threshold register
    pub ltr: LTR,
    ///0x2c - regular sequence register 1
    pub sqr1: SQR1,
    ///0x30 - regular sequence register 2
    pub sqr2: SQR2,
    ///0x34 - regular sequence register 3
    pub sqr3: SQR3,
    ///0x38 - injected sequence register
    pub jsqr: JSQR,
    ///0x3c - injected data register x
    pub jdr1: JDR,
    ///0x40 - injected data register x
    pub jdr2: JDR,
    ///0x44 - injected data register x
    pub jdr3: JDR,
    ///0x48 - injected data register x
    pub jdr4: JDR,
    ///0x4c - regular data register
    pub dr: DR,
}
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
///control register 1
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
///control register 1
pub mod cr1;
///control register 2
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
///control register 2
pub mod cr2;
///sample time register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr1](smpr1) module
pub type SMPR1 = crate::Reg<u32, _SMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR1;
///`read()` method returns [smpr1::R](smpr1::R) reader structure
impl crate::Readable for SMPR1 {}
///`write(|w| ..)` method takes [smpr1::W](smpr1::W) writer structure
impl crate::Writable for SMPR1 {}
///sample time register 1
pub mod smpr1;
///sample time register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr2](smpr2) module
pub type SMPR2 = crate::Reg<u32, _SMPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR2;
///`read()` method returns [smpr2::R](smpr2::R) reader structure
impl crate::Readable for SMPR2 {}
///`write(|w| ..)` method takes [smpr2::W](smpr2::W) writer structure
impl crate::Writable for SMPR2 {}
///sample time register 2
pub mod smpr2;
///injected channel data offset register x
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jofr](jofr) module
pub type JOFR = crate::Reg<u32, _JOFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JOFR;
///`read()` method returns [jofr::R](jofr::R) reader structure
impl crate::Readable for JOFR {}
///`write(|w| ..)` method takes [jofr::W](jofr::W) writer structure
impl crate::Writable for JOFR {}
///injected channel data offset register x
pub mod jofr;
///watchdog higher threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [htr](htr) module
pub type HTR = crate::Reg<u32, _HTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HTR;
///`read()` method returns [htr::R](htr::R) reader structure
impl crate::Readable for HTR {}
///`write(|w| ..)` method takes [htr::W](htr::W) writer structure
impl crate::Writable for HTR {}
///watchdog higher threshold register
pub mod htr;
///watchdog lower threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ltr](ltr) module
pub type LTR = crate::Reg<u32, _LTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTR;
///`read()` method returns [ltr::R](ltr::R) reader structure
impl crate::Readable for LTR {}
///`write(|w| ..)` method takes [ltr::W](ltr::W) writer structure
impl crate::Writable for LTR {}
///watchdog lower threshold register
pub mod ltr;
///regular sequence register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr1](sqr1) module
pub type SQR1 = crate::Reg<u32, _SQR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR1;
///`read()` method returns [sqr1::R](sqr1::R) reader structure
impl crate::Readable for SQR1 {}
///`write(|w| ..)` method takes [sqr1::W](sqr1::W) writer structure
impl crate::Writable for SQR1 {}
///regular sequence register 1
pub mod sqr1;
///regular sequence register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr2](sqr2) module
pub type SQR2 = crate::Reg<u32, _SQR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR2;
///`read()` method returns [sqr2::R](sqr2::R) reader structure
impl crate::Readable for SQR2 {}
///`write(|w| ..)` method takes [sqr2::W](sqr2::W) writer structure
impl crate::Writable for SQR2 {}
///regular sequence register 2
pub mod sqr2;
///regular sequence register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr3](sqr3) module
pub type SQR3 = crate::Reg<u32, _SQR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR3;
///`read()` method returns [sqr3::R](sqr3::R) reader structure
impl crate::Readable for SQR3 {}
///`write(|w| ..)` method takes [sqr3::W](sqr3::W) writer structure
impl crate::Writable for SQR3 {}
///regular sequence register 3
pub mod sqr3;
///injected sequence register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jsqr](jsqr) module
pub type JSQR = crate::Reg<u32, _JSQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JSQR;
///`read()` method returns [jsqr::R](jsqr::R) reader structure
impl crate::Readable for JSQR {}
///`write(|w| ..)` method takes [jsqr::W](jsqr::W) writer structure
impl crate::Writable for JSQR {}
///injected sequence register
pub mod jsqr;
///injected data register x
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdr](jdr) module
pub type JDR = crate::Reg<u32, _JDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR;
///`read()` method returns [jdr::R](jdr::R) reader structure
impl crate::Readable for JDR {}
///injected data register x
pub mod jdr;
///regular data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](dr) module
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
///`read()` method returns [dr::R](dr::R) reader structure
impl crate::Readable for DR {}
///regular data register
pub mod dr;
