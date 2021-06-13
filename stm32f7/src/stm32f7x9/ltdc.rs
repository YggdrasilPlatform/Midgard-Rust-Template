///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    ///0x08 - Synchronization Size Configuration Register
    pub sscr: SSCR,
    ///0x0c - Back Porch Configuration Register
    pub bpcr: BPCR,
    ///0x10 - Active Width Configuration Register
    pub awcr: AWCR,
    ///0x14 - Total Width Configuration Register
    pub twcr: TWCR,
    ///0x18 - Global Control Register
    pub gcr: GCR,
    _reserved5: [u8; 8usize],
    ///0x24 - Shadow Reload Configuration Register
    pub srcr: SRCR,
    _reserved6: [u8; 4usize],
    ///0x2c - Background Color Configuration Register
    pub bccr: BCCR,
    _reserved7: [u8; 4usize],
    ///0x34 - Interrupt Enable Register
    pub ier: IER,
    ///0x38 - Interrupt Status Register
    pub isr: ISR,
    ///0x3c - Interrupt Clear Register
    pub icr: ICR,
    ///0x40 - Line Interrupt Position Configuration Register
    pub lipcr: LIPCR,
    ///0x44 - Current Position Status Register
    pub cpsr: CPSR,
    ///0x48 - Current Display Status Register
    pub cdsr: CDSR,
    _reserved13: [u8; 56usize],
    ///0x84 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    pub layer1: LAYER,
    _reserved14: [u8; 60usize],
    ///0x104 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    pub layer2: LAYER,
}
///Register block
#[repr(C)]
pub struct LAYER {
    ///0x00 - Layerx Control Register
    pub cr: self::layer::CR,
    ///0x04 - Layerx Window Horizontal Position Configuration Register
    pub whpcr: self::layer::WHPCR,
    ///0x08 - Layerx Window Vertical Position Configuration Register
    pub wvpcr: self::layer::WVPCR,
    ///0x0c - Layerx Color Keying Configuration Register
    pub ckcr: self::layer::CKCR,
    ///0x10 - Layerx Pixel Format Configuration Register
    pub pfcr: self::layer::PFCR,
    ///0x14 - Layerx Constant Alpha Configuration Register
    pub cacr: self::layer::CACR,
    ///0x18 - Layerx Default Color Configuration Register
    pub dccr: self::layer::DCCR,
    ///0x1c - Layerx Blending Factors Configuration Register
    pub bfcr: self::layer::BFCR,
    _reserved8: [u8; 8usize],
    ///0x28 - Layerx Color Frame Buffer Address Register
    pub cfbar: self::layer::CFBAR,
    ///0x2c - Layerx Color Frame Buffer Length Register
    pub cfblr: self::layer::CFBLR,
    ///0x30 - Layerx ColorFrame Buffer Line Number Register
    pub cfblnr: self::layer::CFBLNR,
    _reserved11: [u8; 12usize],
    ///0x40 - Layerx CLUT Write Register
    pub clutwr: self::layer::CLUTWR,
}
///Register block
///Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
pub mod layer;
///Synchronization Size Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sscr](sscr) module
pub type SSCR = crate::Reg<u32, _SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCR;
///`read()` method returns [sscr::R](sscr::R) reader structure
impl crate::Readable for SSCR {}
///`write(|w| ..)` method takes [sscr::W](sscr::W) writer structure
impl crate::Writable for SSCR {}
///Synchronization Size Configuration Register
pub mod sscr;
///Back Porch Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bpcr](bpcr) module
pub type BPCR = crate::Reg<u32, _BPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPCR;
///`read()` method returns [bpcr::R](bpcr::R) reader structure
impl crate::Readable for BPCR {}
///`write(|w| ..)` method takes [bpcr::W](bpcr::W) writer structure
impl crate::Writable for BPCR {}
///Back Porch Configuration Register
pub mod bpcr;
///Active Width Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awcr](awcr) module
pub type AWCR = crate::Reg<u32, _AWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWCR;
///`read()` method returns [awcr::R](awcr::R) reader structure
impl crate::Readable for AWCR {}
///`write(|w| ..)` method takes [awcr::W](awcr::W) writer structure
impl crate::Writable for AWCR {}
///Active Width Configuration Register
pub mod awcr;
///Total Width Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [twcr](twcr) module
pub type TWCR = crate::Reg<u32, _TWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWCR;
///`read()` method returns [twcr::R](twcr::R) reader structure
impl crate::Readable for TWCR {}
///`write(|w| ..)` method takes [twcr::W](twcr::W) writer structure
impl crate::Writable for TWCR {}
///Total Width Configuration Register
pub mod twcr;
///Global Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gcr](gcr) module
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
///`read()` method returns [gcr::R](gcr::R) reader structure
impl crate::Readable for GCR {}
///`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure
impl crate::Writable for GCR {}
///Global Control Register
pub mod gcr;
///Shadow Reload Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [srcr](srcr) module
pub type SRCR = crate::Reg<u32, _SRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR;
///`read()` method returns [srcr::R](srcr::R) reader structure
impl crate::Readable for SRCR {}
///`write(|w| ..)` method takes [srcr::W](srcr::W) writer structure
impl crate::Writable for SRCR {}
///Shadow Reload Configuration Register
pub mod srcr;
///Background Color Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bccr](bccr) module
pub type BCCR = crate::Reg<u32, _BCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCCR;
///`read()` method returns [bccr::R](bccr::R) reader structure
impl crate::Readable for BCCR {}
///`write(|w| ..)` method takes [bccr::W](bccr::W) writer structure
impl crate::Writable for BCCR {}
///Background Color Configuration Register
pub mod bccr;
///Interrupt Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](ier) module
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
///`read()` method returns [ier::R](ier::R) reader structure
impl crate::Readable for IER {}
///`write(|w| ..)` method takes [ier::W](ier::W) writer structure
impl crate::Writable for IER {}
///Interrupt Enable Register
pub mod ier;
///Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](isr) module
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
///`read()` method returns [isr::R](isr::R) reader structure
impl crate::Readable for ISR {}
///Interrupt Status Register
pub mod isr;
///Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](icr) module
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
///`write(|w| ..)` method takes [icr::W](icr::W) writer structure
impl crate::Writable for ICR {}
///Interrupt Clear Register
pub mod icr;
///Line Interrupt Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lipcr](lipcr) module
pub type LIPCR = crate::Reg<u32, _LIPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIPCR;
///`read()` method returns [lipcr::R](lipcr::R) reader structure
impl crate::Readable for LIPCR {}
///`write(|w| ..)` method takes [lipcr::W](lipcr::W) writer structure
impl crate::Writable for LIPCR {}
///Line Interrupt Position Configuration Register
pub mod lipcr;
///Current Position Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpsr](cpsr) module
pub type CPSR = crate::Reg<u32, _CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSR;
///`read()` method returns [cpsr::R](cpsr::R) reader structure
impl crate::Readable for CPSR {}
///Current Position Status Register
pub mod cpsr;
///Current Display Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cdsr](cdsr) module
pub type CDSR = crate::Reg<u32, _CDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDSR;
///`read()` method returns [cdsr::R](cdsr::R) reader structure
impl crate::Readable for CDSR {}
///Current Display Status Register
pub mod cdsr;
