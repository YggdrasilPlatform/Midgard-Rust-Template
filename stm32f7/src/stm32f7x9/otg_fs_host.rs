///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS host configuration register (OTG_FS_HCFG)
    pub otg_fs_hcfg: OTG_FS_HCFG,
    ///0x04 - OTG_FS Host frame interval register
    pub otg_fs_hfir: OTG_FS_HFIR,
    ///0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
    pub otg_fs_hfnum: OTG_FS_HFNUM,
    _reserved3: [u8; 4usize],
    ///0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
    pub otg_fs_hptxsts: OTG_FS_HPTXSTS,
    ///0x14 - OTG_FS Host all channels interrupt register
    pub otg_fs_haint: OTG_FS_HAINT,
    ///0x18 - OTG_FS host all channels interrupt mask register
    pub otg_fs_haintmsk: OTG_FS_HAINTMSK,
    _reserved6: [u8; 36usize],
    ///0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)
    pub otg_fs_hprt: OTG_FS_HPRT,
    _reserved7: [u8; 188usize],
    ///0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub otg_fs_hcchar0: OTG_FS_HCCHAR0,
    _reserved8: [u8; 4usize],
    ///0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub otg_fs_hcint0: OTG_FS_HCINT0,
    ///0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub otg_fs_hcintmsk0: OTG_FS_HCINTMSK0,
    ///0x110 - OTG_FS host channel-0 transfer size register
    pub otg_fs_hctsiz0: OTG_FS_HCTSIZ0,
    _reserved11: [u8; 12usize],
    ///0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
    pub otg_fs_hcchar1: OTG_FS_HCCHAR1,
    _reserved12: [u8; 4usize],
    ///0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
    pub otg_fs_hcint1: OTG_FS_HCINT1,
    ///0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
    pub otg_fs_hcintmsk1: OTG_FS_HCINTMSK1,
    ///0x130 - OTG_FS host channel-1 transfer size register
    pub otg_fs_hctsiz1: OTG_FS_HCTSIZ1,
    _reserved15: [u8; 12usize],
    ///0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
    pub otg_fs_hcchar2: OTG_FS_HCCHAR2,
    _reserved16: [u8; 4usize],
    ///0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
    pub otg_fs_hcint2: OTG_FS_HCINT2,
    ///0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
    pub otg_fs_hcintmsk2: OTG_FS_HCINTMSK2,
    ///0x150 - OTG_FS host channel-2 transfer size register
    pub otg_fs_hctsiz2: OTG_FS_HCTSIZ2,
    _reserved19: [u8; 12usize],
    ///0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
    pub otg_fs_hcchar3: OTG_FS_HCCHAR3,
    _reserved20: [u8; 4usize],
    ///0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
    pub otg_fs_hcint3: OTG_FS_HCINT3,
    ///0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
    pub otg_fs_hcintmsk3: OTG_FS_HCINTMSK3,
    ///0x170 - OTG_FS host channel-3 transfer size register
    pub otg_fs_hctsiz3: OTG_FS_HCTSIZ3,
    _reserved23: [u8; 12usize],
    ///0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
    pub otg_fs_hcchar4: OTG_FS_HCCHAR4,
    _reserved24: [u8; 4usize],
    ///0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
    pub otg_fs_hcint4: OTG_FS_HCINT4,
    ///0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
    pub otg_fs_hcintmsk4: OTG_FS_HCINTMSK4,
    ///0x190 - OTG_FS host channel-x transfer size register
    pub otg_fs_hctsiz4: OTG_FS_HCTSIZ4,
    _reserved27: [u8; 12usize],
    ///0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
    pub otg_fs_hcchar5: OTG_FS_HCCHAR5,
    _reserved28: [u8; 4usize],
    ///0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
    pub otg_fs_hcint5: OTG_FS_HCINT5,
    ///0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
    pub otg_fs_hcintmsk5: OTG_FS_HCINTMSK5,
    ///0x1b0 - OTG_FS host channel-5 transfer size register
    pub otg_fs_hctsiz5: OTG_FS_HCTSIZ5,
    _reserved31: [u8; 12usize],
    ///0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
    pub otg_fs_hcchar6: OTG_FS_HCCHAR6,
    _reserved32: [u8; 4usize],
    ///0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
    pub otg_fs_hcint6: OTG_FS_HCINT6,
    ///0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
    pub otg_fs_hcintmsk6: OTG_FS_HCINTMSK6,
    ///0x1d0 - OTG_FS host channel-6 transfer size register
    pub otg_fs_hctsiz6: OTG_FS_HCTSIZ6,
    _reserved35: [u8; 12usize],
    ///0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
    pub otg_fs_hcchar7: OTG_FS_HCCHAR7,
    _reserved36: [u8; 4usize],
    ///0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
    pub otg_fs_hcint7: OTG_FS_HCINT7,
    ///0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
    pub otg_fs_hcintmsk7: OTG_FS_HCINTMSK7,
    ///0x1f0 - OTG_FS host channel-7 transfer size register
    pub otg_fs_hctsiz7: OTG_FS_HCTSIZ7,
    ///0x1f4 - OTG_FS host channel-8 characteristics register
    pub otg_fs_hcchar8: OTG_FS_HCCHAR8,
    ///0x1f8 - OTG_FS host channel-8 interrupt register
    pub otg_fs_hcint8: OTG_FS_HCINT8,
    ///0x1fc - OTG_FS host channel-8 mask register
    pub otg_fs_hcintmsk8: OTG_FS_HCINTMSK8,
    ///0x200 - OTG_FS host channel-8 transfer size register
    pub otg_fs_hctsiz8: OTG_FS_HCTSIZ8,
    ///0x204 - OTG_FS host channel-9 characteristics register
    pub otg_fs_hcchar9: OTG_FS_HCCHAR9,
    ///0x208 - OTG_FS host channel-9 interrupt register
    pub otg_fs_hcint9: OTG_FS_HCINT9,
    ///0x20c - OTG_FS host channel-9 mask register
    pub otg_fs_hcintmsk9: OTG_FS_HCINTMSK9,
    ///0x210 - OTG_FS host channel-9 transfer size register
    pub otg_fs_hctsiz9: OTG_FS_HCTSIZ9,
    ///0x214 - OTG_FS host channel-10 characteristics register
    pub otg_fs_hcchar10: OTG_FS_HCCHAR10,
    ///0x218 - OTG_FS host channel-10 interrupt register
    pub otg_fs_hcint10: OTG_FS_HCINT10,
    ///0x21c - OTG_FS host channel-10 mask register
    pub otg_fs_hcintmsk10: OTG_FS_HCINTMSK10,
    ///0x220 - OTG_FS host channel-10 transfer size register
    pub otg_fs_hctsiz10: OTG_FS_HCTSIZ10,
    ///0x224 - OTG_FS host channel-11 characteristics register
    pub otg_fs_hcchar11: OTG_FS_HCCHAR11,
    ///0x228 - OTG_FS host channel-11 interrupt register
    pub otg_fs_hcint11: OTG_FS_HCINT11,
    ///0x22c - OTG_FS host channel-11 mask register
    pub otg_fs_hcintmsk11: OTG_FS_HCINTMSK11,
    ///0x230 - OTG_FS host channel-11 transfer size register
    pub otg_fs_hctsiz11: OTG_FS_HCTSIZ11,
}
///OTG_FS host configuration register (OTG_FS_HCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcfg](otg_fs_hcfg) module
pub type OTG_FS_HCFG = crate::Reg<u32, _OTG_FS_HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCFG;
///`read()` method returns [otg_fs_hcfg::R](otg_fs_hcfg::R) reader structure
impl crate::Readable for OTG_FS_HCFG {}
///`write(|w| ..)` method takes [otg_fs_hcfg::W](otg_fs_hcfg::W) writer structure
impl crate::Writable for OTG_FS_HCFG {}
///OTG_FS host configuration register (OTG_FS_HCFG)
pub mod otg_fs_hcfg;
///OTG_FS Host frame interval register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hfir](otg_fs_hfir) module
pub type OTG_FS_HFIR = crate::Reg<u32, _OTG_FS_HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HFIR;
///`read()` method returns [otg_fs_hfir::R](otg_fs_hfir::R) reader structure
impl crate::Readable for OTG_FS_HFIR {}
///`write(|w| ..)` method takes [otg_fs_hfir::W](otg_fs_hfir::W) writer structure
impl crate::Writable for OTG_FS_HFIR {}
///OTG_FS Host frame interval register
pub mod otg_fs_hfir;
///OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hfnum](otg_fs_hfnum) module
pub type OTG_FS_HFNUM = crate::Reg<u32, _OTG_FS_HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HFNUM;
///`read()` method returns [otg_fs_hfnum::R](otg_fs_hfnum::R) reader structure
impl crate::Readable for OTG_FS_HFNUM {}
///OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
pub mod otg_fs_hfnum;
///OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hptxsts](otg_fs_hptxsts) module
pub type OTG_FS_HPTXSTS = crate::Reg<u32, _OTG_FS_HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HPTXSTS;
///`read()` method returns [otg_fs_hptxsts::R](otg_fs_hptxsts::R) reader structure
impl crate::Readable for OTG_FS_HPTXSTS {}
///`write(|w| ..)` method takes [otg_fs_hptxsts::W](otg_fs_hptxsts::W) writer structure
impl crate::Writable for OTG_FS_HPTXSTS {}
///OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
pub mod otg_fs_hptxsts;
///OTG_FS Host all channels interrupt register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_haint](otg_fs_haint) module
pub type OTG_FS_HAINT = crate::Reg<u32, _OTG_FS_HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HAINT;
///`read()` method returns [otg_fs_haint::R](otg_fs_haint::R) reader structure
impl crate::Readable for OTG_FS_HAINT {}
///OTG_FS Host all channels interrupt register
pub mod otg_fs_haint;
///OTG_FS host all channels interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_haintmsk](otg_fs_haintmsk) module
pub type OTG_FS_HAINTMSK = crate::Reg<u32, _OTG_FS_HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HAINTMSK;
///`read()` method returns [otg_fs_haintmsk::R](otg_fs_haintmsk::R) reader structure
impl crate::Readable for OTG_FS_HAINTMSK {}
///`write(|w| ..)` method takes [otg_fs_haintmsk::W](otg_fs_haintmsk::W) writer structure
impl crate::Writable for OTG_FS_HAINTMSK {}
///OTG_FS host all channels interrupt mask register
pub mod otg_fs_haintmsk;
///OTG_FS host port control and status register (OTG_FS_HPRT)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hprt](otg_fs_hprt) module
pub type OTG_FS_HPRT = crate::Reg<u32, _OTG_FS_HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HPRT;
///`read()` method returns [otg_fs_hprt::R](otg_fs_hprt::R) reader structure
impl crate::Readable for OTG_FS_HPRT {}
///`write(|w| ..)` method takes [otg_fs_hprt::W](otg_fs_hprt::W) writer structure
impl crate::Writable for OTG_FS_HPRT {}
///OTG_FS host port control and status register (OTG_FS_HPRT)
pub mod otg_fs_hprt;
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar0](otg_fs_hcchar0) module
pub type OTG_FS_HCCHAR0 = crate::Reg<u32, _OTG_FS_HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR0;
///`read()` method returns [otg_fs_hcchar0::R](otg_fs_hcchar0::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR0 {}
///`write(|w| ..)` method takes [otg_fs_hcchar0::W](otg_fs_hcchar0::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR0 {}
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod otg_fs_hcchar0;
///OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar1](otg_fs_hcchar1) module
pub type OTG_FS_HCCHAR1 = crate::Reg<u32, _OTG_FS_HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR1;
///`read()` method returns [otg_fs_hcchar1::R](otg_fs_hcchar1::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR1 {}
///`write(|w| ..)` method takes [otg_fs_hcchar1::W](otg_fs_hcchar1::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR1 {}
///OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
pub mod otg_fs_hcchar1;
///OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar2](otg_fs_hcchar2) module
pub type OTG_FS_HCCHAR2 = crate::Reg<u32, _OTG_FS_HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR2;
///`read()` method returns [otg_fs_hcchar2::R](otg_fs_hcchar2::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR2 {}
///`write(|w| ..)` method takes [otg_fs_hcchar2::W](otg_fs_hcchar2::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR2 {}
///OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
pub mod otg_fs_hcchar2;
///OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar3](otg_fs_hcchar3) module
pub type OTG_FS_HCCHAR3 = crate::Reg<u32, _OTG_FS_HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR3;
///`read()` method returns [otg_fs_hcchar3::R](otg_fs_hcchar3::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR3 {}
///`write(|w| ..)` method takes [otg_fs_hcchar3::W](otg_fs_hcchar3::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR3 {}
///OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
pub mod otg_fs_hcchar3;
///OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar4](otg_fs_hcchar4) module
pub type OTG_FS_HCCHAR4 = crate::Reg<u32, _OTG_FS_HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR4;
///`read()` method returns [otg_fs_hcchar4::R](otg_fs_hcchar4::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR4 {}
///`write(|w| ..)` method takes [otg_fs_hcchar4::W](otg_fs_hcchar4::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR4 {}
///OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
pub mod otg_fs_hcchar4;
///OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar5](otg_fs_hcchar5) module
pub type OTG_FS_HCCHAR5 = crate::Reg<u32, _OTG_FS_HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR5;
///`read()` method returns [otg_fs_hcchar5::R](otg_fs_hcchar5::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR5 {}
///`write(|w| ..)` method takes [otg_fs_hcchar5::W](otg_fs_hcchar5::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR5 {}
///OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
pub mod otg_fs_hcchar5;
///OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar6](otg_fs_hcchar6) module
pub type OTG_FS_HCCHAR6 = crate::Reg<u32, _OTG_FS_HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR6;
///`read()` method returns [otg_fs_hcchar6::R](otg_fs_hcchar6::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR6 {}
///`write(|w| ..)` method takes [otg_fs_hcchar6::W](otg_fs_hcchar6::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR6 {}
///OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
pub mod otg_fs_hcchar6;
///OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar7](otg_fs_hcchar7) module
pub type OTG_FS_HCCHAR7 = crate::Reg<u32, _OTG_FS_HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR7;
///`read()` method returns [otg_fs_hcchar7::R](otg_fs_hcchar7::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR7 {}
///`write(|w| ..)` method takes [otg_fs_hcchar7::W](otg_fs_hcchar7::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR7 {}
///OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
pub mod otg_fs_hcchar7;
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint0](otg_fs_hcint0) module
pub type OTG_FS_HCINT0 = crate::Reg<u32, _OTG_FS_HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT0;
///`read()` method returns [otg_fs_hcint0::R](otg_fs_hcint0::R) reader structure
impl crate::Readable for OTG_FS_HCINT0 {}
///`write(|w| ..)` method takes [otg_fs_hcint0::W](otg_fs_hcint0::W) writer structure
impl crate::Writable for OTG_FS_HCINT0 {}
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod otg_fs_hcint0;
///OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint1](otg_fs_hcint1) module
pub type OTG_FS_HCINT1 = crate::Reg<u32, _OTG_FS_HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT1;
///`read()` method returns [otg_fs_hcint1::R](otg_fs_hcint1::R) reader structure
impl crate::Readable for OTG_FS_HCINT1 {}
///`write(|w| ..)` method takes [otg_fs_hcint1::W](otg_fs_hcint1::W) writer structure
impl crate::Writable for OTG_FS_HCINT1 {}
///OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
pub mod otg_fs_hcint1;
///OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint2](otg_fs_hcint2) module
pub type OTG_FS_HCINT2 = crate::Reg<u32, _OTG_FS_HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT2;
///`read()` method returns [otg_fs_hcint2::R](otg_fs_hcint2::R) reader structure
impl crate::Readable for OTG_FS_HCINT2 {}
///`write(|w| ..)` method takes [otg_fs_hcint2::W](otg_fs_hcint2::W) writer structure
impl crate::Writable for OTG_FS_HCINT2 {}
///OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
pub mod otg_fs_hcint2;
///OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint3](otg_fs_hcint3) module
pub type OTG_FS_HCINT3 = crate::Reg<u32, _OTG_FS_HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT3;
///`read()` method returns [otg_fs_hcint3::R](otg_fs_hcint3::R) reader structure
impl crate::Readable for OTG_FS_HCINT3 {}
///`write(|w| ..)` method takes [otg_fs_hcint3::W](otg_fs_hcint3::W) writer structure
impl crate::Writable for OTG_FS_HCINT3 {}
///OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
pub mod otg_fs_hcint3;
///OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint4](otg_fs_hcint4) module
pub type OTG_FS_HCINT4 = crate::Reg<u32, _OTG_FS_HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT4;
///`read()` method returns [otg_fs_hcint4::R](otg_fs_hcint4::R) reader structure
impl crate::Readable for OTG_FS_HCINT4 {}
///`write(|w| ..)` method takes [otg_fs_hcint4::W](otg_fs_hcint4::W) writer structure
impl crate::Writable for OTG_FS_HCINT4 {}
///OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
pub mod otg_fs_hcint4;
///OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint5](otg_fs_hcint5) module
pub type OTG_FS_HCINT5 = crate::Reg<u32, _OTG_FS_HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT5;
///`read()` method returns [otg_fs_hcint5::R](otg_fs_hcint5::R) reader structure
impl crate::Readable for OTG_FS_HCINT5 {}
///`write(|w| ..)` method takes [otg_fs_hcint5::W](otg_fs_hcint5::W) writer structure
impl crate::Writable for OTG_FS_HCINT5 {}
///OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
pub mod otg_fs_hcint5;
///OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint6](otg_fs_hcint6) module
pub type OTG_FS_HCINT6 = crate::Reg<u32, _OTG_FS_HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT6;
///`read()` method returns [otg_fs_hcint6::R](otg_fs_hcint6::R) reader structure
impl crate::Readable for OTG_FS_HCINT6 {}
///`write(|w| ..)` method takes [otg_fs_hcint6::W](otg_fs_hcint6::W) writer structure
impl crate::Writable for OTG_FS_HCINT6 {}
///OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
pub mod otg_fs_hcint6;
///OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint7](otg_fs_hcint7) module
pub type OTG_FS_HCINT7 = crate::Reg<u32, _OTG_FS_HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT7;
///`read()` method returns [otg_fs_hcint7::R](otg_fs_hcint7::R) reader structure
impl crate::Readable for OTG_FS_HCINT7 {}
///`write(|w| ..)` method takes [otg_fs_hcint7::W](otg_fs_hcint7::W) writer structure
impl crate::Writable for OTG_FS_HCINT7 {}
///OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
pub mod otg_fs_hcint7;
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk0](otg_fs_hcintmsk0) module
pub type OTG_FS_HCINTMSK0 = crate::Reg<u32, _OTG_FS_HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK0;
///`read()` method returns [otg_fs_hcintmsk0::R](otg_fs_hcintmsk0::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK0 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk0::W](otg_fs_hcintmsk0::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK0 {}
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod otg_fs_hcintmsk0;
///OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk1](otg_fs_hcintmsk1) module
pub type OTG_FS_HCINTMSK1 = crate::Reg<u32, _OTG_FS_HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK1;
///`read()` method returns [otg_fs_hcintmsk1::R](otg_fs_hcintmsk1::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK1 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk1::W](otg_fs_hcintmsk1::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK1 {}
///OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
pub mod otg_fs_hcintmsk1;
///OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk2](otg_fs_hcintmsk2) module
pub type OTG_FS_HCINTMSK2 = crate::Reg<u32, _OTG_FS_HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK2;
///`read()` method returns [otg_fs_hcintmsk2::R](otg_fs_hcintmsk2::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK2 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk2::W](otg_fs_hcintmsk2::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK2 {}
///OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
pub mod otg_fs_hcintmsk2;
///OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk3](otg_fs_hcintmsk3) module
pub type OTG_FS_HCINTMSK3 = crate::Reg<u32, _OTG_FS_HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK3;
///`read()` method returns [otg_fs_hcintmsk3::R](otg_fs_hcintmsk3::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK3 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk3::W](otg_fs_hcintmsk3::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK3 {}
///OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
pub mod otg_fs_hcintmsk3;
///OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk4](otg_fs_hcintmsk4) module
pub type OTG_FS_HCINTMSK4 = crate::Reg<u32, _OTG_FS_HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK4;
///`read()` method returns [otg_fs_hcintmsk4::R](otg_fs_hcintmsk4::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK4 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk4::W](otg_fs_hcintmsk4::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK4 {}
///OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
pub mod otg_fs_hcintmsk4;
///OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk5](otg_fs_hcintmsk5) module
pub type OTG_FS_HCINTMSK5 = crate::Reg<u32, _OTG_FS_HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK5;
///`read()` method returns [otg_fs_hcintmsk5::R](otg_fs_hcintmsk5::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK5 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk5::W](otg_fs_hcintmsk5::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK5 {}
///OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
pub mod otg_fs_hcintmsk5;
///OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk6](otg_fs_hcintmsk6) module
pub type OTG_FS_HCINTMSK6 = crate::Reg<u32, _OTG_FS_HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK6;
///`read()` method returns [otg_fs_hcintmsk6::R](otg_fs_hcintmsk6::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK6 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk6::W](otg_fs_hcintmsk6::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK6 {}
///OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
pub mod otg_fs_hcintmsk6;
///OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk7](otg_fs_hcintmsk7) module
pub type OTG_FS_HCINTMSK7 = crate::Reg<u32, _OTG_FS_HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK7;
///`read()` method returns [otg_fs_hcintmsk7::R](otg_fs_hcintmsk7::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK7 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk7::W](otg_fs_hcintmsk7::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK7 {}
///OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
pub mod otg_fs_hcintmsk7;
///OTG_FS host channel-0 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz0](otg_fs_hctsiz0) module
pub type OTG_FS_HCTSIZ0 = crate::Reg<u32, _OTG_FS_HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ0;
///`read()` method returns [otg_fs_hctsiz0::R](otg_fs_hctsiz0::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ0 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz0::W](otg_fs_hctsiz0::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ0 {}
///OTG_FS host channel-0 transfer size register
pub mod otg_fs_hctsiz0;
///OTG_FS host channel-1 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz1](otg_fs_hctsiz1) module
pub type OTG_FS_HCTSIZ1 = crate::Reg<u32, _OTG_FS_HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ1;
///`read()` method returns [otg_fs_hctsiz1::R](otg_fs_hctsiz1::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ1 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz1::W](otg_fs_hctsiz1::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ1 {}
///OTG_FS host channel-1 transfer size register
pub mod otg_fs_hctsiz1;
///OTG_FS host channel-2 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz2](otg_fs_hctsiz2) module
pub type OTG_FS_HCTSIZ2 = crate::Reg<u32, _OTG_FS_HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ2;
///`read()` method returns [otg_fs_hctsiz2::R](otg_fs_hctsiz2::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ2 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz2::W](otg_fs_hctsiz2::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ2 {}
///OTG_FS host channel-2 transfer size register
pub mod otg_fs_hctsiz2;
///OTG_FS host channel-3 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz3](otg_fs_hctsiz3) module
pub type OTG_FS_HCTSIZ3 = crate::Reg<u32, _OTG_FS_HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ3;
///`read()` method returns [otg_fs_hctsiz3::R](otg_fs_hctsiz3::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ3 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz3::W](otg_fs_hctsiz3::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ3 {}
///OTG_FS host channel-3 transfer size register
pub mod otg_fs_hctsiz3;
///OTG_FS host channel-x transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz4](otg_fs_hctsiz4) module
pub type OTG_FS_HCTSIZ4 = crate::Reg<u32, _OTG_FS_HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ4;
///`read()` method returns [otg_fs_hctsiz4::R](otg_fs_hctsiz4::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ4 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz4::W](otg_fs_hctsiz4::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ4 {}
///OTG_FS host channel-x transfer size register
pub mod otg_fs_hctsiz4;
///OTG_FS host channel-5 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz5](otg_fs_hctsiz5) module
pub type OTG_FS_HCTSIZ5 = crate::Reg<u32, _OTG_FS_HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ5;
///`read()` method returns [otg_fs_hctsiz5::R](otg_fs_hctsiz5::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ5 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz5::W](otg_fs_hctsiz5::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ5 {}
///OTG_FS host channel-5 transfer size register
pub mod otg_fs_hctsiz5;
///OTG_FS host channel-6 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz6](otg_fs_hctsiz6) module
pub type OTG_FS_HCTSIZ6 = crate::Reg<u32, _OTG_FS_HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ6;
///`read()` method returns [otg_fs_hctsiz6::R](otg_fs_hctsiz6::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ6 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz6::W](otg_fs_hctsiz6::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ6 {}
///OTG_FS host channel-6 transfer size register
pub mod otg_fs_hctsiz6;
///OTG_FS host channel-7 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz7](otg_fs_hctsiz7) module
pub type OTG_FS_HCTSIZ7 = crate::Reg<u32, _OTG_FS_HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ7;
///`read()` method returns [otg_fs_hctsiz7::R](otg_fs_hctsiz7::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ7 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz7::W](otg_fs_hctsiz7::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ7 {}
///OTG_FS host channel-7 transfer size register
pub mod otg_fs_hctsiz7;
///OTG_FS host channel-8 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar8](otg_fs_hcchar8) module
pub type OTG_FS_HCCHAR8 = crate::Reg<u32, _OTG_FS_HCCHAR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR8;
///`read()` method returns [otg_fs_hcchar8::R](otg_fs_hcchar8::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR8 {}
///`write(|w| ..)` method takes [otg_fs_hcchar8::W](otg_fs_hcchar8::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR8 {}
///OTG_FS host channel-8 characteristics register
pub mod otg_fs_hcchar8;
///OTG_FS host channel-8 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint8](otg_fs_hcint8) module
pub type OTG_FS_HCINT8 = crate::Reg<u32, _OTG_FS_HCINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT8;
///`read()` method returns [otg_fs_hcint8::R](otg_fs_hcint8::R) reader structure
impl crate::Readable for OTG_FS_HCINT8 {}
///`write(|w| ..)` method takes [otg_fs_hcint8::W](otg_fs_hcint8::W) writer structure
impl crate::Writable for OTG_FS_HCINT8 {}
///OTG_FS host channel-8 interrupt register
pub mod otg_fs_hcint8;
///OTG_FS host channel-8 mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk8](otg_fs_hcintmsk8) module
pub type OTG_FS_HCINTMSK8 = crate::Reg<u32, _OTG_FS_HCINTMSK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK8;
///`read()` method returns [otg_fs_hcintmsk8::R](otg_fs_hcintmsk8::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK8 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk8::W](otg_fs_hcintmsk8::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK8 {}
///OTG_FS host channel-8 mask register
pub mod otg_fs_hcintmsk8;
///OTG_FS host channel-8 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz8](otg_fs_hctsiz8) module
pub type OTG_FS_HCTSIZ8 = crate::Reg<u32, _OTG_FS_HCTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ8;
///`read()` method returns [otg_fs_hctsiz8::R](otg_fs_hctsiz8::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ8 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz8::W](otg_fs_hctsiz8::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ8 {}
///OTG_FS host channel-8 transfer size register
pub mod otg_fs_hctsiz8;
///OTG_FS host channel-9 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar9](otg_fs_hcchar9) module
pub type OTG_FS_HCCHAR9 = crate::Reg<u32, _OTG_FS_HCCHAR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR9;
///`read()` method returns [otg_fs_hcchar9::R](otg_fs_hcchar9::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR9 {}
///`write(|w| ..)` method takes [otg_fs_hcchar9::W](otg_fs_hcchar9::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR9 {}
///OTG_FS host channel-9 characteristics register
pub mod otg_fs_hcchar9;
///OTG_FS host channel-9 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint9](otg_fs_hcint9) module
pub type OTG_FS_HCINT9 = crate::Reg<u32, _OTG_FS_HCINT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT9;
///`read()` method returns [otg_fs_hcint9::R](otg_fs_hcint9::R) reader structure
impl crate::Readable for OTG_FS_HCINT9 {}
///`write(|w| ..)` method takes [otg_fs_hcint9::W](otg_fs_hcint9::W) writer structure
impl crate::Writable for OTG_FS_HCINT9 {}
///OTG_FS host channel-9 interrupt register
pub mod otg_fs_hcint9;
///OTG_FS host channel-9 mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk9](otg_fs_hcintmsk9) module
pub type OTG_FS_HCINTMSK9 = crate::Reg<u32, _OTG_FS_HCINTMSK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK9;
///`read()` method returns [otg_fs_hcintmsk9::R](otg_fs_hcintmsk9::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK9 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk9::W](otg_fs_hcintmsk9::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK9 {}
///OTG_FS host channel-9 mask register
pub mod otg_fs_hcintmsk9;
///OTG_FS host channel-9 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz9](otg_fs_hctsiz9) module
pub type OTG_FS_HCTSIZ9 = crate::Reg<u32, _OTG_FS_HCTSIZ9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ9;
///`read()` method returns [otg_fs_hctsiz9::R](otg_fs_hctsiz9::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ9 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz9::W](otg_fs_hctsiz9::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ9 {}
///OTG_FS host channel-9 transfer size register
pub mod otg_fs_hctsiz9;
///OTG_FS host channel-10 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar10](otg_fs_hcchar10) module
pub type OTG_FS_HCCHAR10 = crate::Reg<u32, _OTG_FS_HCCHAR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR10;
///`read()` method returns [otg_fs_hcchar10::R](otg_fs_hcchar10::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR10 {}
///`write(|w| ..)` method takes [otg_fs_hcchar10::W](otg_fs_hcchar10::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR10 {}
///OTG_FS host channel-10 characteristics register
pub mod otg_fs_hcchar10;
///OTG_FS host channel-10 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint10](otg_fs_hcint10) module
pub type OTG_FS_HCINT10 = crate::Reg<u32, _OTG_FS_HCINT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT10;
///`read()` method returns [otg_fs_hcint10::R](otg_fs_hcint10::R) reader structure
impl crate::Readable for OTG_FS_HCINT10 {}
///`write(|w| ..)` method takes [otg_fs_hcint10::W](otg_fs_hcint10::W) writer structure
impl crate::Writable for OTG_FS_HCINT10 {}
///OTG_FS host channel-10 interrupt register
pub mod otg_fs_hcint10;
///OTG_FS host channel-10 mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk10](otg_fs_hcintmsk10) module
pub type OTG_FS_HCINTMSK10 = crate::Reg<u32, _OTG_FS_HCINTMSK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK10;
///`read()` method returns [otg_fs_hcintmsk10::R](otg_fs_hcintmsk10::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK10 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk10::W](otg_fs_hcintmsk10::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK10 {}
///OTG_FS host channel-10 mask register
pub mod otg_fs_hcintmsk10;
///OTG_FS host channel-10 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz10](otg_fs_hctsiz10) module
pub type OTG_FS_HCTSIZ10 = crate::Reg<u32, _OTG_FS_HCTSIZ10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ10;
///`read()` method returns [otg_fs_hctsiz10::R](otg_fs_hctsiz10::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ10 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz10::W](otg_fs_hctsiz10::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ10 {}
///OTG_FS host channel-10 transfer size register
pub mod otg_fs_hctsiz10;
///OTG_FS host channel-11 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcchar11](otg_fs_hcchar11) module
pub type OTG_FS_HCCHAR11 = crate::Reg<u32, _OTG_FS_HCCHAR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR11;
///`read()` method returns [otg_fs_hcchar11::R](otg_fs_hcchar11::R) reader structure
impl crate::Readable for OTG_FS_HCCHAR11 {}
///`write(|w| ..)` method takes [otg_fs_hcchar11::W](otg_fs_hcchar11::W) writer structure
impl crate::Writable for OTG_FS_HCCHAR11 {}
///OTG_FS host channel-11 characteristics register
pub mod otg_fs_hcchar11;
///OTG_FS host channel-11 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcint11](otg_fs_hcint11) module
pub type OTG_FS_HCINT11 = crate::Reg<u32, _OTG_FS_HCINT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT11;
///`read()` method returns [otg_fs_hcint11::R](otg_fs_hcint11::R) reader structure
impl crate::Readable for OTG_FS_HCINT11 {}
///`write(|w| ..)` method takes [otg_fs_hcint11::W](otg_fs_hcint11::W) writer structure
impl crate::Writable for OTG_FS_HCINT11 {}
///OTG_FS host channel-11 interrupt register
pub mod otg_fs_hcint11;
///OTG_FS host channel-11 mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hcintmsk11](otg_fs_hcintmsk11) module
pub type OTG_FS_HCINTMSK11 = crate::Reg<u32, _OTG_FS_HCINTMSK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK11;
///`read()` method returns [otg_fs_hcintmsk11::R](otg_fs_hcintmsk11::R) reader structure
impl crate::Readable for OTG_FS_HCINTMSK11 {}
///`write(|w| ..)` method takes [otg_fs_hcintmsk11::W](otg_fs_hcintmsk11::W) writer structure
impl crate::Writable for OTG_FS_HCINTMSK11 {}
///OTG_FS host channel-11 mask register
pub mod otg_fs_hcintmsk11;
///OTG_FS host channel-11 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_fs_hctsiz11](otg_fs_hctsiz11) module
pub type OTG_FS_HCTSIZ11 = crate::Reg<u32, _OTG_FS_HCTSIZ11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ11;
///`read()` method returns [otg_fs_hctsiz11::R](otg_fs_hctsiz11::R) reader structure
impl crate::Readable for OTG_FS_HCTSIZ11 {}
///`write(|w| ..)` method takes [otg_fs_hctsiz11::W](otg_fs_hctsiz11::W) writer structure
impl crate::Writable for OTG_FS_HCTSIZ11 {}
///OTG_FS host channel-11 transfer size register
pub mod otg_fs_hctsiz11;
