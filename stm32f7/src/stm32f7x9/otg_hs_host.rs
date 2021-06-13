///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_HS host configuration register
    pub otg_hs_hcfg: OTG_HS_HCFG,
    ///0x04 - OTG_HS Host frame interval register
    pub otg_hs_hfir: OTG_HS_HFIR,
    ///0x08 - OTG_HS host frame number/frame time remaining register
    pub otg_hs_hfnum: OTG_HS_HFNUM,
    _reserved3: [u8; 4usize],
    ///0x10 - OTG_HS_Host periodic transmit FIFO/queue status register
    pub otg_hs_hptxsts: OTG_HS_HPTXSTS,
    ///0x14 - OTG_HS Host all channels interrupt register
    pub otg_hs_haint: OTG_HS_HAINT,
    ///0x18 - OTG_HS host all channels interrupt mask register
    pub otg_hs_haintmsk: OTG_HS_HAINTMSK,
    _reserved6: [u8; 36usize],
    ///0x40 - OTG_HS host port control and status register
    pub otg_hs_hprt: OTG_HS_HPRT,
    _reserved7: [u8; 188usize],
    ///0x100 - OTG_HS host channel-0 characteristics register
    pub otg_hs_hcchar0: OTG_HS_HCCHAR0,
    ///0x104 - OTG_HS host channel-0 split control register
    pub otg_hs_hcsplt0: OTG_HS_HCSPLT0,
    ///0x108 - OTG_HS host channel-11 interrupt register
    pub otg_hs_hcint0: OTG_HS_HCINT0,
    ///0x10c - OTG_HS host channel-11 interrupt mask register
    pub otg_hs_hcintmsk0: OTG_HS_HCINTMSK0,
    ///0x110 - OTG_HS host channel-11 transfer size register
    pub otg_hs_hctsiz0: OTG_HS_HCTSIZ0,
    ///0x114 - OTG_HS host channel-0 DMA address register
    pub otg_hs_hcdma0: OTG_HS_HCDMA0,
    _reserved13: [u8; 8usize],
    ///0x120 - OTG_HS host channel-1 characteristics register
    pub otg_hs_hcchar1: OTG_HS_HCCHAR1,
    ///0x124 - OTG_HS host channel-1 split control register
    pub otg_hs_hcsplt1: OTG_HS_HCSPLT1,
    ///0x128 - OTG_HS host channel-1 interrupt register
    pub otg_hs_hcint1: OTG_HS_HCINT1,
    ///0x12c - OTG_HS host channel-1 interrupt mask register
    pub otg_hs_hcintmsk1: OTG_HS_HCINTMSK1,
    ///0x130 - OTG_HS host channel-1 transfer size register
    pub otg_hs_hctsiz1: OTG_HS_HCTSIZ1,
    ///0x134 - OTG_HS host channel-1 DMA address register
    pub otg_hs_hcdma1: OTG_HS_HCDMA1,
    _reserved19: [u8; 8usize],
    ///0x140 - OTG_HS host channel-2 characteristics register
    pub otg_hs_hcchar2: OTG_HS_HCCHAR2,
    ///0x144 - OTG_HS host channel-2 split control register
    pub otg_hs_hcsplt2: OTG_HS_HCSPLT2,
    ///0x148 - OTG_HS host channel-2 interrupt register
    pub otg_hs_hcint2: OTG_HS_HCINT2,
    ///0x14c - OTG_HS host channel-2 interrupt mask register
    pub otg_hs_hcintmsk2: OTG_HS_HCINTMSK2,
    ///0x150 - OTG_HS host channel-2 transfer size register
    pub otg_hs_hctsiz2: OTG_HS_HCTSIZ2,
    ///0x154 - OTG_HS host channel-2 DMA address register
    pub otg_hs_hcdma2: OTG_HS_HCDMA2,
    _reserved25: [u8; 8usize],
    ///0x160 - OTG_HS host channel-3 characteristics register
    pub otg_hs_hcchar3: OTG_HS_HCCHAR3,
    ///0x164 - OTG_HS host channel-3 split control register
    pub otg_hs_hcsplt3: OTG_HS_HCSPLT3,
    ///0x168 - OTG_HS host channel-3 interrupt register
    pub otg_hs_hcint3: OTG_HS_HCINT3,
    ///0x16c - OTG_HS host channel-3 interrupt mask register
    pub otg_hs_hcintmsk3: OTG_HS_HCINTMSK3,
    ///0x170 - OTG_HS host channel-3 transfer size register
    pub otg_hs_hctsiz3: OTG_HS_HCTSIZ3,
    ///0x174 - OTG_HS host channel-3 DMA address register
    pub otg_hs_hcdma3: OTG_HS_HCDMA3,
    _reserved31: [u8; 8usize],
    ///0x180 - OTG_HS host channel-4 characteristics register
    pub otg_hs_hcchar4: OTG_HS_HCCHAR4,
    ///0x184 - OTG_HS host channel-4 split control register
    pub otg_hs_hcsplt4: OTG_HS_HCSPLT4,
    ///0x188 - OTG_HS host channel-4 interrupt register
    pub otg_hs_hcint4: OTG_HS_HCINT4,
    ///0x18c - OTG_HS host channel-4 interrupt mask register
    pub otg_hs_hcintmsk4: OTG_HS_HCINTMSK4,
    ///0x190 - OTG_HS host channel-4 transfer size register
    pub otg_hs_hctsiz4: OTG_HS_HCTSIZ4,
    ///0x194 - OTG_HS host channel-4 DMA address register
    pub otg_hs_hcdma4: OTG_HS_HCDMA4,
    _reserved37: [u8; 8usize],
    ///0x1a0 - OTG_HS host channel-5 characteristics register
    pub otg_hs_hcchar5: OTG_HS_HCCHAR5,
    ///0x1a4 - OTG_HS host channel-5 split control register
    pub otg_hs_hcsplt5: OTG_HS_HCSPLT5,
    ///0x1a8 - OTG_HS host channel-5 interrupt register
    pub otg_hs_hcint5: OTG_HS_HCINT5,
    ///0x1ac - OTG_HS host channel-5 interrupt mask register
    pub otg_hs_hcintmsk5: OTG_HS_HCINTMSK5,
    ///0x1b0 - OTG_HS host channel-5 transfer size register
    pub otg_hs_hctsiz5: OTG_HS_HCTSIZ5,
    ///0x1b4 - OTG_HS host channel-5 DMA address register
    pub otg_hs_hcdma5: OTG_HS_HCDMA5,
    _reserved43: [u8; 8usize],
    ///0x1c0 - OTG_HS host channel-6 characteristics register
    pub otg_hs_hcchar6: OTG_HS_HCCHAR6,
    ///0x1c4 - OTG_HS host channel-6 split control register
    pub otg_hs_hcsplt6: OTG_HS_HCSPLT6,
    ///0x1c8 - OTG_HS host channel-6 interrupt register
    pub otg_hs_hcint6: OTG_HS_HCINT6,
    ///0x1cc - OTG_HS host channel-6 interrupt mask register
    pub otg_hs_hcintmsk6: OTG_HS_HCINTMSK6,
    ///0x1d0 - OTG_HS host channel-6 transfer size register
    pub otg_hs_hctsiz6: OTG_HS_HCTSIZ6,
    ///0x1d4 - OTG_HS host channel-6 DMA address register
    pub otg_hs_hcdma6: OTG_HS_HCDMA6,
    _reserved49: [u8; 8usize],
    ///0x1e0 - OTG_HS host channel-7 characteristics register
    pub otg_hs_hcchar7: OTG_HS_HCCHAR7,
    ///0x1e4 - OTG_HS host channel-7 split control register
    pub otg_hs_hcsplt7: OTG_HS_HCSPLT7,
    ///0x1e8 - OTG_HS host channel-7 interrupt register
    pub otg_hs_hcint7: OTG_HS_HCINT7,
    ///0x1ec - OTG_HS host channel-7 interrupt mask register
    pub otg_hs_hcintmsk7: OTG_HS_HCINTMSK7,
    ///0x1f0 - OTG_HS host channel-7 transfer size register
    pub otg_hs_hctsiz7: OTG_HS_HCTSIZ7,
    ///0x1f4 - OTG_HS host channel-7 DMA address register
    pub otg_hs_hcdma7: OTG_HS_HCDMA7,
    _reserved55: [u8; 8usize],
    ///0x200 - OTG_HS host channel-8 characteristics register
    pub otg_hs_hcchar8: OTG_HS_HCCHAR8,
    ///0x204 - OTG_HS host channel-8 split control register
    pub otg_hs_hcsplt8: OTG_HS_HCSPLT8,
    ///0x208 - OTG_HS host channel-8 interrupt register
    pub otg_hs_hcint8: OTG_HS_HCINT8,
    ///0x20c - OTG_HS host channel-8 interrupt mask register
    pub otg_hs_hcintmsk8: OTG_HS_HCINTMSK8,
    ///0x210 - OTG_HS host channel-8 transfer size register
    pub otg_hs_hctsiz8: OTG_HS_HCTSIZ8,
    ///0x214 - OTG_HS host channel-8 DMA address register
    pub otg_hs_hcdma8: OTG_HS_HCDMA8,
    _reserved61: [u8; 8usize],
    ///0x220 - OTG_HS host channel-9 characteristics register
    pub otg_hs_hcchar9: OTG_HS_HCCHAR9,
    ///0x224 - OTG_HS host channel-9 split control register
    pub otg_hs_hcsplt9: OTG_HS_HCSPLT9,
    ///0x228 - OTG_HS host channel-9 interrupt register
    pub otg_hs_hcint9: OTG_HS_HCINT9,
    ///0x22c - OTG_HS host channel-9 interrupt mask register
    pub otg_hs_hcintmsk9: OTG_HS_HCINTMSK9,
    ///0x230 - OTG_HS host channel-9 transfer size register
    pub otg_hs_hctsiz9: OTG_HS_HCTSIZ9,
    ///0x234 - OTG_HS host channel-9 DMA address register
    pub otg_hs_hcdma9: OTG_HS_HCDMA9,
    _reserved67: [u8; 8usize],
    ///0x240 - OTG_HS host channel-10 characteristics register
    pub otg_hs_hcchar10: OTG_HS_HCCHAR10,
    ///0x244 - OTG_HS host channel-10 split control register
    pub otg_hs_hcsplt10: OTG_HS_HCSPLT10,
    ///0x248 - OTG_HS host channel-10 interrupt register
    pub otg_hs_hcint10: OTG_HS_HCINT10,
    ///0x24c - OTG_HS host channel-10 interrupt mask register
    pub otg_hs_hcintmsk10: OTG_HS_HCINTMSK10,
    ///0x250 - OTG_HS host channel-10 transfer size register
    pub otg_hs_hctsiz10: OTG_HS_HCTSIZ10,
    ///0x254 - OTG_HS host channel-10 DMA address register
    pub otg_hs_hcdma10: OTG_HS_HCDMA10,
    _reserved73: [u8; 8usize],
    ///0x260 - OTG_HS host channel-11 characteristics register
    pub otg_hs_hcchar11: OTG_HS_HCCHAR11,
    ///0x264 - OTG_HS host channel-11 split control register
    pub otg_hs_hcsplt11: OTG_HS_HCSPLT11,
    ///0x268 - OTG_HS host channel-11 interrupt register
    pub otg_hs_hcint11: OTG_HS_HCINT11,
    ///0x26c - OTG_HS host channel-11 interrupt mask register
    pub otg_hs_hcintmsk11: OTG_HS_HCINTMSK11,
    ///0x270 - OTG_HS host channel-11 transfer size register
    pub otg_hs_hctsiz11: OTG_HS_HCTSIZ11,
    ///0x274 - OTG_HS host channel-11 DMA address register
    pub otg_hs_hcdma11: OTG_HS_HCDMA11,
    ///0x278 - OTG_HS host channel-12 characteristics register
    pub otg_hs_hcchar12: OTG_HS_HCCHAR12,
    ///0x27c - OTG_HS host channel-12 split control register
    pub otg_hs_hcsplt12: OTG_HS_HCSPLT12,
    ///0x280 - OTG_HS host channel-12 interrupt register
    pub otg_hs_hcint12: OTG_HS_HCINT12,
    ///0x284 - OTG_HS host channel-12 interrupt mask register
    pub otg_hs_hcintmsk12: OTG_HS_HCINTMSK12,
    ///0x288 - OTG_HS host channel-12 transfer size register
    pub otg_hs_hctsiz12: OTG_HS_HCTSIZ12,
    ///0x28c - OTG_HS host channel-12 DMA address register
    pub otg_hs_hcdma12: OTG_HS_HCDMA12,
    ///0x290 - OTG_HS host channel-13 characteristics register
    pub otg_hs_hcchar13: OTG_HS_HCCHAR13,
    ///0x294 - OTG_HS host channel-13 split control register
    pub otg_hs_hcsplt13: OTG_HS_HCSPLT13,
    ///0x298 - OTG_HS host channel-13 interrupt register
    pub otg_hs_hcint13: OTG_HS_HCINT13,
    ///0x29c - OTG_HS host channel-13 interrupt mask register
    pub otg_hs_hcintmsk13: OTG_HS_HCINTMSK13,
    ///0x2a0 - OTG_HS host channel-13 transfer size register
    pub otg_hs_hctsiz13: OTG_HS_HCTSIZ13,
    ///0x2a4 - OTG_HS host channel-13 DMA address register
    pub otg_hs_hcdma13: OTG_HS_HCDMA13,
    ///0x2a8 - OTG_HS host channel-14 characteristics register
    pub otg_hs_hcchar14: OTG_HS_HCCHAR14,
    ///0x2ac - OTG_HS host channel-14 split control register
    pub otg_hs_hcsplt14: OTG_HS_HCSPLT14,
    ///0x2b0 - OTG_HS host channel-14 interrupt register
    pub otg_hs_hcint14: OTG_HS_HCINT14,
    ///0x2b4 - OTG_HS host channel-14 interrupt mask register
    pub otg_hs_hcintmsk14: OTG_HS_HCINTMSK14,
    ///0x2b8 - OTG_HS host channel-14 transfer size register
    pub otg_hs_hctsiz14: OTG_HS_HCTSIZ14,
    ///0x2bc - OTG_HS host channel-14 DMA address register
    pub otg_hs_hcdma14: OTG_HS_HCDMA14,
    ///0x2c0 - OTG_HS host channel-15 characteristics register
    pub otg_hs_hcchar15: OTG_HS_HCCHAR15,
    ///0x2c4 - OTG_HS host channel-15 split control register
    pub otg_hs_hcsplt15: OTG_HS_HCSPLT15,
    ///0x2c8 - OTG_HS host channel-15 interrupt register
    pub otg_hs_hcint15: OTG_HS_HCINT15,
    ///0x2cc - OTG_HS host channel-15 interrupt mask register
    pub otg_hs_hcintmsk15: OTG_HS_HCINTMSK15,
    ///0x2d0 - OTG_HS host channel-15 transfer size register
    pub otg_hs_hctsiz15: OTG_HS_HCTSIZ15,
    ///0x2d4 - OTG_HS host channel-15 DMA address register
    pub otg_hs_hcdma15: OTG_HS_HCDMA15,
}
///OTG_HS host configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcfg](otg_hs_hcfg) module
pub type OTG_HS_HCFG = crate::Reg<u32, _OTG_HS_HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCFG;
///`read()` method returns [otg_hs_hcfg::R](otg_hs_hcfg::R) reader structure
impl crate::Readable for OTG_HS_HCFG {}
///`write(|w| ..)` method takes [otg_hs_hcfg::W](otg_hs_hcfg::W) writer structure
impl crate::Writable for OTG_HS_HCFG {}
///OTG_HS host configuration register
pub mod otg_hs_hcfg;
///OTG_HS Host frame interval register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hfir](otg_hs_hfir) module
pub type OTG_HS_HFIR = crate::Reg<u32, _OTG_HS_HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HFIR;
///`read()` method returns [otg_hs_hfir::R](otg_hs_hfir::R) reader structure
impl crate::Readable for OTG_HS_HFIR {}
///`write(|w| ..)` method takes [otg_hs_hfir::W](otg_hs_hfir::W) writer structure
impl crate::Writable for OTG_HS_HFIR {}
///OTG_HS Host frame interval register
pub mod otg_hs_hfir;
///OTG_HS host frame number/frame time remaining register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hfnum](otg_hs_hfnum) module
pub type OTG_HS_HFNUM = crate::Reg<u32, _OTG_HS_HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HFNUM;
///`read()` method returns [otg_hs_hfnum::R](otg_hs_hfnum::R) reader structure
impl crate::Readable for OTG_HS_HFNUM {}
///OTG_HS host frame number/frame time remaining register
pub mod otg_hs_hfnum;
///OTG_HS_Host periodic transmit FIFO/queue status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hptxsts](otg_hs_hptxsts) module
pub type OTG_HS_HPTXSTS = crate::Reg<u32, _OTG_HS_HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HPTXSTS;
///`read()` method returns [otg_hs_hptxsts::R](otg_hs_hptxsts::R) reader structure
impl crate::Readable for OTG_HS_HPTXSTS {}
///`write(|w| ..)` method takes [otg_hs_hptxsts::W](otg_hs_hptxsts::W) writer structure
impl crate::Writable for OTG_HS_HPTXSTS {}
///OTG_HS_Host periodic transmit FIFO/queue status register
pub mod otg_hs_hptxsts;
///OTG_HS Host all channels interrupt register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_haint](otg_hs_haint) module
pub type OTG_HS_HAINT = crate::Reg<u32, _OTG_HS_HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HAINT;
///`read()` method returns [otg_hs_haint::R](otg_hs_haint::R) reader structure
impl crate::Readable for OTG_HS_HAINT {}
///OTG_HS Host all channels interrupt register
pub mod otg_hs_haint;
///OTG_HS host all channels interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_haintmsk](otg_hs_haintmsk) module
pub type OTG_HS_HAINTMSK = crate::Reg<u32, _OTG_HS_HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HAINTMSK;
///`read()` method returns [otg_hs_haintmsk::R](otg_hs_haintmsk::R) reader structure
impl crate::Readable for OTG_HS_HAINTMSK {}
///`write(|w| ..)` method takes [otg_hs_haintmsk::W](otg_hs_haintmsk::W) writer structure
impl crate::Writable for OTG_HS_HAINTMSK {}
///OTG_HS host all channels interrupt mask register
pub mod otg_hs_haintmsk;
///OTG_HS host port control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hprt](otg_hs_hprt) module
pub type OTG_HS_HPRT = crate::Reg<u32, _OTG_HS_HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HPRT;
///`read()` method returns [otg_hs_hprt::R](otg_hs_hprt::R) reader structure
impl crate::Readable for OTG_HS_HPRT {}
///`write(|w| ..)` method takes [otg_hs_hprt::W](otg_hs_hprt::W) writer structure
impl crate::Writable for OTG_HS_HPRT {}
///OTG_HS host port control and status register
pub mod otg_hs_hprt;
///OTG_HS host channel-0 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar0](otg_hs_hcchar0) module
pub type OTG_HS_HCCHAR0 = crate::Reg<u32, _OTG_HS_HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR0;
///`read()` method returns [otg_hs_hcchar0::R](otg_hs_hcchar0::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR0 {}
///`write(|w| ..)` method takes [otg_hs_hcchar0::W](otg_hs_hcchar0::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR0 {}
///OTG_HS host channel-0 characteristics register
pub mod otg_hs_hcchar0;
///OTG_HS host channel-1 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar1](otg_hs_hcchar1) module
pub type OTG_HS_HCCHAR1 = crate::Reg<u32, _OTG_HS_HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR1;
///`read()` method returns [otg_hs_hcchar1::R](otg_hs_hcchar1::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR1 {}
///`write(|w| ..)` method takes [otg_hs_hcchar1::W](otg_hs_hcchar1::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR1 {}
///OTG_HS host channel-1 characteristics register
pub mod otg_hs_hcchar1;
///OTG_HS host channel-2 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar2](otg_hs_hcchar2) module
pub type OTG_HS_HCCHAR2 = crate::Reg<u32, _OTG_HS_HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR2;
///`read()` method returns [otg_hs_hcchar2::R](otg_hs_hcchar2::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR2 {}
///`write(|w| ..)` method takes [otg_hs_hcchar2::W](otg_hs_hcchar2::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR2 {}
///OTG_HS host channel-2 characteristics register
pub mod otg_hs_hcchar2;
///OTG_HS host channel-3 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar3](otg_hs_hcchar3) module
pub type OTG_HS_HCCHAR3 = crate::Reg<u32, _OTG_HS_HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR3;
///`read()` method returns [otg_hs_hcchar3::R](otg_hs_hcchar3::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR3 {}
///`write(|w| ..)` method takes [otg_hs_hcchar3::W](otg_hs_hcchar3::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR3 {}
///OTG_HS host channel-3 characteristics register
pub mod otg_hs_hcchar3;
///OTG_HS host channel-4 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar4](otg_hs_hcchar4) module
pub type OTG_HS_HCCHAR4 = crate::Reg<u32, _OTG_HS_HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR4;
///`read()` method returns [otg_hs_hcchar4::R](otg_hs_hcchar4::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR4 {}
///`write(|w| ..)` method takes [otg_hs_hcchar4::W](otg_hs_hcchar4::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR4 {}
///OTG_HS host channel-4 characteristics register
pub mod otg_hs_hcchar4;
///OTG_HS host channel-5 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar5](otg_hs_hcchar5) module
pub type OTG_HS_HCCHAR5 = crate::Reg<u32, _OTG_HS_HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR5;
///`read()` method returns [otg_hs_hcchar5::R](otg_hs_hcchar5::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR5 {}
///`write(|w| ..)` method takes [otg_hs_hcchar5::W](otg_hs_hcchar5::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR5 {}
///OTG_HS host channel-5 characteristics register
pub mod otg_hs_hcchar5;
///OTG_HS host channel-6 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar6](otg_hs_hcchar6) module
pub type OTG_HS_HCCHAR6 = crate::Reg<u32, _OTG_HS_HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR6;
///`read()` method returns [otg_hs_hcchar6::R](otg_hs_hcchar6::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR6 {}
///`write(|w| ..)` method takes [otg_hs_hcchar6::W](otg_hs_hcchar6::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR6 {}
///OTG_HS host channel-6 characteristics register
pub mod otg_hs_hcchar6;
///OTG_HS host channel-7 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar7](otg_hs_hcchar7) module
pub type OTG_HS_HCCHAR7 = crate::Reg<u32, _OTG_HS_HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR7;
///`read()` method returns [otg_hs_hcchar7::R](otg_hs_hcchar7::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR7 {}
///`write(|w| ..)` method takes [otg_hs_hcchar7::W](otg_hs_hcchar7::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR7 {}
///OTG_HS host channel-7 characteristics register
pub mod otg_hs_hcchar7;
///OTG_HS host channel-8 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar8](otg_hs_hcchar8) module
pub type OTG_HS_HCCHAR8 = crate::Reg<u32, _OTG_HS_HCCHAR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR8;
///`read()` method returns [otg_hs_hcchar8::R](otg_hs_hcchar8::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR8 {}
///`write(|w| ..)` method takes [otg_hs_hcchar8::W](otg_hs_hcchar8::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR8 {}
///OTG_HS host channel-8 characteristics register
pub mod otg_hs_hcchar8;
///OTG_HS host channel-9 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar9](otg_hs_hcchar9) module
pub type OTG_HS_HCCHAR9 = crate::Reg<u32, _OTG_HS_HCCHAR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR9;
///`read()` method returns [otg_hs_hcchar9::R](otg_hs_hcchar9::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR9 {}
///`write(|w| ..)` method takes [otg_hs_hcchar9::W](otg_hs_hcchar9::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR9 {}
///OTG_HS host channel-9 characteristics register
pub mod otg_hs_hcchar9;
///OTG_HS host channel-10 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar10](otg_hs_hcchar10) module
pub type OTG_HS_HCCHAR10 = crate::Reg<u32, _OTG_HS_HCCHAR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR10;
///`read()` method returns [otg_hs_hcchar10::R](otg_hs_hcchar10::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR10 {}
///`write(|w| ..)` method takes [otg_hs_hcchar10::W](otg_hs_hcchar10::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR10 {}
///OTG_HS host channel-10 characteristics register
pub mod otg_hs_hcchar10;
///OTG_HS host channel-11 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar11](otg_hs_hcchar11) module
pub type OTG_HS_HCCHAR11 = crate::Reg<u32, _OTG_HS_HCCHAR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR11;
///`read()` method returns [otg_hs_hcchar11::R](otg_hs_hcchar11::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR11 {}
///`write(|w| ..)` method takes [otg_hs_hcchar11::W](otg_hs_hcchar11::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR11 {}
///OTG_HS host channel-11 characteristics register
pub mod otg_hs_hcchar11;
///OTG_HS host channel-0 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt0](otg_hs_hcsplt0) module
pub type OTG_HS_HCSPLT0 = crate::Reg<u32, _OTG_HS_HCSPLT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT0;
///`read()` method returns [otg_hs_hcsplt0::R](otg_hs_hcsplt0::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT0 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt0::W](otg_hs_hcsplt0::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT0 {}
///OTG_HS host channel-0 split control register
pub mod otg_hs_hcsplt0;
///OTG_HS host channel-1 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt1](otg_hs_hcsplt1) module
pub type OTG_HS_HCSPLT1 = crate::Reg<u32, _OTG_HS_HCSPLT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT1;
///`read()` method returns [otg_hs_hcsplt1::R](otg_hs_hcsplt1::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT1 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt1::W](otg_hs_hcsplt1::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT1 {}
///OTG_HS host channel-1 split control register
pub mod otg_hs_hcsplt1;
///OTG_HS host channel-2 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt2](otg_hs_hcsplt2) module
pub type OTG_HS_HCSPLT2 = crate::Reg<u32, _OTG_HS_HCSPLT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT2;
///`read()` method returns [otg_hs_hcsplt2::R](otg_hs_hcsplt2::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT2 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt2::W](otg_hs_hcsplt2::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT2 {}
///OTG_HS host channel-2 split control register
pub mod otg_hs_hcsplt2;
///OTG_HS host channel-3 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt3](otg_hs_hcsplt3) module
pub type OTG_HS_HCSPLT3 = crate::Reg<u32, _OTG_HS_HCSPLT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT3;
///`read()` method returns [otg_hs_hcsplt3::R](otg_hs_hcsplt3::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT3 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt3::W](otg_hs_hcsplt3::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT3 {}
///OTG_HS host channel-3 split control register
pub mod otg_hs_hcsplt3;
///OTG_HS host channel-4 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt4](otg_hs_hcsplt4) module
pub type OTG_HS_HCSPLT4 = crate::Reg<u32, _OTG_HS_HCSPLT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT4;
///`read()` method returns [otg_hs_hcsplt4::R](otg_hs_hcsplt4::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT4 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt4::W](otg_hs_hcsplt4::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT4 {}
///OTG_HS host channel-4 split control register
pub mod otg_hs_hcsplt4;
///OTG_HS host channel-5 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt5](otg_hs_hcsplt5) module
pub type OTG_HS_HCSPLT5 = crate::Reg<u32, _OTG_HS_HCSPLT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT5;
///`read()` method returns [otg_hs_hcsplt5::R](otg_hs_hcsplt5::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT5 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt5::W](otg_hs_hcsplt5::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT5 {}
///OTG_HS host channel-5 split control register
pub mod otg_hs_hcsplt5;
///OTG_HS host channel-6 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt6](otg_hs_hcsplt6) module
pub type OTG_HS_HCSPLT6 = crate::Reg<u32, _OTG_HS_HCSPLT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT6;
///`read()` method returns [otg_hs_hcsplt6::R](otg_hs_hcsplt6::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT6 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt6::W](otg_hs_hcsplt6::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT6 {}
///OTG_HS host channel-6 split control register
pub mod otg_hs_hcsplt6;
///OTG_HS host channel-7 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt7](otg_hs_hcsplt7) module
pub type OTG_HS_HCSPLT7 = crate::Reg<u32, _OTG_HS_HCSPLT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT7;
///`read()` method returns [otg_hs_hcsplt7::R](otg_hs_hcsplt7::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT7 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt7::W](otg_hs_hcsplt7::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT7 {}
///OTG_HS host channel-7 split control register
pub mod otg_hs_hcsplt7;
///OTG_HS host channel-8 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt8](otg_hs_hcsplt8) module
pub type OTG_HS_HCSPLT8 = crate::Reg<u32, _OTG_HS_HCSPLT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT8;
///`read()` method returns [otg_hs_hcsplt8::R](otg_hs_hcsplt8::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT8 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt8::W](otg_hs_hcsplt8::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT8 {}
///OTG_HS host channel-8 split control register
pub mod otg_hs_hcsplt8;
///OTG_HS host channel-9 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt9](otg_hs_hcsplt9) module
pub type OTG_HS_HCSPLT9 = crate::Reg<u32, _OTG_HS_HCSPLT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT9;
///`read()` method returns [otg_hs_hcsplt9::R](otg_hs_hcsplt9::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT9 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt9::W](otg_hs_hcsplt9::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT9 {}
///OTG_HS host channel-9 split control register
pub mod otg_hs_hcsplt9;
///OTG_HS host channel-10 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt10](otg_hs_hcsplt10) module
pub type OTG_HS_HCSPLT10 = crate::Reg<u32, _OTG_HS_HCSPLT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT10;
///`read()` method returns [otg_hs_hcsplt10::R](otg_hs_hcsplt10::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT10 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt10::W](otg_hs_hcsplt10::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT10 {}
///OTG_HS host channel-10 split control register
pub mod otg_hs_hcsplt10;
///OTG_HS host channel-11 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt11](otg_hs_hcsplt11) module
pub type OTG_HS_HCSPLT11 = crate::Reg<u32, _OTG_HS_HCSPLT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT11;
///`read()` method returns [otg_hs_hcsplt11::R](otg_hs_hcsplt11::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT11 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt11::W](otg_hs_hcsplt11::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT11 {}
///OTG_HS host channel-11 split control register
pub mod otg_hs_hcsplt11;
///OTG_HS host channel-11 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint0](otg_hs_hcint0) module
pub type OTG_HS_HCINT0 = crate::Reg<u32, _OTG_HS_HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT0;
///`read()` method returns [otg_hs_hcint0::R](otg_hs_hcint0::R) reader structure
impl crate::Readable for OTG_HS_HCINT0 {}
///`write(|w| ..)` method takes [otg_hs_hcint0::W](otg_hs_hcint0::W) writer structure
impl crate::Writable for OTG_HS_HCINT0 {}
///OTG_HS host channel-11 interrupt register
pub mod otg_hs_hcint0;
///OTG_HS host channel-1 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint1](otg_hs_hcint1) module
pub type OTG_HS_HCINT1 = crate::Reg<u32, _OTG_HS_HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT1;
///`read()` method returns [otg_hs_hcint1::R](otg_hs_hcint1::R) reader structure
impl crate::Readable for OTG_HS_HCINT1 {}
///`write(|w| ..)` method takes [otg_hs_hcint1::W](otg_hs_hcint1::W) writer structure
impl crate::Writable for OTG_HS_HCINT1 {}
///OTG_HS host channel-1 interrupt register
pub mod otg_hs_hcint1;
///OTG_HS host channel-2 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint2](otg_hs_hcint2) module
pub type OTG_HS_HCINT2 = crate::Reg<u32, _OTG_HS_HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT2;
///`read()` method returns [otg_hs_hcint2::R](otg_hs_hcint2::R) reader structure
impl crate::Readable for OTG_HS_HCINT2 {}
///`write(|w| ..)` method takes [otg_hs_hcint2::W](otg_hs_hcint2::W) writer structure
impl crate::Writable for OTG_HS_HCINT2 {}
///OTG_HS host channel-2 interrupt register
pub mod otg_hs_hcint2;
///OTG_HS host channel-3 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint3](otg_hs_hcint3) module
pub type OTG_HS_HCINT3 = crate::Reg<u32, _OTG_HS_HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT3;
///`read()` method returns [otg_hs_hcint3::R](otg_hs_hcint3::R) reader structure
impl crate::Readable for OTG_HS_HCINT3 {}
///`write(|w| ..)` method takes [otg_hs_hcint3::W](otg_hs_hcint3::W) writer structure
impl crate::Writable for OTG_HS_HCINT3 {}
///OTG_HS host channel-3 interrupt register
pub mod otg_hs_hcint3;
///OTG_HS host channel-4 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint4](otg_hs_hcint4) module
pub type OTG_HS_HCINT4 = crate::Reg<u32, _OTG_HS_HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT4;
///`read()` method returns [otg_hs_hcint4::R](otg_hs_hcint4::R) reader structure
impl crate::Readable for OTG_HS_HCINT4 {}
///`write(|w| ..)` method takes [otg_hs_hcint4::W](otg_hs_hcint4::W) writer structure
impl crate::Writable for OTG_HS_HCINT4 {}
///OTG_HS host channel-4 interrupt register
pub mod otg_hs_hcint4;
///OTG_HS host channel-5 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint5](otg_hs_hcint5) module
pub type OTG_HS_HCINT5 = crate::Reg<u32, _OTG_HS_HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT5;
///`read()` method returns [otg_hs_hcint5::R](otg_hs_hcint5::R) reader structure
impl crate::Readable for OTG_HS_HCINT5 {}
///`write(|w| ..)` method takes [otg_hs_hcint5::W](otg_hs_hcint5::W) writer structure
impl crate::Writable for OTG_HS_HCINT5 {}
///OTG_HS host channel-5 interrupt register
pub mod otg_hs_hcint5;
///OTG_HS host channel-6 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint6](otg_hs_hcint6) module
pub type OTG_HS_HCINT6 = crate::Reg<u32, _OTG_HS_HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT6;
///`read()` method returns [otg_hs_hcint6::R](otg_hs_hcint6::R) reader structure
impl crate::Readable for OTG_HS_HCINT6 {}
///`write(|w| ..)` method takes [otg_hs_hcint6::W](otg_hs_hcint6::W) writer structure
impl crate::Writable for OTG_HS_HCINT6 {}
///OTG_HS host channel-6 interrupt register
pub mod otg_hs_hcint6;
///OTG_HS host channel-7 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint7](otg_hs_hcint7) module
pub type OTG_HS_HCINT7 = crate::Reg<u32, _OTG_HS_HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT7;
///`read()` method returns [otg_hs_hcint7::R](otg_hs_hcint7::R) reader structure
impl crate::Readable for OTG_HS_HCINT7 {}
///`write(|w| ..)` method takes [otg_hs_hcint7::W](otg_hs_hcint7::W) writer structure
impl crate::Writable for OTG_HS_HCINT7 {}
///OTG_HS host channel-7 interrupt register
pub mod otg_hs_hcint7;
///OTG_HS host channel-8 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint8](otg_hs_hcint8) module
pub type OTG_HS_HCINT8 = crate::Reg<u32, _OTG_HS_HCINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT8;
///`read()` method returns [otg_hs_hcint8::R](otg_hs_hcint8::R) reader structure
impl crate::Readable for OTG_HS_HCINT8 {}
///`write(|w| ..)` method takes [otg_hs_hcint8::W](otg_hs_hcint8::W) writer structure
impl crate::Writable for OTG_HS_HCINT8 {}
///OTG_HS host channel-8 interrupt register
pub mod otg_hs_hcint8;
///OTG_HS host channel-9 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint9](otg_hs_hcint9) module
pub type OTG_HS_HCINT9 = crate::Reg<u32, _OTG_HS_HCINT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT9;
///`read()` method returns [otg_hs_hcint9::R](otg_hs_hcint9::R) reader structure
impl crate::Readable for OTG_HS_HCINT9 {}
///`write(|w| ..)` method takes [otg_hs_hcint9::W](otg_hs_hcint9::W) writer structure
impl crate::Writable for OTG_HS_HCINT9 {}
///OTG_HS host channel-9 interrupt register
pub mod otg_hs_hcint9;
///OTG_HS host channel-10 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint10](otg_hs_hcint10) module
pub type OTG_HS_HCINT10 = crate::Reg<u32, _OTG_HS_HCINT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT10;
///`read()` method returns [otg_hs_hcint10::R](otg_hs_hcint10::R) reader structure
impl crate::Readable for OTG_HS_HCINT10 {}
///`write(|w| ..)` method takes [otg_hs_hcint10::W](otg_hs_hcint10::W) writer structure
impl crate::Writable for OTG_HS_HCINT10 {}
///OTG_HS host channel-10 interrupt register
pub mod otg_hs_hcint10;
///OTG_HS host channel-11 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint11](otg_hs_hcint11) module
pub type OTG_HS_HCINT11 = crate::Reg<u32, _OTG_HS_HCINT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT11;
///`read()` method returns [otg_hs_hcint11::R](otg_hs_hcint11::R) reader structure
impl crate::Readable for OTG_HS_HCINT11 {}
///`write(|w| ..)` method takes [otg_hs_hcint11::W](otg_hs_hcint11::W) writer structure
impl crate::Writable for OTG_HS_HCINT11 {}
///OTG_HS host channel-11 interrupt register
pub mod otg_hs_hcint11;
///OTG_HS host channel-11 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk0](otg_hs_hcintmsk0) module
pub type OTG_HS_HCINTMSK0 = crate::Reg<u32, _OTG_HS_HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK0;
///`read()` method returns [otg_hs_hcintmsk0::R](otg_hs_hcintmsk0::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK0 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk0::W](otg_hs_hcintmsk0::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK0 {}
///OTG_HS host channel-11 interrupt mask register
pub mod otg_hs_hcintmsk0;
///OTG_HS host channel-1 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk1](otg_hs_hcintmsk1) module
pub type OTG_HS_HCINTMSK1 = crate::Reg<u32, _OTG_HS_HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK1;
///`read()` method returns [otg_hs_hcintmsk1::R](otg_hs_hcintmsk1::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK1 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk1::W](otg_hs_hcintmsk1::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK1 {}
///OTG_HS host channel-1 interrupt mask register
pub mod otg_hs_hcintmsk1;
///OTG_HS host channel-2 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk2](otg_hs_hcintmsk2) module
pub type OTG_HS_HCINTMSK2 = crate::Reg<u32, _OTG_HS_HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK2;
///`read()` method returns [otg_hs_hcintmsk2::R](otg_hs_hcintmsk2::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK2 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk2::W](otg_hs_hcintmsk2::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK2 {}
///OTG_HS host channel-2 interrupt mask register
pub mod otg_hs_hcintmsk2;
///OTG_HS host channel-3 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk3](otg_hs_hcintmsk3) module
pub type OTG_HS_HCINTMSK3 = crate::Reg<u32, _OTG_HS_HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK3;
///`read()` method returns [otg_hs_hcintmsk3::R](otg_hs_hcintmsk3::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK3 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk3::W](otg_hs_hcintmsk3::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK3 {}
///OTG_HS host channel-3 interrupt mask register
pub mod otg_hs_hcintmsk3;
///OTG_HS host channel-4 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk4](otg_hs_hcintmsk4) module
pub type OTG_HS_HCINTMSK4 = crate::Reg<u32, _OTG_HS_HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK4;
///`read()` method returns [otg_hs_hcintmsk4::R](otg_hs_hcintmsk4::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK4 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk4::W](otg_hs_hcintmsk4::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK4 {}
///OTG_HS host channel-4 interrupt mask register
pub mod otg_hs_hcintmsk4;
///OTG_HS host channel-5 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk5](otg_hs_hcintmsk5) module
pub type OTG_HS_HCINTMSK5 = crate::Reg<u32, _OTG_HS_HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK5;
///`read()` method returns [otg_hs_hcintmsk5::R](otg_hs_hcintmsk5::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK5 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk5::W](otg_hs_hcintmsk5::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK5 {}
///OTG_HS host channel-5 interrupt mask register
pub mod otg_hs_hcintmsk5;
///OTG_HS host channel-6 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk6](otg_hs_hcintmsk6) module
pub type OTG_HS_HCINTMSK6 = crate::Reg<u32, _OTG_HS_HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK6;
///`read()` method returns [otg_hs_hcintmsk6::R](otg_hs_hcintmsk6::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK6 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk6::W](otg_hs_hcintmsk6::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK6 {}
///OTG_HS host channel-6 interrupt mask register
pub mod otg_hs_hcintmsk6;
///OTG_HS host channel-7 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk7](otg_hs_hcintmsk7) module
pub type OTG_HS_HCINTMSK7 = crate::Reg<u32, _OTG_HS_HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK7;
///`read()` method returns [otg_hs_hcintmsk7::R](otg_hs_hcintmsk7::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK7 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk7::W](otg_hs_hcintmsk7::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK7 {}
///OTG_HS host channel-7 interrupt mask register
pub mod otg_hs_hcintmsk7;
///OTG_HS host channel-8 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk8](otg_hs_hcintmsk8) module
pub type OTG_HS_HCINTMSK8 = crate::Reg<u32, _OTG_HS_HCINTMSK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK8;
///`read()` method returns [otg_hs_hcintmsk8::R](otg_hs_hcintmsk8::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK8 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk8::W](otg_hs_hcintmsk8::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK8 {}
///OTG_HS host channel-8 interrupt mask register
pub mod otg_hs_hcintmsk8;
///OTG_HS host channel-9 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk9](otg_hs_hcintmsk9) module
pub type OTG_HS_HCINTMSK9 = crate::Reg<u32, _OTG_HS_HCINTMSK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK9;
///`read()` method returns [otg_hs_hcintmsk9::R](otg_hs_hcintmsk9::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK9 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk9::W](otg_hs_hcintmsk9::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK9 {}
///OTG_HS host channel-9 interrupt mask register
pub mod otg_hs_hcintmsk9;
///OTG_HS host channel-10 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk10](otg_hs_hcintmsk10) module
pub type OTG_HS_HCINTMSK10 = crate::Reg<u32, _OTG_HS_HCINTMSK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK10;
///`read()` method returns [otg_hs_hcintmsk10::R](otg_hs_hcintmsk10::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK10 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk10::W](otg_hs_hcintmsk10::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK10 {}
///OTG_HS host channel-10 interrupt mask register
pub mod otg_hs_hcintmsk10;
///OTG_HS host channel-11 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk11](otg_hs_hcintmsk11) module
pub type OTG_HS_HCINTMSK11 = crate::Reg<u32, _OTG_HS_HCINTMSK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK11;
///`read()` method returns [otg_hs_hcintmsk11::R](otg_hs_hcintmsk11::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK11 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk11::W](otg_hs_hcintmsk11::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK11 {}
///OTG_HS host channel-11 interrupt mask register
pub mod otg_hs_hcintmsk11;
///OTG_HS host channel-11 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz0](otg_hs_hctsiz0) module
pub type OTG_HS_HCTSIZ0 = crate::Reg<u32, _OTG_HS_HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ0;
///`read()` method returns [otg_hs_hctsiz0::R](otg_hs_hctsiz0::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ0 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz0::W](otg_hs_hctsiz0::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ0 {}
///OTG_HS host channel-11 transfer size register
pub mod otg_hs_hctsiz0;
///OTG_HS host channel-1 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz1](otg_hs_hctsiz1) module
pub type OTG_HS_HCTSIZ1 = crate::Reg<u32, _OTG_HS_HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ1;
///`read()` method returns [otg_hs_hctsiz1::R](otg_hs_hctsiz1::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ1 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz1::W](otg_hs_hctsiz1::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ1 {}
///OTG_HS host channel-1 transfer size register
pub mod otg_hs_hctsiz1;
///OTG_HS host channel-2 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz2](otg_hs_hctsiz2) module
pub type OTG_HS_HCTSIZ2 = crate::Reg<u32, _OTG_HS_HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ2;
///`read()` method returns [otg_hs_hctsiz2::R](otg_hs_hctsiz2::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ2 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz2::W](otg_hs_hctsiz2::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ2 {}
///OTG_HS host channel-2 transfer size register
pub mod otg_hs_hctsiz2;
///OTG_HS host channel-3 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz3](otg_hs_hctsiz3) module
pub type OTG_HS_HCTSIZ3 = crate::Reg<u32, _OTG_HS_HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ3;
///`read()` method returns [otg_hs_hctsiz3::R](otg_hs_hctsiz3::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ3 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz3::W](otg_hs_hctsiz3::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ3 {}
///OTG_HS host channel-3 transfer size register
pub mod otg_hs_hctsiz3;
///OTG_HS host channel-4 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz4](otg_hs_hctsiz4) module
pub type OTG_HS_HCTSIZ4 = crate::Reg<u32, _OTG_HS_HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ4;
///`read()` method returns [otg_hs_hctsiz4::R](otg_hs_hctsiz4::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ4 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz4::W](otg_hs_hctsiz4::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ4 {}
///OTG_HS host channel-4 transfer size register
pub mod otg_hs_hctsiz4;
///OTG_HS host channel-5 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz5](otg_hs_hctsiz5) module
pub type OTG_HS_HCTSIZ5 = crate::Reg<u32, _OTG_HS_HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ5;
///`read()` method returns [otg_hs_hctsiz5::R](otg_hs_hctsiz5::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ5 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz5::W](otg_hs_hctsiz5::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ5 {}
///OTG_HS host channel-5 transfer size register
pub mod otg_hs_hctsiz5;
///OTG_HS host channel-6 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz6](otg_hs_hctsiz6) module
pub type OTG_HS_HCTSIZ6 = crate::Reg<u32, _OTG_HS_HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ6;
///`read()` method returns [otg_hs_hctsiz6::R](otg_hs_hctsiz6::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ6 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz6::W](otg_hs_hctsiz6::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ6 {}
///OTG_HS host channel-6 transfer size register
pub mod otg_hs_hctsiz6;
///OTG_HS host channel-7 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz7](otg_hs_hctsiz7) module
pub type OTG_HS_HCTSIZ7 = crate::Reg<u32, _OTG_HS_HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ7;
///`read()` method returns [otg_hs_hctsiz7::R](otg_hs_hctsiz7::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ7 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz7::W](otg_hs_hctsiz7::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ7 {}
///OTG_HS host channel-7 transfer size register
pub mod otg_hs_hctsiz7;
///OTG_HS host channel-8 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz8](otg_hs_hctsiz8) module
pub type OTG_HS_HCTSIZ8 = crate::Reg<u32, _OTG_HS_HCTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ8;
///`read()` method returns [otg_hs_hctsiz8::R](otg_hs_hctsiz8::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ8 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz8::W](otg_hs_hctsiz8::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ8 {}
///OTG_HS host channel-8 transfer size register
pub mod otg_hs_hctsiz8;
///OTG_HS host channel-9 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz9](otg_hs_hctsiz9) module
pub type OTG_HS_HCTSIZ9 = crate::Reg<u32, _OTG_HS_HCTSIZ9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ9;
///`read()` method returns [otg_hs_hctsiz9::R](otg_hs_hctsiz9::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ9 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz9::W](otg_hs_hctsiz9::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ9 {}
///OTG_HS host channel-9 transfer size register
pub mod otg_hs_hctsiz9;
///OTG_HS host channel-10 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz10](otg_hs_hctsiz10) module
pub type OTG_HS_HCTSIZ10 = crate::Reg<u32, _OTG_HS_HCTSIZ10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ10;
///`read()` method returns [otg_hs_hctsiz10::R](otg_hs_hctsiz10::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ10 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz10::W](otg_hs_hctsiz10::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ10 {}
///OTG_HS host channel-10 transfer size register
pub mod otg_hs_hctsiz10;
///OTG_HS host channel-11 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz11](otg_hs_hctsiz11) module
pub type OTG_HS_HCTSIZ11 = crate::Reg<u32, _OTG_HS_HCTSIZ11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ11;
///`read()` method returns [otg_hs_hctsiz11::R](otg_hs_hctsiz11::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ11 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz11::W](otg_hs_hctsiz11::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ11 {}
///OTG_HS host channel-11 transfer size register
pub mod otg_hs_hctsiz11;
///OTG_HS host channel-0 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma0](otg_hs_hcdma0) module
pub type OTG_HS_HCDMA0 = crate::Reg<u32, _OTG_HS_HCDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA0;
///`read()` method returns [otg_hs_hcdma0::R](otg_hs_hcdma0::R) reader structure
impl crate::Readable for OTG_HS_HCDMA0 {}
///`write(|w| ..)` method takes [otg_hs_hcdma0::W](otg_hs_hcdma0::W) writer structure
impl crate::Writable for OTG_HS_HCDMA0 {}
///OTG_HS host channel-0 DMA address register
pub mod otg_hs_hcdma0;
///OTG_HS host channel-1 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma1](otg_hs_hcdma1) module
pub type OTG_HS_HCDMA1 = crate::Reg<u32, _OTG_HS_HCDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA1;
///`read()` method returns [otg_hs_hcdma1::R](otg_hs_hcdma1::R) reader structure
impl crate::Readable for OTG_HS_HCDMA1 {}
///`write(|w| ..)` method takes [otg_hs_hcdma1::W](otg_hs_hcdma1::W) writer structure
impl crate::Writable for OTG_HS_HCDMA1 {}
///OTG_HS host channel-1 DMA address register
pub mod otg_hs_hcdma1;
///OTG_HS host channel-2 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma2](otg_hs_hcdma2) module
pub type OTG_HS_HCDMA2 = crate::Reg<u32, _OTG_HS_HCDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA2;
///`read()` method returns [otg_hs_hcdma2::R](otg_hs_hcdma2::R) reader structure
impl crate::Readable for OTG_HS_HCDMA2 {}
///`write(|w| ..)` method takes [otg_hs_hcdma2::W](otg_hs_hcdma2::W) writer structure
impl crate::Writable for OTG_HS_HCDMA2 {}
///OTG_HS host channel-2 DMA address register
pub mod otg_hs_hcdma2;
///OTG_HS host channel-3 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma3](otg_hs_hcdma3) module
pub type OTG_HS_HCDMA3 = crate::Reg<u32, _OTG_HS_HCDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA3;
///`read()` method returns [otg_hs_hcdma3::R](otg_hs_hcdma3::R) reader structure
impl crate::Readable for OTG_HS_HCDMA3 {}
///`write(|w| ..)` method takes [otg_hs_hcdma3::W](otg_hs_hcdma3::W) writer structure
impl crate::Writable for OTG_HS_HCDMA3 {}
///OTG_HS host channel-3 DMA address register
pub mod otg_hs_hcdma3;
///OTG_HS host channel-4 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma4](otg_hs_hcdma4) module
pub type OTG_HS_HCDMA4 = crate::Reg<u32, _OTG_HS_HCDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA4;
///`read()` method returns [otg_hs_hcdma4::R](otg_hs_hcdma4::R) reader structure
impl crate::Readable for OTG_HS_HCDMA4 {}
///`write(|w| ..)` method takes [otg_hs_hcdma4::W](otg_hs_hcdma4::W) writer structure
impl crate::Writable for OTG_HS_HCDMA4 {}
///OTG_HS host channel-4 DMA address register
pub mod otg_hs_hcdma4;
///OTG_HS host channel-5 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma5](otg_hs_hcdma5) module
pub type OTG_HS_HCDMA5 = crate::Reg<u32, _OTG_HS_HCDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA5;
///`read()` method returns [otg_hs_hcdma5::R](otg_hs_hcdma5::R) reader structure
impl crate::Readable for OTG_HS_HCDMA5 {}
///`write(|w| ..)` method takes [otg_hs_hcdma5::W](otg_hs_hcdma5::W) writer structure
impl crate::Writable for OTG_HS_HCDMA5 {}
///OTG_HS host channel-5 DMA address register
pub mod otg_hs_hcdma5;
///OTG_HS host channel-6 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma6](otg_hs_hcdma6) module
pub type OTG_HS_HCDMA6 = crate::Reg<u32, _OTG_HS_HCDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA6;
///`read()` method returns [otg_hs_hcdma6::R](otg_hs_hcdma6::R) reader structure
impl crate::Readable for OTG_HS_HCDMA6 {}
///`write(|w| ..)` method takes [otg_hs_hcdma6::W](otg_hs_hcdma6::W) writer structure
impl crate::Writable for OTG_HS_HCDMA6 {}
///OTG_HS host channel-6 DMA address register
pub mod otg_hs_hcdma6;
///OTG_HS host channel-7 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma7](otg_hs_hcdma7) module
pub type OTG_HS_HCDMA7 = crate::Reg<u32, _OTG_HS_HCDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA7;
///`read()` method returns [otg_hs_hcdma7::R](otg_hs_hcdma7::R) reader structure
impl crate::Readable for OTG_HS_HCDMA7 {}
///`write(|w| ..)` method takes [otg_hs_hcdma7::W](otg_hs_hcdma7::W) writer structure
impl crate::Writable for OTG_HS_HCDMA7 {}
///OTG_HS host channel-7 DMA address register
pub mod otg_hs_hcdma7;
///OTG_HS host channel-8 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma8](otg_hs_hcdma8) module
pub type OTG_HS_HCDMA8 = crate::Reg<u32, _OTG_HS_HCDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA8;
///`read()` method returns [otg_hs_hcdma8::R](otg_hs_hcdma8::R) reader structure
impl crate::Readable for OTG_HS_HCDMA8 {}
///`write(|w| ..)` method takes [otg_hs_hcdma8::W](otg_hs_hcdma8::W) writer structure
impl crate::Writable for OTG_HS_HCDMA8 {}
///OTG_HS host channel-8 DMA address register
pub mod otg_hs_hcdma8;
///OTG_HS host channel-9 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma9](otg_hs_hcdma9) module
pub type OTG_HS_HCDMA9 = crate::Reg<u32, _OTG_HS_HCDMA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA9;
///`read()` method returns [otg_hs_hcdma9::R](otg_hs_hcdma9::R) reader structure
impl crate::Readable for OTG_HS_HCDMA9 {}
///`write(|w| ..)` method takes [otg_hs_hcdma9::W](otg_hs_hcdma9::W) writer structure
impl crate::Writable for OTG_HS_HCDMA9 {}
///OTG_HS host channel-9 DMA address register
pub mod otg_hs_hcdma9;
///OTG_HS host channel-10 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma10](otg_hs_hcdma10) module
pub type OTG_HS_HCDMA10 = crate::Reg<u32, _OTG_HS_HCDMA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA10;
///`read()` method returns [otg_hs_hcdma10::R](otg_hs_hcdma10::R) reader structure
impl crate::Readable for OTG_HS_HCDMA10 {}
///`write(|w| ..)` method takes [otg_hs_hcdma10::W](otg_hs_hcdma10::W) writer structure
impl crate::Writable for OTG_HS_HCDMA10 {}
///OTG_HS host channel-10 DMA address register
pub mod otg_hs_hcdma10;
///OTG_HS host channel-11 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma11](otg_hs_hcdma11) module
pub type OTG_HS_HCDMA11 = crate::Reg<u32, _OTG_HS_HCDMA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA11;
///`read()` method returns [otg_hs_hcdma11::R](otg_hs_hcdma11::R) reader structure
impl crate::Readable for OTG_HS_HCDMA11 {}
///`write(|w| ..)` method takes [otg_hs_hcdma11::W](otg_hs_hcdma11::W) writer structure
impl crate::Writable for OTG_HS_HCDMA11 {}
///OTG_HS host channel-11 DMA address register
pub mod otg_hs_hcdma11;
///OTG_HS host channel-12 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar12](otg_hs_hcchar12) module
pub type OTG_HS_HCCHAR12 = crate::Reg<u32, _OTG_HS_HCCHAR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR12;
///`read()` method returns [otg_hs_hcchar12::R](otg_hs_hcchar12::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR12 {}
///`write(|w| ..)` method takes [otg_hs_hcchar12::W](otg_hs_hcchar12::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR12 {}
///OTG_HS host channel-12 characteristics register
pub mod otg_hs_hcchar12;
///OTG_HS host channel-12 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt12](otg_hs_hcsplt12) module
pub type OTG_HS_HCSPLT12 = crate::Reg<u32, _OTG_HS_HCSPLT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT12;
///`read()` method returns [otg_hs_hcsplt12::R](otg_hs_hcsplt12::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT12 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt12::W](otg_hs_hcsplt12::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT12 {}
///OTG_HS host channel-12 split control register
pub mod otg_hs_hcsplt12;
///OTG_HS host channel-12 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint12](otg_hs_hcint12) module
pub type OTG_HS_HCINT12 = crate::Reg<u32, _OTG_HS_HCINT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT12;
///`read()` method returns [otg_hs_hcint12::R](otg_hs_hcint12::R) reader structure
impl crate::Readable for OTG_HS_HCINT12 {}
///`write(|w| ..)` method takes [otg_hs_hcint12::W](otg_hs_hcint12::W) writer structure
impl crate::Writable for OTG_HS_HCINT12 {}
///OTG_HS host channel-12 interrupt register
pub mod otg_hs_hcint12;
///OTG_HS host channel-12 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk12](otg_hs_hcintmsk12) module
pub type OTG_HS_HCINTMSK12 = crate::Reg<u32, _OTG_HS_HCINTMSK12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK12;
///`read()` method returns [otg_hs_hcintmsk12::R](otg_hs_hcintmsk12::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK12 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk12::W](otg_hs_hcintmsk12::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK12 {}
///OTG_HS host channel-12 interrupt mask register
pub mod otg_hs_hcintmsk12;
///OTG_HS host channel-12 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz12](otg_hs_hctsiz12) module
pub type OTG_HS_HCTSIZ12 = crate::Reg<u32, _OTG_HS_HCTSIZ12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ12;
///`read()` method returns [otg_hs_hctsiz12::R](otg_hs_hctsiz12::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ12 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz12::W](otg_hs_hctsiz12::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ12 {}
///OTG_HS host channel-12 transfer size register
pub mod otg_hs_hctsiz12;
///OTG_HS host channel-12 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma12](otg_hs_hcdma12) module
pub type OTG_HS_HCDMA12 = crate::Reg<u32, _OTG_HS_HCDMA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA12;
///`read()` method returns [otg_hs_hcdma12::R](otg_hs_hcdma12::R) reader structure
impl crate::Readable for OTG_HS_HCDMA12 {}
///`write(|w| ..)` method takes [otg_hs_hcdma12::W](otg_hs_hcdma12::W) writer structure
impl crate::Writable for OTG_HS_HCDMA12 {}
///OTG_HS host channel-12 DMA address register
pub mod otg_hs_hcdma12;
///OTG_HS host channel-13 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar13](otg_hs_hcchar13) module
pub type OTG_HS_HCCHAR13 = crate::Reg<u32, _OTG_HS_HCCHAR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR13;
///`read()` method returns [otg_hs_hcchar13::R](otg_hs_hcchar13::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR13 {}
///`write(|w| ..)` method takes [otg_hs_hcchar13::W](otg_hs_hcchar13::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR13 {}
///OTG_HS host channel-13 characteristics register
pub mod otg_hs_hcchar13;
///OTG_HS host channel-13 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt13](otg_hs_hcsplt13) module
pub type OTG_HS_HCSPLT13 = crate::Reg<u32, _OTG_HS_HCSPLT13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT13;
///`read()` method returns [otg_hs_hcsplt13::R](otg_hs_hcsplt13::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT13 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt13::W](otg_hs_hcsplt13::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT13 {}
///OTG_HS host channel-13 split control register
pub mod otg_hs_hcsplt13;
///OTG_HS host channel-13 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint13](otg_hs_hcint13) module
pub type OTG_HS_HCINT13 = crate::Reg<u32, _OTG_HS_HCINT13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT13;
///`read()` method returns [otg_hs_hcint13::R](otg_hs_hcint13::R) reader structure
impl crate::Readable for OTG_HS_HCINT13 {}
///`write(|w| ..)` method takes [otg_hs_hcint13::W](otg_hs_hcint13::W) writer structure
impl crate::Writable for OTG_HS_HCINT13 {}
///OTG_HS host channel-13 interrupt register
pub mod otg_hs_hcint13;
///OTG_HS host channel-13 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk13](otg_hs_hcintmsk13) module
pub type OTG_HS_HCINTMSK13 = crate::Reg<u32, _OTG_HS_HCINTMSK13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK13;
///`read()` method returns [otg_hs_hcintmsk13::R](otg_hs_hcintmsk13::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK13 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk13::W](otg_hs_hcintmsk13::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK13 {}
///OTG_HS host channel-13 interrupt mask register
pub mod otg_hs_hcintmsk13;
///OTG_HS host channel-13 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz13](otg_hs_hctsiz13) module
pub type OTG_HS_HCTSIZ13 = crate::Reg<u32, _OTG_HS_HCTSIZ13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ13;
///`read()` method returns [otg_hs_hctsiz13::R](otg_hs_hctsiz13::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ13 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz13::W](otg_hs_hctsiz13::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ13 {}
///OTG_HS host channel-13 transfer size register
pub mod otg_hs_hctsiz13;
///OTG_HS host channel-13 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma13](otg_hs_hcdma13) module
pub type OTG_HS_HCDMA13 = crate::Reg<u32, _OTG_HS_HCDMA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA13;
///`read()` method returns [otg_hs_hcdma13::R](otg_hs_hcdma13::R) reader structure
impl crate::Readable for OTG_HS_HCDMA13 {}
///`write(|w| ..)` method takes [otg_hs_hcdma13::W](otg_hs_hcdma13::W) writer structure
impl crate::Writable for OTG_HS_HCDMA13 {}
///OTG_HS host channel-13 DMA address register
pub mod otg_hs_hcdma13;
///OTG_HS host channel-14 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar14](otg_hs_hcchar14) module
pub type OTG_HS_HCCHAR14 = crate::Reg<u32, _OTG_HS_HCCHAR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR14;
///`read()` method returns [otg_hs_hcchar14::R](otg_hs_hcchar14::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR14 {}
///`write(|w| ..)` method takes [otg_hs_hcchar14::W](otg_hs_hcchar14::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR14 {}
///OTG_HS host channel-14 characteristics register
pub mod otg_hs_hcchar14;
///OTG_HS host channel-14 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt14](otg_hs_hcsplt14) module
pub type OTG_HS_HCSPLT14 = crate::Reg<u32, _OTG_HS_HCSPLT14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT14;
///`read()` method returns [otg_hs_hcsplt14::R](otg_hs_hcsplt14::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT14 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt14::W](otg_hs_hcsplt14::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT14 {}
///OTG_HS host channel-14 split control register
pub mod otg_hs_hcsplt14;
///OTG_HS host channel-14 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint14](otg_hs_hcint14) module
pub type OTG_HS_HCINT14 = crate::Reg<u32, _OTG_HS_HCINT14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT14;
///`read()` method returns [otg_hs_hcint14::R](otg_hs_hcint14::R) reader structure
impl crate::Readable for OTG_HS_HCINT14 {}
///`write(|w| ..)` method takes [otg_hs_hcint14::W](otg_hs_hcint14::W) writer structure
impl crate::Writable for OTG_HS_HCINT14 {}
///OTG_HS host channel-14 interrupt register
pub mod otg_hs_hcint14;
///OTG_HS host channel-14 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk14](otg_hs_hcintmsk14) module
pub type OTG_HS_HCINTMSK14 = crate::Reg<u32, _OTG_HS_HCINTMSK14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK14;
///`read()` method returns [otg_hs_hcintmsk14::R](otg_hs_hcintmsk14::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK14 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk14::W](otg_hs_hcintmsk14::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK14 {}
///OTG_HS host channel-14 interrupt mask register
pub mod otg_hs_hcintmsk14;
///OTG_HS host channel-14 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz14](otg_hs_hctsiz14) module
pub type OTG_HS_HCTSIZ14 = crate::Reg<u32, _OTG_HS_HCTSIZ14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ14;
///`read()` method returns [otg_hs_hctsiz14::R](otg_hs_hctsiz14::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ14 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz14::W](otg_hs_hctsiz14::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ14 {}
///OTG_HS host channel-14 transfer size register
pub mod otg_hs_hctsiz14;
///OTG_HS host channel-14 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma14](otg_hs_hcdma14) module
pub type OTG_HS_HCDMA14 = crate::Reg<u32, _OTG_HS_HCDMA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA14;
///`read()` method returns [otg_hs_hcdma14::R](otg_hs_hcdma14::R) reader structure
impl crate::Readable for OTG_HS_HCDMA14 {}
///`write(|w| ..)` method takes [otg_hs_hcdma14::W](otg_hs_hcdma14::W) writer structure
impl crate::Writable for OTG_HS_HCDMA14 {}
///OTG_HS host channel-14 DMA address register
pub mod otg_hs_hcdma14;
///OTG_HS host channel-15 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcchar15](otg_hs_hcchar15) module
pub type OTG_HS_HCCHAR15 = crate::Reg<u32, _OTG_HS_HCCHAR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR15;
///`read()` method returns [otg_hs_hcchar15::R](otg_hs_hcchar15::R) reader structure
impl crate::Readable for OTG_HS_HCCHAR15 {}
///`write(|w| ..)` method takes [otg_hs_hcchar15::W](otg_hs_hcchar15::W) writer structure
impl crate::Writable for OTG_HS_HCCHAR15 {}
///OTG_HS host channel-15 characteristics register
pub mod otg_hs_hcchar15;
///OTG_HS host channel-15 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcsplt15](otg_hs_hcsplt15) module
pub type OTG_HS_HCSPLT15 = crate::Reg<u32, _OTG_HS_HCSPLT15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT15;
///`read()` method returns [otg_hs_hcsplt15::R](otg_hs_hcsplt15::R) reader structure
impl crate::Readable for OTG_HS_HCSPLT15 {}
///`write(|w| ..)` method takes [otg_hs_hcsplt15::W](otg_hs_hcsplt15::W) writer structure
impl crate::Writable for OTG_HS_HCSPLT15 {}
///OTG_HS host channel-15 split control register
pub mod otg_hs_hcsplt15;
///OTG_HS host channel-15 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcint15](otg_hs_hcint15) module
pub type OTG_HS_HCINT15 = crate::Reg<u32, _OTG_HS_HCINT15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT15;
///`read()` method returns [otg_hs_hcint15::R](otg_hs_hcint15::R) reader structure
impl crate::Readable for OTG_HS_HCINT15 {}
///`write(|w| ..)` method takes [otg_hs_hcint15::W](otg_hs_hcint15::W) writer structure
impl crate::Writable for OTG_HS_HCINT15 {}
///OTG_HS host channel-15 interrupt register
pub mod otg_hs_hcint15;
///OTG_HS host channel-15 interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcintmsk15](otg_hs_hcintmsk15) module
pub type OTG_HS_HCINTMSK15 = crate::Reg<u32, _OTG_HS_HCINTMSK15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK15;
///`read()` method returns [otg_hs_hcintmsk15::R](otg_hs_hcintmsk15::R) reader structure
impl crate::Readable for OTG_HS_HCINTMSK15 {}
///`write(|w| ..)` method takes [otg_hs_hcintmsk15::W](otg_hs_hcintmsk15::W) writer structure
impl crate::Writable for OTG_HS_HCINTMSK15 {}
///OTG_HS host channel-15 interrupt mask register
pub mod otg_hs_hcintmsk15;
///OTG_HS host channel-15 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hctsiz15](otg_hs_hctsiz15) module
pub type OTG_HS_HCTSIZ15 = crate::Reg<u32, _OTG_HS_HCTSIZ15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ15;
///`read()` method returns [otg_hs_hctsiz15::R](otg_hs_hctsiz15::R) reader structure
impl crate::Readable for OTG_HS_HCTSIZ15 {}
///`write(|w| ..)` method takes [otg_hs_hctsiz15::W](otg_hs_hctsiz15::W) writer structure
impl crate::Writable for OTG_HS_HCTSIZ15 {}
///OTG_HS host channel-15 transfer size register
pub mod otg_hs_hctsiz15;
///OTG_HS host channel-15 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_hcdma15](otg_hs_hcdma15) module
pub type OTG_HS_HCDMA15 = crate::Reg<u32, _OTG_HS_HCDMA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA15;
///`read()` method returns [otg_hs_hcdma15::R](otg_hs_hcdma15::R) reader structure
impl crate::Readable for OTG_HS_HCDMA15 {}
///`write(|w| ..)` method takes [otg_hs_hcdma15::W](otg_hs_hcdma15::W) writer structure
impl crate::Writable for OTG_HS_HCDMA15 {}
///OTG_HS host channel-15 DMA address register
pub mod otg_hs_hcdma15;
