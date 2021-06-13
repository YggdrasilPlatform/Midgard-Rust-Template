///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Ethernet DMA bus mode register
    pub dmabmr: DMABMR,
    ///0x04 - Ethernet DMA transmit poll demand register
    pub dmatpdr: DMATPDR,
    ///0x08 - EHERNET DMA receive poll demand register
    pub dmarpdr: DMARPDR,
    ///0x0c - Ethernet DMA receive descriptor list address register
    pub dmardlar: DMARDLAR,
    ///0x10 - Ethernet DMA transmit descriptor list address register
    pub dmatdlar: DMATDLAR,
    ///0x14 - Ethernet DMA status register
    pub dmasr: DMASR,
    ///0x18 - Ethernet DMA operation mode register
    pub dmaomr: DMAOMR,
    ///0x1c - Ethernet DMA interrupt enable register
    pub dmaier: DMAIER,
    ///0x20 - Ethernet DMA missed frame and buffer overflow counter register
    pub dmamfbocr: DMAMFBOCR,
    ///0x24 - Ethernet DMA receive status watchdog timer register
    pub dmarswtr: DMARSWTR,
    _reserved10: [u8; 32usize],
    ///0x48 - Ethernet DMA current host transmit descriptor register
    pub dmachtdr: DMACHTDR,
    ///0x4c - Ethernet DMA current host receive descriptor register
    pub dmachrdr: DMACHRDR,
    ///0x50 - Ethernet DMA current host transmit buffer address register
    pub dmachtbar: DMACHTBAR,
    ///0x54 - Ethernet DMA current host receive buffer address register
    pub dmachrbar: DMACHRBAR,
}
///Ethernet DMA bus mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmabmr](dmabmr) module
pub type DMABMR = crate::Reg<u32, _DMABMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMABMR;
///`read()` method returns [dmabmr::R](dmabmr::R) reader structure
impl crate::Readable for DMABMR {}
///`write(|w| ..)` method takes [dmabmr::W](dmabmr::W) writer structure
impl crate::Writable for DMABMR {}
///Ethernet DMA bus mode register
pub mod dmabmr;
///Ethernet DMA transmit poll demand register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmatpdr](dmatpdr) module
pub type DMATPDR = crate::Reg<u32, _DMATPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATPDR;
///`read()` method returns [dmatpdr::R](dmatpdr::R) reader structure
impl crate::Readable for DMATPDR {}
///`write(|w| ..)` method takes [dmatpdr::W](dmatpdr::W) writer structure
impl crate::Writable for DMATPDR {}
///Ethernet DMA transmit poll demand register
pub mod dmatpdr;
///EHERNET DMA receive poll demand register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmarpdr](dmarpdr) module
pub type DMARPDR = crate::Reg<u32, _DMARPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARPDR;
///`read()` method returns [dmarpdr::R](dmarpdr::R) reader structure
impl crate::Readable for DMARPDR {}
///`write(|w| ..)` method takes [dmarpdr::W](dmarpdr::W) writer structure
impl crate::Writable for DMARPDR {}
///EHERNET DMA receive poll demand register
pub mod dmarpdr;
///Ethernet DMA receive descriptor list address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmardlar](dmardlar) module
pub type DMARDLAR = crate::Reg<u32, _DMARDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARDLAR;
///`read()` method returns [dmardlar::R](dmardlar::R) reader structure
impl crate::Readable for DMARDLAR {}
///`write(|w| ..)` method takes [dmardlar::W](dmardlar::W) writer structure
impl crate::Writable for DMARDLAR {}
///Ethernet DMA receive descriptor list address register
pub mod dmardlar;
///Ethernet DMA transmit descriptor list address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmatdlar](dmatdlar) module
pub type DMATDLAR = crate::Reg<u32, _DMATDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATDLAR;
///`read()` method returns [dmatdlar::R](dmatdlar::R) reader structure
impl crate::Readable for DMATDLAR {}
///`write(|w| ..)` method takes [dmatdlar::W](dmatdlar::W) writer structure
impl crate::Writable for DMATDLAR {}
///Ethernet DMA transmit descriptor list address register
pub mod dmatdlar;
///Ethernet DMA status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmasr](dmasr) module
pub type DMASR = crate::Reg<u32, _DMASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASR;
///`read()` method returns [dmasr::R](dmasr::R) reader structure
impl crate::Readable for DMASR {}
///`write(|w| ..)` method takes [dmasr::W](dmasr::W) writer structure
impl crate::Writable for DMASR {}
///Ethernet DMA status register
pub mod dmasr;
///Ethernet DMA operation mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaomr](dmaomr) module
pub type DMAOMR = crate::Reg<u32, _DMAOMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAOMR;
///`read()` method returns [dmaomr::R](dmaomr::R) reader structure
impl crate::Readable for DMAOMR {}
///`write(|w| ..)` method takes [dmaomr::W](dmaomr::W) writer structure
impl crate::Writable for DMAOMR {}
///Ethernet DMA operation mode register
pub mod dmaomr;
///Ethernet DMA interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaier](dmaier) module
pub type DMAIER = crate::Reg<u32, _DMAIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIER;
///`read()` method returns [dmaier::R](dmaier::R) reader structure
impl crate::Readable for DMAIER {}
///`write(|w| ..)` method takes [dmaier::W](dmaier::W) writer structure
impl crate::Writable for DMAIER {}
///Ethernet DMA interrupt enable register
pub mod dmaier;
///Ethernet DMA missed frame and buffer overflow counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmamfbocr](dmamfbocr) module
pub type DMAMFBOCR = crate::Reg<u32, _DMAMFBOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMFBOCR;
///`read()` method returns [dmamfbocr::R](dmamfbocr::R) reader structure
impl crate::Readable for DMAMFBOCR {}
///`write(|w| ..)` method takes [dmamfbocr::W](dmamfbocr::W) writer structure
impl crate::Writable for DMAMFBOCR {}
///Ethernet DMA missed frame and buffer overflow counter register
pub mod dmamfbocr;
///Ethernet DMA receive status watchdog timer register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmarswtr](dmarswtr) module
pub type DMARSWTR = crate::Reg<u32, _DMARSWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARSWTR;
///`read()` method returns [dmarswtr::R](dmarswtr::R) reader structure
impl crate::Readable for DMARSWTR {}
///`write(|w| ..)` method takes [dmarswtr::W](dmarswtr::W) writer structure
impl crate::Writable for DMARSWTR {}
///Ethernet DMA receive status watchdog timer register
pub mod dmarswtr;
///Ethernet DMA current host transmit descriptor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmachtdr](dmachtdr) module
pub type DMACHTDR = crate::Reg<u32, _DMACHTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHTDR;
///`read()` method returns [dmachtdr::R](dmachtdr::R) reader structure
impl crate::Readable for DMACHTDR {}
///Ethernet DMA current host transmit descriptor register
pub mod dmachtdr;
///Ethernet DMA current host receive descriptor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmachrdr](dmachrdr) module
pub type DMACHRDR = crate::Reg<u32, _DMACHRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHRDR;
///`read()` method returns [dmachrdr::R](dmachrdr::R) reader structure
impl crate::Readable for DMACHRDR {}
///Ethernet DMA current host receive descriptor register
pub mod dmachrdr;
///Ethernet DMA current host transmit buffer address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmachtbar](dmachtbar) module
pub type DMACHTBAR = crate::Reg<u32, _DMACHTBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHTBAR;
///`read()` method returns [dmachtbar::R](dmachtbar::R) reader structure
impl crate::Readable for DMACHTBAR {}
///Ethernet DMA current host transmit buffer address register
pub mod dmachtbar;
///Ethernet DMA current host receive buffer address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmachrbar](dmachrbar) module
pub type DMACHRBAR = crate::Reg<u32, _DMACHRBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHRBAR;
///`read()` method returns [dmachrbar::R](dmachrbar::R) reader structure
impl crate::Readable for DMACHRBAR {}
///Ethernet DMA current host receive buffer address register
pub mod dmachrbar;
