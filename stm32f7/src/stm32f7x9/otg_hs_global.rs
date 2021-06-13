///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_HS control and status register
    pub otg_hs_gotgctl: OTG_HS_GOTGCTL,
    ///0x04 - OTG_HS interrupt register
    pub otg_hs_gotgint: OTG_HS_GOTGINT,
    ///0x08 - OTG_HS AHB configuration register
    pub otg_hs_gahbcfg: OTG_HS_GAHBCFG,
    ///0x0c - OTG_HS USB configuration register
    pub otg_hs_gusbcfg: OTG_HS_GUSBCFG,
    ///0x10 - OTG_HS reset register
    pub otg_hs_grstctl: OTG_HS_GRSTCTL,
    ///0x14 - OTG_HS core interrupt register
    pub otg_hs_gintsts: OTG_HS_GINTSTS,
    ///0x18 - OTG_HS interrupt mask register
    pub otg_hs_gintmsk: OTG_HS_GINTMSK,
    _reserved_7_otg_hs_grxstsr: [u8; 4usize],
    _reserved_8_otg_hs_grxstsp: [u8; 4usize],
    ///0x24 - OTG_HS Receive FIFO size register
    pub otg_hs_grxfsiz: OTG_HS_GRXFSIZ,
    _reserved_10_otg_hs: [u8; 4usize],
    ///0x2c - OTG_HS nonperiodic transmit FIFO/queue status register
    pub otg_hs_gnptxsts: OTG_HS_GNPTXSTS,
    _reserved12: [u8; 8usize],
    ///0x38 - OTG_HS general core configuration register
    pub otg_hs_gccfg: OTG_HS_GCCFG,
    ///0x3c - OTG_HS core ID register
    pub otg_hs_cid: OTG_HS_CID,
    _reserved14: [u8; 20usize],
    ///0x54 - OTG core LPM configuration register
    pub otg_hs_glpmcfg: OTG_HS_GLPMCFG,
    _reserved15: [u8; 168usize],
    ///0x100 - OTG_HS Host periodic transmit FIFO size register
    pub otg_hs_hptxfsiz: OTG_HS_HPTXFSIZ,
    ///0x104 - OTG_HS device IN endpoint transmit FIFO size register
    pub otg_hs_dieptxf1: OTG_HS_DIEPTXF1,
    ///0x108 - OTG_HS device IN endpoint transmit FIFO size register
    pub otg_hs_dieptxf2: OTG_HS_DIEPTXF2,
    _reserved18: [u8; 16usize],
    ///0x11c - OTG_HS device IN endpoint transmit FIFO size register
    pub otg_hs_dieptxf3: OTG_HS_DIEPTXF3,
    ///0x120 - OTG_HS device IN endpoint transmit FIFO size register
    pub otg_hs_dieptxf4: OTG_HS_DIEPTXF4,
    ///0x124 - OTG_HS device IN endpoint transmit FIFO size register
    pub otg_hs_dieptxf5: OTG_HS_DIEPTXF5,
    ///0x128 - OTG_HS device IN endpoint transmit FIFO size register
    pub otg_hs_dieptxf6: OTG_HS_DIEPTXF6,
    ///0x12c - OTG_HS device IN endpoint transmit FIFO size register
    pub otg_hs_dieptxf7: OTG_HS_DIEPTXF7,
}
impl RegisterBlock {
    ///0x1c - OTG_HS Receive status debug read register (peripheral mode mode)
    #[inline(always)]
    pub fn otg_hs_grxstsr_device(&self) -> &OTG_HS_GRXSTSR_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_HS_GRXSTSR_DEVICE)
        }
    }
    ///0x1c - OTG_HS Receive status debug read register (peripheral mode mode)
    #[inline(always)]
    pub fn otg_hs_grxstsr_device_mut(&self) -> &mut OTG_HS_GRXSTSR_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OTG_HS_GRXSTSR_DEVICE)
        }
    }
    ///0x1c - OTG_HS Receive status debug read register (host mode)
    #[inline(always)]
    pub fn otg_hs_grxstsr_host(&self) -> &OTG_HS_GRXSTSR_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_HS_GRXSTSR_HOST)
        }
    }
    ///0x1c - OTG_HS Receive status debug read register (host mode)
    #[inline(always)]
    pub fn otg_hs_grxstsr_host_mut(&self) -> &mut OTG_HS_GRXSTSR_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OTG_HS_GRXSTSR_HOST)
        }
    }
    ///0x20 - OTG_HS status read and pop register (peripheral mode)
    #[inline(always)]
    pub fn otg_hs_grxstsp_device(&self) -> &OTG_HS_GRXSTSP_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_HS_GRXSTSP_DEVICE)
        }
    }
    ///0x20 - OTG_HS status read and pop register (peripheral mode)
    #[inline(always)]
    pub fn otg_hs_grxstsp_device_mut(&self) -> &mut OTG_HS_GRXSTSP_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OTG_HS_GRXSTSP_DEVICE)
        }
    }
    ///0x20 - OTG_HS status read and pop register (host mode)
    #[inline(always)]
    pub fn otg_hs_grxstsp_host(&self) -> &OTG_HS_GRXSTSP_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_HS_GRXSTSP_HOST)
        }
    }
    ///0x20 - OTG_HS status read and pop register (host mode)
    #[inline(always)]
    pub fn otg_hs_grxstsp_host_mut(&self) -> &mut OTG_HS_GRXSTSP_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OTG_HS_GRXSTSP_HOST)
        }
    }
    ///0x28 - Endpoint 0 transmit FIFO size (peripheral mode)
    #[inline(always)]
    pub fn otg_hs_dieptxf0_device(&self) -> &OTG_HS_DIEPTXF0_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_HS_DIEPTXF0_DEVICE)
        }
    }
    ///0x28 - Endpoint 0 transmit FIFO size (peripheral mode)
    #[inline(always)]
    pub fn otg_hs_dieptxf0_device_mut(&self) -> &mut OTG_HS_DIEPTXF0_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut OTG_HS_DIEPTXF0_DEVICE)
        }
    }
    ///0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)
    #[inline(always)]
    pub fn otg_hs_hnptxfsiz_host(&self) -> &OTG_HS_HNPTXFSIZ_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_HS_HNPTXFSIZ_HOST)
        }
    }
    ///0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)
    #[inline(always)]
    pub fn otg_hs_hnptxfsiz_host_mut(&self) -> &mut OTG_HS_HNPTXFSIZ_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut OTG_HS_HNPTXFSIZ_HOST)
        }
    }
}
///OTG_HS control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gotgctl](otg_hs_gotgctl) module
pub type OTG_HS_GOTGCTL = crate::Reg<u32, _OTG_HS_GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GOTGCTL;
///`read()` method returns [otg_hs_gotgctl::R](otg_hs_gotgctl::R) reader structure
impl crate::Readable for OTG_HS_GOTGCTL {}
///`write(|w| ..)` method takes [otg_hs_gotgctl::W](otg_hs_gotgctl::W) writer structure
impl crate::Writable for OTG_HS_GOTGCTL {}
///OTG_HS control and status register
pub mod otg_hs_gotgctl;
///OTG_HS interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gotgint](otg_hs_gotgint) module
pub type OTG_HS_GOTGINT = crate::Reg<u32, _OTG_HS_GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GOTGINT;
///`read()` method returns [otg_hs_gotgint::R](otg_hs_gotgint::R) reader structure
impl crate::Readable for OTG_HS_GOTGINT {}
///`write(|w| ..)` method takes [otg_hs_gotgint::W](otg_hs_gotgint::W) writer structure
impl crate::Writable for OTG_HS_GOTGINT {}
///OTG_HS interrupt register
pub mod otg_hs_gotgint;
///OTG_HS AHB configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gahbcfg](otg_hs_gahbcfg) module
pub type OTG_HS_GAHBCFG = crate::Reg<u32, _OTG_HS_GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GAHBCFG;
///`read()` method returns [otg_hs_gahbcfg::R](otg_hs_gahbcfg::R) reader structure
impl crate::Readable for OTG_HS_GAHBCFG {}
///`write(|w| ..)` method takes [otg_hs_gahbcfg::W](otg_hs_gahbcfg::W) writer structure
impl crate::Writable for OTG_HS_GAHBCFG {}
///OTG_HS AHB configuration register
pub mod otg_hs_gahbcfg;
///OTG_HS USB configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gusbcfg](otg_hs_gusbcfg) module
pub type OTG_HS_GUSBCFG = crate::Reg<u32, _OTG_HS_GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GUSBCFG;
///`read()` method returns [otg_hs_gusbcfg::R](otg_hs_gusbcfg::R) reader structure
impl crate::Readable for OTG_HS_GUSBCFG {}
///`write(|w| ..)` method takes [otg_hs_gusbcfg::W](otg_hs_gusbcfg::W) writer structure
impl crate::Writable for OTG_HS_GUSBCFG {}
///OTG_HS USB configuration register
pub mod otg_hs_gusbcfg;
///OTG_HS reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_grstctl](otg_hs_grstctl) module
pub type OTG_HS_GRSTCTL = crate::Reg<u32, _OTG_HS_GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRSTCTL;
///`read()` method returns [otg_hs_grstctl::R](otg_hs_grstctl::R) reader structure
impl crate::Readable for OTG_HS_GRSTCTL {}
///`write(|w| ..)` method takes [otg_hs_grstctl::W](otg_hs_grstctl::W) writer structure
impl crate::Writable for OTG_HS_GRSTCTL {}
///OTG_HS reset register
pub mod otg_hs_grstctl;
///OTG_HS core interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gintsts](otg_hs_gintsts) module
pub type OTG_HS_GINTSTS = crate::Reg<u32, _OTG_HS_GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GINTSTS;
///`read()` method returns [otg_hs_gintsts::R](otg_hs_gintsts::R) reader structure
impl crate::Readable for OTG_HS_GINTSTS {}
///`write(|w| ..)` method takes [otg_hs_gintsts::W](otg_hs_gintsts::W) writer structure
impl crate::Writable for OTG_HS_GINTSTS {}
///OTG_HS core interrupt register
pub mod otg_hs_gintsts;
///OTG_HS interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gintmsk](otg_hs_gintmsk) module
pub type OTG_HS_GINTMSK = crate::Reg<u32, _OTG_HS_GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GINTMSK;
///`read()` method returns [otg_hs_gintmsk::R](otg_hs_gintmsk::R) reader structure
impl crate::Readable for OTG_HS_GINTMSK {}
///`write(|w| ..)` method takes [otg_hs_gintmsk::W](otg_hs_gintmsk::W) writer structure
impl crate::Writable for OTG_HS_GINTMSK {}
///OTG_HS interrupt mask register
pub mod otg_hs_gintmsk;
///OTG_HS Receive status debug read register (host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_grxstsr_host](otg_hs_grxstsr_host) module
pub type OTG_HS_GRXSTSR_HOST = crate::Reg<u32, _OTG_HS_GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSR_HOST;
///`read()` method returns [otg_hs_grxstsr_host::R](otg_hs_grxstsr_host::R) reader structure
impl crate::Readable for OTG_HS_GRXSTSR_HOST {}
///OTG_HS Receive status debug read register (host mode)
pub mod otg_hs_grxstsr_host;
///OTG_HS status read and pop register (host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_grxstsp_host](otg_hs_grxstsp_host) module
pub type OTG_HS_GRXSTSP_HOST = crate::Reg<u32, _OTG_HS_GRXSTSP_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSP_HOST;
///`read()` method returns [otg_hs_grxstsp_host::R](otg_hs_grxstsp_host::R) reader structure
impl crate::Readable for OTG_HS_GRXSTSP_HOST {}
///OTG_HS status read and pop register (host mode)
pub mod otg_hs_grxstsp_host;
///OTG_HS Receive FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_grxfsiz](otg_hs_grxfsiz) module
pub type OTG_HS_GRXFSIZ = crate::Reg<u32, _OTG_HS_GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXFSIZ;
///`read()` method returns [otg_hs_grxfsiz::R](otg_hs_grxfsiz::R) reader structure
impl crate::Readable for OTG_HS_GRXFSIZ {}
///`write(|w| ..)` method takes [otg_hs_grxfsiz::W](otg_hs_grxfsiz::W) writer structure
impl crate::Writable for OTG_HS_GRXFSIZ {}
///OTG_HS Receive FIFO size register
pub mod otg_hs_grxfsiz;
///OTG_HS nonperiodic transmit FIFO size register (host mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hnptxfsiz_host](otg_hs_hnptxfsiz_host) module
pub type OTG_HS_HNPTXFSIZ_HOST = crate::Reg<u32, _OTG_HS_HNPTXFSIZ_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HNPTXFSIZ_HOST;
///`read()` method returns [otg_hs_hnptxfsiz_host::R](otg_hs_hnptxfsiz_host::R) reader structure
impl crate::Readable for OTG_HS_HNPTXFSIZ_HOST {}
///`write(|w| ..)` method takes [otg_hs_hnptxfsiz_host::W](otg_hs_hnptxfsiz_host::W) writer structure
impl crate::Writable for OTG_HS_HNPTXFSIZ_HOST {}
///OTG_HS nonperiodic transmit FIFO size register (host mode)
pub mod otg_hs_hnptxfsiz_host;
///Endpoint 0 transmit FIFO size (peripheral mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf0_device](otg_hs_dieptxf0_device) module
pub type OTG_HS_DIEPTXF0_DEVICE = crate::Reg<u32, _OTG_HS_DIEPTXF0_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF0_DEVICE;
///`read()` method returns [otg_hs_dieptxf0_device::R](otg_hs_dieptxf0_device::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF0_DEVICE {}
///`write(|w| ..)` method takes [otg_hs_dieptxf0_device::W](otg_hs_dieptxf0_device::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF0_DEVICE {}
///Endpoint 0 transmit FIFO size (peripheral mode)
pub mod otg_hs_dieptxf0_device;
///OTG_HS nonperiodic transmit FIFO/queue status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gnptxsts](otg_hs_gnptxsts) module
pub type OTG_HS_GNPTXSTS = crate::Reg<u32, _OTG_HS_GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GNPTXSTS;
///`read()` method returns [otg_hs_gnptxsts::R](otg_hs_gnptxsts::R) reader structure
impl crate::Readable for OTG_HS_GNPTXSTS {}
///OTG_HS nonperiodic transmit FIFO/queue status register
pub mod otg_hs_gnptxsts;
///OTG_HS general core configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_gccfg](otg_hs_gccfg) module
pub type OTG_HS_GCCFG = crate::Reg<u32, _OTG_HS_GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GCCFG;
///`read()` method returns [otg_hs_gccfg::R](otg_hs_gccfg::R) reader structure
impl crate::Readable for OTG_HS_GCCFG {}
///`write(|w| ..)` method takes [otg_hs_gccfg::W](otg_hs_gccfg::W) writer structure
impl crate::Writable for OTG_HS_GCCFG {}
///OTG_HS general core configuration register
pub mod otg_hs_gccfg;
///OTG_HS core ID register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_cid](otg_hs_cid) module
pub type OTG_HS_CID = crate::Reg<u32, _OTG_HS_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_CID;
///`read()` method returns [otg_hs_cid::R](otg_hs_cid::R) reader structure
impl crate::Readable for OTG_HS_CID {}
///`write(|w| ..)` method takes [otg_hs_cid::W](otg_hs_cid::W) writer structure
impl crate::Writable for OTG_HS_CID {}
///OTG_HS core ID register
pub mod otg_hs_cid;
///OTG_HS Host periodic transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hptxfsiz](otg_hs_hptxfsiz) module
pub type OTG_HS_HPTXFSIZ = crate::Reg<u32, _OTG_HS_HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HPTXFSIZ;
///`read()` method returns [otg_hs_hptxfsiz::R](otg_hs_hptxfsiz::R) reader structure
impl crate::Readable for OTG_HS_HPTXFSIZ {}
///`write(|w| ..)` method takes [otg_hs_hptxfsiz::W](otg_hs_hptxfsiz::W) writer structure
impl crate::Writable for OTG_HS_HPTXFSIZ {}
///OTG_HS Host periodic transmit FIFO size register
pub mod otg_hs_hptxfsiz;
///OTG_HS device IN endpoint transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf1](otg_hs_dieptxf1) module
pub type OTG_HS_DIEPTXF1 = crate::Reg<u32, _OTG_HS_DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF1;
///`read()` method returns [otg_hs_dieptxf1::R](otg_hs_dieptxf1::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF1 {}
///`write(|w| ..)` method takes [otg_hs_dieptxf1::W](otg_hs_dieptxf1::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF1 {}
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf1;
///OTG_HS device IN endpoint transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf2](otg_hs_dieptxf2) module
pub type OTG_HS_DIEPTXF2 = crate::Reg<u32, _OTG_HS_DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF2;
///`read()` method returns [otg_hs_dieptxf2::R](otg_hs_dieptxf2::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF2 {}
///`write(|w| ..)` method takes [otg_hs_dieptxf2::W](otg_hs_dieptxf2::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF2 {}
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf2;
///OTG_HS device IN endpoint transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf3](otg_hs_dieptxf3) module
pub type OTG_HS_DIEPTXF3 = crate::Reg<u32, _OTG_HS_DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF3;
///`read()` method returns [otg_hs_dieptxf3::R](otg_hs_dieptxf3::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF3 {}
///`write(|w| ..)` method takes [otg_hs_dieptxf3::W](otg_hs_dieptxf3::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF3 {}
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf3;
///OTG_HS device IN endpoint transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf4](otg_hs_dieptxf4) module
pub type OTG_HS_DIEPTXF4 = crate::Reg<u32, _OTG_HS_DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF4;
///`read()` method returns [otg_hs_dieptxf4::R](otg_hs_dieptxf4::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF4 {}
///`write(|w| ..)` method takes [otg_hs_dieptxf4::W](otg_hs_dieptxf4::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF4 {}
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf4;
///OTG_HS device IN endpoint transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf5](otg_hs_dieptxf5) module
pub type OTG_HS_DIEPTXF5 = crate::Reg<u32, _OTG_HS_DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF5;
///`read()` method returns [otg_hs_dieptxf5::R](otg_hs_dieptxf5::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF5 {}
///`write(|w| ..)` method takes [otg_hs_dieptxf5::W](otg_hs_dieptxf5::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF5 {}
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf5;
///OTG_HS device IN endpoint transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf6](otg_hs_dieptxf6) module
pub type OTG_HS_DIEPTXF6 = crate::Reg<u32, _OTG_HS_DIEPTXF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF6;
///`read()` method returns [otg_hs_dieptxf6::R](otg_hs_dieptxf6::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF6 {}
///`write(|w| ..)` method takes [otg_hs_dieptxf6::W](otg_hs_dieptxf6::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF6 {}
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf6;
///OTG_HS device IN endpoint transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptxf7](otg_hs_dieptxf7) module
pub type OTG_HS_DIEPTXF7 = crate::Reg<u32, _OTG_HS_DIEPTXF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF7;
///`read()` method returns [otg_hs_dieptxf7::R](otg_hs_dieptxf7::R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF7 {}
///`write(|w| ..)` method takes [otg_hs_dieptxf7::W](otg_hs_dieptxf7::W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF7 {}
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf7;
///OTG_HS Receive status debug read register (peripheral mode mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_grxstsr_device](otg_hs_grxstsr_device) module
pub type OTG_HS_GRXSTSR_DEVICE = crate::Reg<u32, _OTG_HS_GRXSTSR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSR_DEVICE;
///`read()` method returns [otg_hs_grxstsr_device::R](otg_hs_grxstsr_device::R) reader structure
impl crate::Readable for OTG_HS_GRXSTSR_DEVICE {}
///OTG_HS Receive status debug read register (peripheral mode mode)
pub mod otg_hs_grxstsr_device;
///OTG_HS status read and pop register (peripheral mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_grxstsp_device](otg_hs_grxstsp_device) module
pub type OTG_HS_GRXSTSP_DEVICE = crate::Reg<u32, _OTG_HS_GRXSTSP_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSP_DEVICE;
///`read()` method returns [otg_hs_grxstsp_device::R](otg_hs_grxstsp_device::R) reader structure
impl crate::Readable for OTG_HS_GRXSTSP_DEVICE {}
///OTG_HS status read and pop register (peripheral mode)
pub mod otg_hs_grxstsp_device;
///OTG core LPM configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_glpmcfg](otg_hs_glpmcfg) module
pub type OTG_HS_GLPMCFG = crate::Reg<u32, _OTG_HS_GLPMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GLPMCFG;
///`read()` method returns [otg_hs_glpmcfg::R](otg_hs_glpmcfg::R) reader structure
impl crate::Readable for OTG_HS_GLPMCFG {}
///`write(|w| ..)` method takes [otg_hs_glpmcfg::W](otg_hs_glpmcfg::W) writer structure
impl crate::Writable for OTG_HS_GLPMCFG {}
///OTG core LPM configuration register
pub mod otg_hs_glpmcfg;
