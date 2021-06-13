///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)
    pub otg_fs_gotgctl: OTG_FS_GOTGCTL,
    ///0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)
    pub otg_fs_gotgint: OTG_FS_GOTGINT,
    ///0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
    pub otg_fs_gahbcfg: OTG_FS_GAHBCFG,
    ///0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)
    pub otg_fs_gusbcfg: OTG_FS_GUSBCFG,
    ///0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)
    pub otg_fs_grstctl: OTG_FS_GRSTCTL,
    ///0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)
    pub otg_fs_gintsts: OTG_FS_GINTSTS,
    ///0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)
    pub otg_fs_gintmsk: OTG_FS_GINTMSK,
    _reserved_7_otg_fs_grxstsr: [u8; 4usize],
    _reserved_8_otg_fs_grxstsp: [u8; 4usize],
    ///0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
    pub otg_fs_grxfsiz: OTG_FS_GRXFSIZ,
    _reserved_10_otg_fs: [u8; 4usize],
    ///0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
    pub otg_fs_hnptxsts: OTG_FS_HNPTXSTS,
    ///0x30 - OTG I2C access register
    pub otg_fs_gi2cctl: OTG_FS_GI2CCTL,
    _reserved13: [u8; 4usize],
    ///0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)
    pub otg_fs_gccfg: OTG_FS_GCCFG,
    ///0x3c - core ID register
    pub otg_fs_cid: OTG_FS_CID,
    _reserved15: [u8; 20usize],
    ///0x54 - OTG core LPM configuration register
    pub otg_fs_glpmcfg: OTG_FS_GLPMCFG,
    ///0x58 - OTG power down register
    pub otg_fs_gpwrdn: OTG_FS_GPWRDN,
    _reserved17: [u8; 4usize],
    ///0x60 - OTG ADP timer, control and status register
    pub otg_fs_gadpctl: OTG_FS_GADPCTL,
    _reserved18: [u8; 156usize],
    ///0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
    pub otg_fs_hptxfsiz: OTG_FS_HPTXFSIZ,
    ///0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)
    pub otg_fs_dieptxf1: OTG_FS_DIEPTXF1,
    ///0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    pub otg_fs_dieptxf2: OTG_FS_DIEPTXF2,
    ///0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
    pub otg_fs_dieptxf3: OTG_FS_DIEPTXF3,
    ///0x110 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
    pub otg_fs_dieptxf4: OTG_FS_DIEPTXF4,
    ///0x114 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
    pub otg_fs_dieptxf5: OTG_FS_DIEPTXF5,
}
impl RegisterBlock {
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub fn otg_fs_grxstsr_host(&self) -> &OTG_FS_GRXSTSR_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_FS_GRXSTSR_HOST)
        }
    }
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub fn otg_fs_grxstsr_host_mut(&self) -> &mut OTG_FS_GRXSTSR_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OTG_FS_GRXSTSR_HOST)
        }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub fn otg_fs_grxstsr_device(&self) -> &OTG_FS_GRXSTSR_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_FS_GRXSTSR_DEVICE)
        }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub fn otg_fs_grxstsr_device_mut(&self) -> &mut OTG_FS_GRXSTSR_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OTG_FS_GRXSTSR_DEVICE)
        }
    }
    ///0x20 - OTG status read and pop register (Host mode)
    #[inline(always)]
    pub fn otg_fs_grxstsp_host(&self) -> &OTG_FS_GRXSTSP_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_FS_GRXSTSP_HOST)
        }
    }
    ///0x20 - OTG status read and pop register (Host mode)
    #[inline(always)]
    pub fn otg_fs_grxstsp_host_mut(&self) -> &mut OTG_FS_GRXSTSP_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OTG_FS_GRXSTSP_HOST)
        }
    }
    ///0x20 - OTG status read and pop register (Device mode)
    #[inline(always)]
    pub fn otg_fs_grxstsp_device(&self) -> &OTG_FS_GRXSTSP_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_FS_GRXSTSP_DEVICE)
        }
    }
    ///0x20 - OTG status read and pop register (Device mode)
    #[inline(always)]
    pub fn otg_fs_grxstsp_device_mut(&self) -> &mut OTG_FS_GRXSTSP_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OTG_FS_GRXSTSP_DEVICE)
        }
    }
    ///0x28 - OTG_FS Host non-periodic transmit FIFO size register
    #[inline(always)]
    pub fn otg_fs_hnptxfsiz_host(&self) -> &OTG_FS_HNPTXFSIZ_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_FS_HNPTXFSIZ_HOST)
        }
    }
    ///0x28 - OTG_FS Host non-periodic transmit FIFO size register
    #[inline(always)]
    pub fn otg_fs_hnptxfsiz_host_mut(&self) -> &mut OTG_FS_HNPTXFSIZ_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut OTG_FS_HNPTXFSIZ_HOST)
        }
    }
    ///0x28 - OTG_FS Endpoint 0 Transmit FIFO size
    #[inline(always)]
    pub fn otg_fs_dieptxf0_device(&self) -> &OTG_FS_DIEPTXF0_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_FS_DIEPTXF0_DEVICE)
        }
    }
    ///0x28 - OTG_FS Endpoint 0 Transmit FIFO size
    #[inline(always)]
    pub fn otg_fs_dieptxf0_device_mut(&self) -> &mut OTG_FS_DIEPTXF0_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut OTG_FS_DIEPTXF0_DEVICE)
        }
    }
}
///OTG_FS control and status register (OTG_FS_GOTGCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gotgctl](otg_fs_gotgctl) module
pub type OTG_FS_GOTGCTL = crate::Reg<u32, _OTG_FS_GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GOTGCTL;
///`read()` method returns [otg_fs_gotgctl::R](otg_fs_gotgctl::R) reader structure
impl crate::Readable for OTG_FS_GOTGCTL {}
///`write(|w| ..)` method takes [otg_fs_gotgctl::W](otg_fs_gotgctl::W) writer structure
impl crate::Writable for OTG_FS_GOTGCTL {}
///OTG_FS control and status register (OTG_FS_GOTGCTL)
pub mod otg_fs_gotgctl;
///OTG_FS interrupt register (OTG_FS_GOTGINT)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gotgint](otg_fs_gotgint) module
pub type OTG_FS_GOTGINT = crate::Reg<u32, _OTG_FS_GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GOTGINT;
///`read()` method returns [otg_fs_gotgint::R](otg_fs_gotgint::R) reader structure
impl crate::Readable for OTG_FS_GOTGINT {}
///`write(|w| ..)` method takes [otg_fs_gotgint::W](otg_fs_gotgint::W) writer structure
impl crate::Writable for OTG_FS_GOTGINT {}
///OTG_FS interrupt register (OTG_FS_GOTGINT)
pub mod otg_fs_gotgint;
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gahbcfg](otg_fs_gahbcfg) module
pub type OTG_FS_GAHBCFG = crate::Reg<u32, _OTG_FS_GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GAHBCFG;
///`read()` method returns [otg_fs_gahbcfg::R](otg_fs_gahbcfg::R) reader structure
impl crate::Readable for OTG_FS_GAHBCFG {}
///`write(|w| ..)` method takes [otg_fs_gahbcfg::W](otg_fs_gahbcfg::W) writer structure
impl crate::Writable for OTG_FS_GAHBCFG {}
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
pub mod otg_fs_gahbcfg;
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gusbcfg](otg_fs_gusbcfg) module
pub type OTG_FS_GUSBCFG = crate::Reg<u32, _OTG_FS_GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GUSBCFG;
///`read()` method returns [otg_fs_gusbcfg::R](otg_fs_gusbcfg::R) reader structure
impl crate::Readable for OTG_FS_GUSBCFG {}
///`write(|w| ..)` method takes [otg_fs_gusbcfg::W](otg_fs_gusbcfg::W) writer structure
impl crate::Writable for OTG_FS_GUSBCFG {}
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
pub mod otg_fs_gusbcfg;
///OTG_FS reset register (OTG_FS_GRSTCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_grstctl](otg_fs_grstctl) module
pub type OTG_FS_GRSTCTL = crate::Reg<u32, _OTG_FS_GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRSTCTL;
///`read()` method returns [otg_fs_grstctl::R](otg_fs_grstctl::R) reader structure
impl crate::Readable for OTG_FS_GRSTCTL {}
///`write(|w| ..)` method takes [otg_fs_grstctl::W](otg_fs_grstctl::W) writer structure
impl crate::Writable for OTG_FS_GRSTCTL {}
///OTG_FS reset register (OTG_FS_GRSTCTL)
pub mod otg_fs_grstctl;
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gintsts](otg_fs_gintsts) module
pub type OTG_FS_GINTSTS = crate::Reg<u32, _OTG_FS_GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GINTSTS;
///`read()` method returns [otg_fs_gintsts::R](otg_fs_gintsts::R) reader structure
impl crate::Readable for OTG_FS_GINTSTS {}
///`write(|w| ..)` method takes [otg_fs_gintsts::W](otg_fs_gintsts::W) writer structure
impl crate::Writable for OTG_FS_GINTSTS {}
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
pub mod otg_fs_gintsts;
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gintmsk](otg_fs_gintmsk) module
pub type OTG_FS_GINTMSK = crate::Reg<u32, _OTG_FS_GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GINTMSK;
///`read()` method returns [otg_fs_gintmsk::R](otg_fs_gintmsk::R) reader structure
impl crate::Readable for OTG_FS_GINTMSK {}
///`write(|w| ..)` method takes [otg_fs_gintmsk::W](otg_fs_gintmsk::W) writer structure
impl crate::Writable for OTG_FS_GINTMSK {}
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
pub mod otg_fs_gintmsk;
///OTG_FS Receive status debug read(Device mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_grxstsr_device](otg_fs_grxstsr_device) module
pub type OTG_FS_GRXSTSR_DEVICE = crate::Reg<u32, _OTG_FS_GRXSTSR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSR_DEVICE;
///`read()` method returns [otg_fs_grxstsr_device::R](otg_fs_grxstsr_device::R) reader structure
impl crate::Readable for OTG_FS_GRXSTSR_DEVICE {}
///OTG_FS Receive status debug read(Device mode)
pub mod otg_fs_grxstsr_device;
///OTG_FS Receive status debug read(Host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_grxstsr_host](otg_fs_grxstsr_host) module
pub type OTG_FS_GRXSTSR_HOST = crate::Reg<u32, _OTG_FS_GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSR_HOST;
///`read()` method returns [otg_fs_grxstsr_host::R](otg_fs_grxstsr_host::R) reader structure
impl crate::Readable for OTG_FS_GRXSTSR_HOST {}
///OTG_FS Receive status debug read(Host mode)
pub mod otg_fs_grxstsr_host;
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_grxfsiz](otg_fs_grxfsiz) module
pub type OTG_FS_GRXFSIZ = crate::Reg<u32, _OTG_FS_GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXFSIZ;
///`read()` method returns [otg_fs_grxfsiz::R](otg_fs_grxfsiz::R) reader structure
impl crate::Readable for OTG_FS_GRXFSIZ {}
///`write(|w| ..)` method takes [otg_fs_grxfsiz::W](otg_fs_grxfsiz::W) writer structure
impl crate::Writable for OTG_FS_GRXFSIZ {}
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
pub mod otg_fs_grxfsiz;
///OTG_FS Endpoint 0 Transmit FIFO size
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_dieptxf0_device](otg_fs_dieptxf0_device) module
pub type OTG_FS_DIEPTXF0_DEVICE = crate::Reg<u32, _OTG_FS_DIEPTXF0_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF0_DEVICE;
///`read()` method returns [otg_fs_dieptxf0_device::R](otg_fs_dieptxf0_device::R) reader structure
impl crate::Readable for OTG_FS_DIEPTXF0_DEVICE {}
///`write(|w| ..)` method takes [otg_fs_dieptxf0_device::W](otg_fs_dieptxf0_device::W) writer structure
impl crate::Writable for OTG_FS_DIEPTXF0_DEVICE {}
///OTG_FS Endpoint 0 Transmit FIFO size
pub mod otg_fs_dieptxf0_device;
///OTG_FS Host non-periodic transmit FIFO size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hnptxfsiz_host](otg_fs_hnptxfsiz_host) module
pub type OTG_FS_HNPTXFSIZ_HOST = crate::Reg<u32, _OTG_FS_HNPTXFSIZ_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HNPTXFSIZ_HOST;
///`read()` method returns [otg_fs_hnptxfsiz_host::R](otg_fs_hnptxfsiz_host::R) reader structure
impl crate::Readable for OTG_FS_HNPTXFSIZ_HOST {}
///`write(|w| ..)` method takes [otg_fs_hnptxfsiz_host::W](otg_fs_hnptxfsiz_host::W) writer structure
impl crate::Writable for OTG_FS_HNPTXFSIZ_HOST {}
///OTG_FS Host non-periodic transmit FIFO size register
pub mod otg_fs_hnptxfsiz_host;
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hnptxsts](otg_fs_hnptxsts) module
pub type OTG_FS_HNPTXSTS = crate::Reg<u32, _OTG_FS_HNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HNPTXSTS;
///`read()` method returns [otg_fs_hnptxsts::R](otg_fs_hnptxsts::R) reader structure
impl crate::Readable for OTG_FS_HNPTXSTS {}
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
pub mod otg_fs_hnptxsts;
///OTG_FS general core configuration register (OTG_FS_GCCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gccfg](otg_fs_gccfg) module
pub type OTG_FS_GCCFG = crate::Reg<u32, _OTG_FS_GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GCCFG;
///`read()` method returns [otg_fs_gccfg::R](otg_fs_gccfg::R) reader structure
impl crate::Readable for OTG_FS_GCCFG {}
///`write(|w| ..)` method takes [otg_fs_gccfg::W](otg_fs_gccfg::W) writer structure
impl crate::Writable for OTG_FS_GCCFG {}
///OTG_FS general core configuration register (OTG_FS_GCCFG)
pub mod otg_fs_gccfg;
///core ID register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_cid](otg_fs_cid) module
pub type OTG_FS_CID = crate::Reg<u32, _OTG_FS_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_CID;
///`read()` method returns [otg_fs_cid::R](otg_fs_cid::R) reader structure
impl crate::Readable for OTG_FS_CID {}
///`write(|w| ..)` method takes [otg_fs_cid::W](otg_fs_cid::W) writer structure
impl crate::Writable for OTG_FS_CID {}
///core ID register
pub mod otg_fs_cid;
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hptxfsiz](otg_fs_hptxfsiz) module
pub type OTG_FS_HPTXFSIZ = crate::Reg<u32, _OTG_FS_HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HPTXFSIZ;
///`read()` method returns [otg_fs_hptxfsiz::R](otg_fs_hptxfsiz::R) reader structure
impl crate::Readable for OTG_FS_HPTXFSIZ {}
///`write(|w| ..)` method takes [otg_fs_hptxfsiz::W](otg_fs_hptxfsiz::W) writer structure
impl crate::Writable for OTG_FS_HPTXFSIZ {}
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
pub mod otg_fs_hptxfsiz;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_dieptxf1](otg_fs_dieptxf1) module
pub type OTG_FS_DIEPTXF1 = crate::Reg<u32, _OTG_FS_DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF1;
///`read()` method returns [otg_fs_dieptxf1::R](otg_fs_dieptxf1::R) reader structure
impl crate::Readable for OTG_FS_DIEPTXF1 {}
///`write(|w| ..)` method takes [otg_fs_dieptxf1::W](otg_fs_dieptxf1::W) writer structure
impl crate::Writable for OTG_FS_DIEPTXF1 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)
pub mod otg_fs_dieptxf1;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_dieptxf2](otg_fs_dieptxf2) module
pub type OTG_FS_DIEPTXF2 = crate::Reg<u32, _OTG_FS_DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF2;
///`read()` method returns [otg_fs_dieptxf2::R](otg_fs_dieptxf2::R) reader structure
impl crate::Readable for OTG_FS_DIEPTXF2 {}
///`write(|w| ..)` method takes [otg_fs_dieptxf2::W](otg_fs_dieptxf2::W) writer structure
impl crate::Writable for OTG_FS_DIEPTXF2 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
pub mod otg_fs_dieptxf2;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_dieptxf3](otg_fs_dieptxf3) module
pub type OTG_FS_DIEPTXF3 = crate::Reg<u32, _OTG_FS_DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF3;
///`read()` method returns [otg_fs_dieptxf3::R](otg_fs_dieptxf3::R) reader structure
impl crate::Readable for OTG_FS_DIEPTXF3 {}
///`write(|w| ..)` method takes [otg_fs_dieptxf3::W](otg_fs_dieptxf3::W) writer structure
impl crate::Writable for OTG_FS_DIEPTXF3 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
pub mod otg_fs_dieptxf3;
///OTG status read and pop register (Device mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_grxstsp_device](otg_fs_grxstsp_device) module
pub type OTG_FS_GRXSTSP_DEVICE = crate::Reg<u32, _OTG_FS_GRXSTSP_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSP_DEVICE;
///`read()` method returns [otg_fs_grxstsp_device::R](otg_fs_grxstsp_device::R) reader structure
impl crate::Readable for OTG_FS_GRXSTSP_DEVICE {}
///OTG status read and pop register (Device mode)
pub mod otg_fs_grxstsp_device;
///OTG status read and pop register (Host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_grxstsp_host](otg_fs_grxstsp_host) module
pub type OTG_FS_GRXSTSP_HOST = crate::Reg<u32, _OTG_FS_GRXSTSP_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSP_HOST;
///`read()` method returns [otg_fs_grxstsp_host::R](otg_fs_grxstsp_host::R) reader structure
impl crate::Readable for OTG_FS_GRXSTSP_HOST {}
///OTG status read and pop register (Host mode)
pub mod otg_fs_grxstsp_host;
///OTG I2C access register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gi2cctl](otg_fs_gi2cctl) module
pub type OTG_FS_GI2CCTL = crate::Reg<u32, _OTG_FS_GI2CCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GI2CCTL;
///`read()` method returns [otg_fs_gi2cctl::R](otg_fs_gi2cctl::R) reader structure
impl crate::Readable for OTG_FS_GI2CCTL {}
///`write(|w| ..)` method takes [otg_fs_gi2cctl::W](otg_fs_gi2cctl::W) writer structure
impl crate::Writable for OTG_FS_GI2CCTL {}
///OTG I2C access register
pub mod otg_fs_gi2cctl;
///OTG power down register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gpwrdn](otg_fs_gpwrdn) module
pub type OTG_FS_GPWRDN = crate::Reg<u32, _OTG_FS_GPWRDN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GPWRDN;
///`read()` method returns [otg_fs_gpwrdn::R](otg_fs_gpwrdn::R) reader structure
impl crate::Readable for OTG_FS_GPWRDN {}
///`write(|w| ..)` method takes [otg_fs_gpwrdn::W](otg_fs_gpwrdn::W) writer structure
impl crate::Writable for OTG_FS_GPWRDN {}
///OTG power down register
pub mod otg_fs_gpwrdn;
///OTG ADP timer, control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_gadpctl](otg_fs_gadpctl) module
pub type OTG_FS_GADPCTL = crate::Reg<u32, _OTG_FS_GADPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GADPCTL;
///`read()` method returns [otg_fs_gadpctl::R](otg_fs_gadpctl::R) reader structure
impl crate::Readable for OTG_FS_GADPCTL {}
///`write(|w| ..)` method takes [otg_fs_gadpctl::W](otg_fs_gadpctl::W) writer structure
impl crate::Writable for OTG_FS_GADPCTL {}
///OTG ADP timer, control and status register
pub mod otg_fs_gadpctl;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_dieptxf4](otg_fs_dieptxf4) module
pub type OTG_FS_DIEPTXF4 = crate::Reg<u32, _OTG_FS_DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF4;
///`read()` method returns [otg_fs_dieptxf4::R](otg_fs_dieptxf4::R) reader structure
impl crate::Readable for OTG_FS_DIEPTXF4 {}
///`write(|w| ..)` method takes [otg_fs_dieptxf4::W](otg_fs_dieptxf4::W) writer structure
impl crate::Writable for OTG_FS_DIEPTXF4 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
pub mod otg_fs_dieptxf4;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_dieptxf5](otg_fs_dieptxf5) module
pub type OTG_FS_DIEPTXF5 = crate::Reg<u32, _OTG_FS_DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF5;
///`read()` method returns [otg_fs_dieptxf5::R](otg_fs_dieptxf5::R) reader structure
impl crate::Readable for OTG_FS_DIEPTXF5 {}
///`write(|w| ..)` method takes [otg_fs_dieptxf5::W](otg_fs_dieptxf5::W) writer structure
impl crate::Writable for OTG_FS_DIEPTXF5 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
pub mod otg_fs_dieptxf5;
///OTG core LPM configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_glpmcfg](otg_fs_glpmcfg) module
pub type OTG_FS_GLPMCFG = crate::Reg<u32, _OTG_FS_GLPMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GLPMCFG;
///`read()` method returns [otg_fs_glpmcfg::R](otg_fs_glpmcfg::R) reader structure
impl crate::Readable for OTG_FS_GLPMCFG {}
///`write(|w| ..)` method takes [otg_fs_glpmcfg::W](otg_fs_glpmcfg::W) writer structure
impl crate::Writable for OTG_FS_GLPMCFG {}
///OTG core LPM configuration register
pub mod otg_fs_glpmcfg;
