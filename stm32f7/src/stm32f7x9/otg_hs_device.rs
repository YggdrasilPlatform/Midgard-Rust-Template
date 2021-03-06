///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_HS device configuration register
    pub otg_hs_dcfg: OTG_HS_DCFG,
    ///0x04 - OTG_HS device control register
    pub otg_hs_dctl: OTG_HS_DCTL,
    ///0x08 - OTG_HS device status register
    pub otg_hs_dsts: OTG_HS_DSTS,
    _reserved3: [u8; 4usize],
    ///0x10 - OTG_HS device IN endpoint common interrupt mask register
    pub otg_hs_diepmsk: OTG_HS_DIEPMSK,
    ///0x14 - OTG_HS device OUT endpoint common interrupt mask register
    pub otg_hs_doepmsk: OTG_HS_DOEPMSK,
    ///0x18 - OTG_HS device all endpoints interrupt register
    pub otg_hs_daint: OTG_HS_DAINT,
    ///0x1c - OTG_HS all endpoints interrupt mask register
    pub otg_hs_daintmsk: OTG_HS_DAINTMSK,
    _reserved7: [u8; 8usize],
    ///0x28 - OTG_HS device VBUS discharge time register
    pub otg_hs_dvbusdis: OTG_HS_DVBUSDIS,
    ///0x2c - OTG_HS device VBUS pulsing time register
    pub otg_hs_dvbuspulse: OTG_HS_DVBUSPULSE,
    ///0x30 - OTG_HS Device threshold control register
    pub otg_hs_dthrctl: OTG_HS_DTHRCTL,
    ///0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register
    pub otg_hs_diepempmsk: OTG_HS_DIEPEMPMSK,
    ///0x38 - OTG_HS device each endpoint interrupt register
    pub otg_hs_deachint: OTG_HS_DEACHINT,
    ///0x3c - OTG_HS device each endpoint interrupt register mask
    pub otg_hs_deachintmsk: OTG_HS_DEACHINTMSK,
    _reserved13: [u8; 192usize],
    ///0x100 - OTG device endpoint-0 control register
    pub otg_hs_diepctl0: OTG_HS_DIEPCTL0,
    _reserved14: [u8; 4usize],
    ///0x108 - OTG device endpoint-0 interrupt register
    pub otg_hs_diepint0: OTG_HS_DIEPINT0,
    _reserved15: [u8; 4usize],
    ///0x110 - OTG_HS device IN endpoint 0 transfer size register
    pub otg_hs_dieptsiz0: OTG_HS_DIEPTSIZ0,
    ///0x114 - OTG_HS device endpoint-1 DMA address register
    pub otg_hs_diepdma1: OTG_HS_DIEPDMA1,
    ///0x118 - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts0: OTG_HS_DTXFSTS0,
    _reserved18: [u8; 4usize],
    ///0x120 - OTG device endpoint-1 control register
    pub otg_hs_diepctl1: OTG_HS_DIEPCTL1,
    _reserved19: [u8; 4usize],
    ///0x128 - OTG device endpoint-1 interrupt register
    pub otg_hs_diepint1: OTG_HS_DIEPINT1,
    _reserved20: [u8; 4usize],
    ///0x130 - OTG_HS device endpoint transfer size register
    pub otg_hs_dieptsiz1: OTG_HS_DIEPTSIZ1,
    ///0x134 - OTG_HS device endpoint-2 DMA address register
    pub otg_hs_diepdma2: OTG_HS_DIEPDMA2,
    ///0x138 - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts1: OTG_HS_DTXFSTS1,
    _reserved23: [u8; 4usize],
    ///0x140 - OTG device endpoint-2 control register
    pub otg_hs_diepctl2: OTG_HS_DIEPCTL2,
    _reserved24: [u8; 4usize],
    ///0x148 - OTG device endpoint-2 interrupt register
    pub otg_hs_diepint2: OTG_HS_DIEPINT2,
    _reserved25: [u8; 4usize],
    ///0x150 - OTG_HS device endpoint transfer size register
    pub otg_hs_dieptsiz2: OTG_HS_DIEPTSIZ2,
    ///0x154 - OTG_HS device endpoint-3 DMA address register
    pub otg_hs_diepdma3: OTG_HS_DIEPDMA3,
    ///0x158 - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts2: OTG_HS_DTXFSTS2,
    _reserved28: [u8; 4usize],
    ///0x160 - OTG device endpoint-3 control register
    pub otg_hs_diepctl3: OTG_HS_DIEPCTL3,
    _reserved29: [u8; 4usize],
    ///0x168 - OTG device endpoint-3 interrupt register
    pub otg_hs_diepint3: OTG_HS_DIEPINT3,
    _reserved30: [u8; 4usize],
    ///0x170 - OTG_HS device endpoint transfer size register
    pub otg_hs_dieptsiz3: OTG_HS_DIEPTSIZ3,
    ///0x174 - OTG_HS device endpoint-4 DMA address register
    pub otg_hs_diepdma4: OTG_HS_DIEPDMA4,
    ///0x178 - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts3: OTG_HS_DTXFSTS3,
    _reserved33: [u8; 4usize],
    ///0x180 - OTG device endpoint-4 control register
    pub otg_hs_diepctl4: OTG_HS_DIEPCTL4,
    _reserved34: [u8; 4usize],
    ///0x188 - OTG device endpoint-4 interrupt register
    pub otg_hs_diepint4: OTG_HS_DIEPINT4,
    _reserved35: [u8; 4usize],
    ///0x190 - OTG_HS device endpoint transfer size register
    pub otg_hs_dieptsiz4: OTG_HS_DIEPTSIZ4,
    ///0x194 - OTG_HS device endpoint-5 DMA address register
    pub otg_hs_diepdma5: OTG_HS_DIEPDMA5,
    ///0x198 - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts4: OTG_HS_DTXFSTS4,
    _reserved38: [u8; 4usize],
    _reserved_38_otg_hs: [u8; 4usize],
    ///0x1a4 - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts6: OTG_HS_DTXFSTS6,
    _reserved_40_otg_hs: [u8; 4usize],
    ///0x1ac - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts7: OTG_HS_DTXFSTS7,
    ///0x1b0 - OTG_HS device endpoint transfer size register
    pub otg_hs_dieptsiz5: OTG_HS_DIEPTSIZ5,
    _reserved43: [u8; 4usize],
    ///0x1b8 - OTG_HS device IN endpoint transmit FIFO status register
    pub otg_hs_dtxfsts5: OTG_HS_DTXFSTS5,
    _reserved44: [u8; 4usize],
    ///0x1c0 - OTG device endpoint-6 control register
    pub otg_hs_diepctl6: OTG_HS_DIEPCTL6,
    _reserved45: [u8; 4usize],
    ///0x1c8 - OTG device endpoint-6 interrupt register
    pub otg_hs_diepint6: OTG_HS_DIEPINT6,
    _reserved46: [u8; 20usize],
    ///0x1e0 - OTG device endpoint-7 control register
    pub otg_hs_diepctl7: OTG_HS_DIEPCTL7,
    _reserved47: [u8; 4usize],
    ///0x1e8 - OTG device endpoint-7 interrupt register
    pub otg_hs_diepint7: OTG_HS_DIEPINT7,
    _reserved48: [u8; 276usize],
    ///0x300 - OTG_HS device control OUT endpoint 0 control register
    pub otg_hs_doepctl0: OTG_HS_DOEPCTL0,
    _reserved49: [u8; 4usize],
    ///0x308 - OTG_HS device endpoint-0 interrupt register
    pub otg_hs_doepint0: OTG_HS_DOEPINT0,
    _reserved50: [u8; 4usize],
    ///0x310 - OTG_HS device endpoint-0 transfer size register
    pub otg_hs_doeptsiz0: OTG_HS_DOEPTSIZ0,
    _reserved51: [u8; 12usize],
    ///0x320 - OTG device endpoint-1 control register
    pub otg_hs_doepctl1: OTG_HS_DOEPCTL1,
    _reserved52: [u8; 4usize],
    ///0x328 - OTG_HS device endpoint-1 interrupt register
    pub otg_hs_doepint1: OTG_HS_DOEPINT1,
    _reserved53: [u8; 4usize],
    ///0x330 - OTG_HS device endpoint-1 transfer size register
    pub otg_hs_doeptsiz1: OTG_HS_DOEPTSIZ1,
    _reserved54: [u8; 12usize],
    ///0x340 - OTG device endpoint-2 control register
    pub otg_hs_doepctl2: OTG_HS_DOEPCTL2,
    _reserved55: [u8; 4usize],
    ///0x348 - OTG_HS device endpoint-2 interrupt register
    pub otg_hs_doepint2: OTG_HS_DOEPINT2,
    _reserved56: [u8; 4usize],
    ///0x350 - OTG_HS device endpoint-2 transfer size register
    pub otg_hs_doeptsiz2: OTG_HS_DOEPTSIZ2,
    _reserved57: [u8; 12usize],
    ///0x360 - OTG device endpoint-3 control register
    pub otg_hs_doepctl3: OTG_HS_DOEPCTL3,
    _reserved58: [u8; 4usize],
    ///0x368 - OTG_HS device endpoint-3 interrupt register
    pub otg_hs_doepint3: OTG_HS_DOEPINT3,
    _reserved59: [u8; 4usize],
    ///0x370 - OTG_HS device endpoint-3 transfer size register
    pub otg_hs_doeptsiz3: OTG_HS_DOEPTSIZ3,
    _reserved60: [u8; 12usize],
    ///0x380 - OTG device endpoint-4 control register
    pub otg_hs_doepctl4: OTG_HS_DOEPCTL4,
    _reserved61: [u8; 4usize],
    ///0x388 - OTG_HS device endpoint-4 interrupt register
    pub otg_hs_doepint4: OTG_HS_DOEPINT4,
    _reserved62: [u8; 4usize],
    ///0x390 - OTG_HS device endpoint-4 transfer size register
    pub otg_hs_doeptsiz4: OTG_HS_DOEPTSIZ4,
    _reserved63: [u8; 12usize],
    ///0x3a0 - OTG device endpoint-5 control register
    pub otg_hs_doepctl5: OTG_HS_DOEPCTL5,
    _reserved64: [u8; 4usize],
    ///0x3a8 - OTG_HS device endpoint-5 interrupt register
    pub otg_hs_doepint5: OTG_HS_DOEPINT5,
    _reserved65: [u8; 4usize],
    ///0x3b0 - OTG_HS device endpoint-5 transfer size register
    pub otg_hs_doeptsiz5: OTG_HS_DOEPTSIZ5,
    _reserved66: [u8; 12usize],
    ///0x3c0 - OTG device endpoint-6 control register
    pub otg_hs_doepctl6: OTG_HS_DOEPCTL6,
    _reserved67: [u8; 4usize],
    ///0x3c8 - OTG_HS device endpoint-6 interrupt register
    pub otg_hs_doepint6: OTG_HS_DOEPINT6,
    _reserved68: [u8; 4usize],
    ///0x3d0 - OTG_HS device endpoint-6 transfer size register
    pub otg_hs_doeptsiz6: OTG_HS_DOEPTSIZ6,
    _reserved69: [u8; 12usize],
    ///0x3e0 - OTG device endpoint-7 control register
    pub otg_hs_doepctl7: OTG_HS_DOEPCTL7,
    _reserved70: [u8; 4usize],
    ///0x3e8 - OTG_HS device endpoint-7 interrupt register
    pub otg_hs_doepint7: OTG_HS_DOEPINT7,
    _reserved71: [u8; 4usize],
    ///0x3f0 - OTG_HS device endpoint-7 transfer size register
    pub otg_hs_doeptsiz7: OTG_HS_DOEPTSIZ7,
}
impl RegisterBlock {
    ///0x1a0 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub fn otg_hs_dieptsiz6(&self) -> &OTG_HS_DIEPTSIZ6 {
        unsafe { &*(((self as *const Self) as *const u8).add(416usize) as *const OTG_HS_DIEPTSIZ6) }
    }
    ///0x1a0 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub fn otg_hs_dieptsiz6_mut(&self) -> &mut OTG_HS_DIEPTSIZ6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(416usize) as *mut OTG_HS_DIEPTSIZ6) }
    }
    ///0x1a0 - OTG device endpoint-5 control register
    #[inline(always)]
    pub fn otg_hs_diepctl5(&self) -> &OTG_HS_DIEPCTL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(416usize) as *const OTG_HS_DIEPCTL5) }
    }
    ///0x1a0 - OTG device endpoint-5 control register
    #[inline(always)]
    pub fn otg_hs_diepctl5_mut(&self) -> &mut OTG_HS_DIEPCTL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(416usize) as *mut OTG_HS_DIEPCTL5) }
    }
    ///0x1a8 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub fn otg_hs_dieptsiz7(&self) -> &OTG_HS_DIEPTSIZ7 {
        unsafe { &*(((self as *const Self) as *const u8).add(424usize) as *const OTG_HS_DIEPTSIZ7) }
    }
    ///0x1a8 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub fn otg_hs_dieptsiz7_mut(&self) -> &mut OTG_HS_DIEPTSIZ7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(424usize) as *mut OTG_HS_DIEPTSIZ7) }
    }
    ///0x1a8 - OTG device endpoint-5 interrupt register
    #[inline(always)]
    pub fn otg_hs_diepint5(&self) -> &OTG_HS_DIEPINT5 {
        unsafe { &*(((self as *const Self) as *const u8).add(424usize) as *const OTG_HS_DIEPINT5) }
    }
    ///0x1a8 - OTG device endpoint-5 interrupt register
    #[inline(always)]
    pub fn otg_hs_diepint5_mut(&self) -> &mut OTG_HS_DIEPINT5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(424usize) as *mut OTG_HS_DIEPINT5) }
    }
}
///OTG_HS device configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dcfg](otg_hs_dcfg) module
pub type OTG_HS_DCFG = crate::Reg<u32, _OTG_HS_DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DCFG;
///`read()` method returns [otg_hs_dcfg::R](otg_hs_dcfg::R) reader structure
impl crate::Readable for OTG_HS_DCFG {}
///`write(|w| ..)` method takes [otg_hs_dcfg::W](otg_hs_dcfg::W) writer structure
impl crate::Writable for OTG_HS_DCFG {}
///OTG_HS device configuration register
pub mod otg_hs_dcfg;
///OTG_HS device control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dctl](otg_hs_dctl) module
pub type OTG_HS_DCTL = crate::Reg<u32, _OTG_HS_DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DCTL;
///`read()` method returns [otg_hs_dctl::R](otg_hs_dctl::R) reader structure
impl crate::Readable for OTG_HS_DCTL {}
///`write(|w| ..)` method takes [otg_hs_dctl::W](otg_hs_dctl::W) writer structure
impl crate::Writable for OTG_HS_DCTL {}
///OTG_HS device control register
pub mod otg_hs_dctl;
///OTG_HS device status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dsts](otg_hs_dsts) module
pub type OTG_HS_DSTS = crate::Reg<u32, _OTG_HS_DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DSTS;
///`read()` method returns [otg_hs_dsts::R](otg_hs_dsts::R) reader structure
impl crate::Readable for OTG_HS_DSTS {}
///OTG_HS device status register
pub mod otg_hs_dsts;
///OTG_HS device IN endpoint common interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepmsk](otg_hs_diepmsk) module
pub type OTG_HS_DIEPMSK = crate::Reg<u32, _OTG_HS_DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPMSK;
///`read()` method returns [otg_hs_diepmsk::R](otg_hs_diepmsk::R) reader structure
impl crate::Readable for OTG_HS_DIEPMSK {}
///`write(|w| ..)` method takes [otg_hs_diepmsk::W](otg_hs_diepmsk::W) writer structure
impl crate::Writable for OTG_HS_DIEPMSK {}
///OTG_HS device IN endpoint common interrupt mask register
pub mod otg_hs_diepmsk;
///OTG_HS device OUT endpoint common interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepmsk](otg_hs_doepmsk) module
pub type OTG_HS_DOEPMSK = crate::Reg<u32, _OTG_HS_DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPMSK;
///`read()` method returns [otg_hs_doepmsk::R](otg_hs_doepmsk::R) reader structure
impl crate::Readable for OTG_HS_DOEPMSK {}
///`write(|w| ..)` method takes [otg_hs_doepmsk::W](otg_hs_doepmsk::W) writer structure
impl crate::Writable for OTG_HS_DOEPMSK {}
///OTG_HS device OUT endpoint common interrupt mask register
pub mod otg_hs_doepmsk;
///OTG_HS device all endpoints interrupt register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_daint](otg_hs_daint) module
pub type OTG_HS_DAINT = crate::Reg<u32, _OTG_HS_DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DAINT;
///`read()` method returns [otg_hs_daint::R](otg_hs_daint::R) reader structure
impl crate::Readable for OTG_HS_DAINT {}
///OTG_HS device all endpoints interrupt register
pub mod otg_hs_daint;
///OTG_HS all endpoints interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_daintmsk](otg_hs_daintmsk) module
pub type OTG_HS_DAINTMSK = crate::Reg<u32, _OTG_HS_DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DAINTMSK;
///`read()` method returns [otg_hs_daintmsk::R](otg_hs_daintmsk::R) reader structure
impl crate::Readable for OTG_HS_DAINTMSK {}
///`write(|w| ..)` method takes [otg_hs_daintmsk::W](otg_hs_daintmsk::W) writer structure
impl crate::Writable for OTG_HS_DAINTMSK {}
///OTG_HS all endpoints interrupt mask register
pub mod otg_hs_daintmsk;
///OTG_HS device VBUS discharge time register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dvbusdis](otg_hs_dvbusdis) module
pub type OTG_HS_DVBUSDIS = crate::Reg<u32, _OTG_HS_DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DVBUSDIS;
///`read()` method returns [otg_hs_dvbusdis::R](otg_hs_dvbusdis::R) reader structure
impl crate::Readable for OTG_HS_DVBUSDIS {}
///`write(|w| ..)` method takes [otg_hs_dvbusdis::W](otg_hs_dvbusdis::W) writer structure
impl crate::Writable for OTG_HS_DVBUSDIS {}
///OTG_HS device VBUS discharge time register
pub mod otg_hs_dvbusdis;
///OTG_HS device VBUS pulsing time register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dvbuspulse](otg_hs_dvbuspulse) module
pub type OTG_HS_DVBUSPULSE = crate::Reg<u32, _OTG_HS_DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DVBUSPULSE;
///`read()` method returns [otg_hs_dvbuspulse::R](otg_hs_dvbuspulse::R) reader structure
impl crate::Readable for OTG_HS_DVBUSPULSE {}
///`write(|w| ..)` method takes [otg_hs_dvbuspulse::W](otg_hs_dvbuspulse::W) writer structure
impl crate::Writable for OTG_HS_DVBUSPULSE {}
///OTG_HS device VBUS pulsing time register
pub mod otg_hs_dvbuspulse;
///OTG_HS Device threshold control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dthrctl](otg_hs_dthrctl) module
pub type OTG_HS_DTHRCTL = crate::Reg<u32, _OTG_HS_DTHRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTHRCTL;
///`read()` method returns [otg_hs_dthrctl::R](otg_hs_dthrctl::R) reader structure
impl crate::Readable for OTG_HS_DTHRCTL {}
///`write(|w| ..)` method takes [otg_hs_dthrctl::W](otg_hs_dthrctl::W) writer structure
impl crate::Writable for OTG_HS_DTHRCTL {}
///OTG_HS Device threshold control register
pub mod otg_hs_dthrctl;
///OTG_HS device IN endpoint FIFO empty interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepempmsk](otg_hs_diepempmsk) module
pub type OTG_HS_DIEPEMPMSK = crate::Reg<u32, _OTG_HS_DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPEMPMSK;
///`read()` method returns [otg_hs_diepempmsk::R](otg_hs_diepempmsk::R) reader structure
impl crate::Readable for OTG_HS_DIEPEMPMSK {}
///`write(|w| ..)` method takes [otg_hs_diepempmsk::W](otg_hs_diepempmsk::W) writer structure
impl crate::Writable for OTG_HS_DIEPEMPMSK {}
///OTG_HS device IN endpoint FIFO empty interrupt mask register
pub mod otg_hs_diepempmsk;
///OTG_HS device each endpoint interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_deachint](otg_hs_deachint) module
pub type OTG_HS_DEACHINT = crate::Reg<u32, _OTG_HS_DEACHINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DEACHINT;
///`read()` method returns [otg_hs_deachint::R](otg_hs_deachint::R) reader structure
impl crate::Readable for OTG_HS_DEACHINT {}
///`write(|w| ..)` method takes [otg_hs_deachint::W](otg_hs_deachint::W) writer structure
impl crate::Writable for OTG_HS_DEACHINT {}
///OTG_HS device each endpoint interrupt register
pub mod otg_hs_deachint;
///OTG_HS device each endpoint interrupt register mask
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_deachintmsk](otg_hs_deachintmsk) module
pub type OTG_HS_DEACHINTMSK = crate::Reg<u32, _OTG_HS_DEACHINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DEACHINTMSK;
///`read()` method returns [otg_hs_deachintmsk::R](otg_hs_deachintmsk::R) reader structure
impl crate::Readable for OTG_HS_DEACHINTMSK {}
///`write(|w| ..)` method takes [otg_hs_deachintmsk::W](otg_hs_deachintmsk::W) writer structure
impl crate::Writable for OTG_HS_DEACHINTMSK {}
///OTG_HS device each endpoint interrupt register mask
pub mod otg_hs_deachintmsk;
///OTG device endpoint-0 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl0](otg_hs_diepctl0) module
pub type OTG_HS_DIEPCTL0 = crate::Reg<u32, _OTG_HS_DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL0;
///`read()` method returns [otg_hs_diepctl0::R](otg_hs_diepctl0::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL0 {}
///`write(|w| ..)` method takes [otg_hs_diepctl0::W](otg_hs_diepctl0::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL0 {}
///OTG device endpoint-0 control register
pub mod otg_hs_diepctl0;
///OTG device endpoint-1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl1](otg_hs_diepctl1) module
pub type OTG_HS_DIEPCTL1 = crate::Reg<u32, _OTG_HS_DIEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL1;
///`read()` method returns [otg_hs_diepctl1::R](otg_hs_diepctl1::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL1 {}
///`write(|w| ..)` method takes [otg_hs_diepctl1::W](otg_hs_diepctl1::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL1 {}
///OTG device endpoint-1 control register
pub mod otg_hs_diepctl1;
///OTG device endpoint-2 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl2](otg_hs_diepctl2) module
pub type OTG_HS_DIEPCTL2 = crate::Reg<u32, _OTG_HS_DIEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL2;
///`read()` method returns [otg_hs_diepctl2::R](otg_hs_diepctl2::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL2 {}
///`write(|w| ..)` method takes [otg_hs_diepctl2::W](otg_hs_diepctl2::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL2 {}
///OTG device endpoint-2 control register
pub mod otg_hs_diepctl2;
///OTG device endpoint-3 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl3](otg_hs_diepctl3) module
pub type OTG_HS_DIEPCTL3 = crate::Reg<u32, _OTG_HS_DIEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL3;
///`read()` method returns [otg_hs_diepctl3::R](otg_hs_diepctl3::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL3 {}
///`write(|w| ..)` method takes [otg_hs_diepctl3::W](otg_hs_diepctl3::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL3 {}
///OTG device endpoint-3 control register
pub mod otg_hs_diepctl3;
///OTG device endpoint-4 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl4](otg_hs_diepctl4) module
pub type OTG_HS_DIEPCTL4 = crate::Reg<u32, _OTG_HS_DIEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL4;
///`read()` method returns [otg_hs_diepctl4::R](otg_hs_diepctl4::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL4 {}
///`write(|w| ..)` method takes [otg_hs_diepctl4::W](otg_hs_diepctl4::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL4 {}
///OTG device endpoint-4 control register
pub mod otg_hs_diepctl4;
///OTG device endpoint-5 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl5](otg_hs_diepctl5) module
pub type OTG_HS_DIEPCTL5 = crate::Reg<u32, _OTG_HS_DIEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL5;
///`read()` method returns [otg_hs_diepctl5::R](otg_hs_diepctl5::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL5 {}
///`write(|w| ..)` method takes [otg_hs_diepctl5::W](otg_hs_diepctl5::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL5 {}
///OTG device endpoint-5 control register
pub mod otg_hs_diepctl5;
///OTG device endpoint-6 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl6](otg_hs_diepctl6) module
pub type OTG_HS_DIEPCTL6 = crate::Reg<u32, _OTG_HS_DIEPCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL6;
///`read()` method returns [otg_hs_diepctl6::R](otg_hs_diepctl6::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL6 {}
///`write(|w| ..)` method takes [otg_hs_diepctl6::W](otg_hs_diepctl6::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL6 {}
///OTG device endpoint-6 control register
pub mod otg_hs_diepctl6;
///OTG device endpoint-7 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepctl7](otg_hs_diepctl7) module
pub type OTG_HS_DIEPCTL7 = crate::Reg<u32, _OTG_HS_DIEPCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPCTL7;
///`read()` method returns [otg_hs_diepctl7::R](otg_hs_diepctl7::R) reader structure
impl crate::Readable for OTG_HS_DIEPCTL7 {}
///`write(|w| ..)` method takes [otg_hs_diepctl7::W](otg_hs_diepctl7::W) writer structure
impl crate::Writable for OTG_HS_DIEPCTL7 {}
///OTG device endpoint-7 control register
pub mod otg_hs_diepctl7;
///OTG device endpoint-0 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint0](otg_hs_diepint0) module
pub type OTG_HS_DIEPINT0 = crate::Reg<u32, _OTG_HS_DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT0;
///`read()` method returns [otg_hs_diepint0::R](otg_hs_diepint0::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT0 {}
///`write(|w| ..)` method takes [otg_hs_diepint0::W](otg_hs_diepint0::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT0 {}
///OTG device endpoint-0 interrupt register
pub mod otg_hs_diepint0;
///OTG device endpoint-1 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint1](otg_hs_diepint1) module
pub type OTG_HS_DIEPINT1 = crate::Reg<u32, _OTG_HS_DIEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT1;
///`read()` method returns [otg_hs_diepint1::R](otg_hs_diepint1::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT1 {}
///`write(|w| ..)` method takes [otg_hs_diepint1::W](otg_hs_diepint1::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT1 {}
///OTG device endpoint-1 interrupt register
pub mod otg_hs_diepint1;
///OTG device endpoint-2 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint2](otg_hs_diepint2) module
pub type OTG_HS_DIEPINT2 = crate::Reg<u32, _OTG_HS_DIEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT2;
///`read()` method returns [otg_hs_diepint2::R](otg_hs_diepint2::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT2 {}
///`write(|w| ..)` method takes [otg_hs_diepint2::W](otg_hs_diepint2::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT2 {}
///OTG device endpoint-2 interrupt register
pub mod otg_hs_diepint2;
///OTG device endpoint-3 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint3](otg_hs_diepint3) module
pub type OTG_HS_DIEPINT3 = crate::Reg<u32, _OTG_HS_DIEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT3;
///`read()` method returns [otg_hs_diepint3::R](otg_hs_diepint3::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT3 {}
///`write(|w| ..)` method takes [otg_hs_diepint3::W](otg_hs_diepint3::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT3 {}
///OTG device endpoint-3 interrupt register
pub mod otg_hs_diepint3;
///OTG device endpoint-4 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint4](otg_hs_diepint4) module
pub type OTG_HS_DIEPINT4 = crate::Reg<u32, _OTG_HS_DIEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT4;
///`read()` method returns [otg_hs_diepint4::R](otg_hs_diepint4::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT4 {}
///`write(|w| ..)` method takes [otg_hs_diepint4::W](otg_hs_diepint4::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT4 {}
///OTG device endpoint-4 interrupt register
pub mod otg_hs_diepint4;
///OTG device endpoint-5 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint5](otg_hs_diepint5) module
pub type OTG_HS_DIEPINT5 = crate::Reg<u32, _OTG_HS_DIEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT5;
///`read()` method returns [otg_hs_diepint5::R](otg_hs_diepint5::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT5 {}
///`write(|w| ..)` method takes [otg_hs_diepint5::W](otg_hs_diepint5::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT5 {}
///OTG device endpoint-5 interrupt register
pub mod otg_hs_diepint5;
///OTG device endpoint-6 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint6](otg_hs_diepint6) module
pub type OTG_HS_DIEPINT6 = crate::Reg<u32, _OTG_HS_DIEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT6;
///`read()` method returns [otg_hs_diepint6::R](otg_hs_diepint6::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT6 {}
///`write(|w| ..)` method takes [otg_hs_diepint6::W](otg_hs_diepint6::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT6 {}
///OTG device endpoint-6 interrupt register
pub mod otg_hs_diepint6;
///OTG device endpoint-7 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepint7](otg_hs_diepint7) module
pub type OTG_HS_DIEPINT7 = crate::Reg<u32, _OTG_HS_DIEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPINT7;
///`read()` method returns [otg_hs_diepint7::R](otg_hs_diepint7::R) reader structure
impl crate::Readable for OTG_HS_DIEPINT7 {}
///`write(|w| ..)` method takes [otg_hs_diepint7::W](otg_hs_diepint7::W) writer structure
impl crate::Writable for OTG_HS_DIEPINT7 {}
///OTG device endpoint-7 interrupt register
pub mod otg_hs_diepint7;
///OTG_HS device IN endpoint 0 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz0](otg_hs_dieptsiz0) module
pub type OTG_HS_DIEPTSIZ0 = crate::Reg<u32, _OTG_HS_DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ0;
///`read()` method returns [otg_hs_dieptsiz0::R](otg_hs_dieptsiz0::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ0 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz0::W](otg_hs_dieptsiz0::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ0 {}
///OTG_HS device IN endpoint 0 transfer size register
pub mod otg_hs_dieptsiz0;
///OTG_HS device endpoint-1 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepdma1](otg_hs_diepdma1) module
pub type OTG_HS_DIEPDMA1 = crate::Reg<u32, _OTG_HS_DIEPDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA1;
///`read()` method returns [otg_hs_diepdma1::R](otg_hs_diepdma1::R) reader structure
impl crate::Readable for OTG_HS_DIEPDMA1 {}
///`write(|w| ..)` method takes [otg_hs_diepdma1::W](otg_hs_diepdma1::W) writer structure
impl crate::Writable for OTG_HS_DIEPDMA1 {}
///OTG_HS device endpoint-1 DMA address register
pub mod otg_hs_diepdma1;
///OTG_HS device endpoint-2 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepdma2](otg_hs_diepdma2) module
pub type OTG_HS_DIEPDMA2 = crate::Reg<u32, _OTG_HS_DIEPDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA2;
///`read()` method returns [otg_hs_diepdma2::R](otg_hs_diepdma2::R) reader structure
impl crate::Readable for OTG_HS_DIEPDMA2 {}
///`write(|w| ..)` method takes [otg_hs_diepdma2::W](otg_hs_diepdma2::W) writer structure
impl crate::Writable for OTG_HS_DIEPDMA2 {}
///OTG_HS device endpoint-2 DMA address register
pub mod otg_hs_diepdma2;
///OTG_HS device endpoint-3 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepdma3](otg_hs_diepdma3) module
pub type OTG_HS_DIEPDMA3 = crate::Reg<u32, _OTG_HS_DIEPDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA3;
///`read()` method returns [otg_hs_diepdma3::R](otg_hs_diepdma3::R) reader structure
impl crate::Readable for OTG_HS_DIEPDMA3 {}
///`write(|w| ..)` method takes [otg_hs_diepdma3::W](otg_hs_diepdma3::W) writer structure
impl crate::Writable for OTG_HS_DIEPDMA3 {}
///OTG_HS device endpoint-3 DMA address register
pub mod otg_hs_diepdma3;
///OTG_HS device endpoint-4 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepdma4](otg_hs_diepdma4) module
pub type OTG_HS_DIEPDMA4 = crate::Reg<u32, _OTG_HS_DIEPDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA4;
///`read()` method returns [otg_hs_diepdma4::R](otg_hs_diepdma4::R) reader structure
impl crate::Readable for OTG_HS_DIEPDMA4 {}
///`write(|w| ..)` method takes [otg_hs_diepdma4::W](otg_hs_diepdma4::W) writer structure
impl crate::Writable for OTG_HS_DIEPDMA4 {}
///OTG_HS device endpoint-4 DMA address register
pub mod otg_hs_diepdma4;
///OTG_HS device endpoint-5 DMA address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_diepdma5](otg_hs_diepdma5) module
pub type OTG_HS_DIEPDMA5 = crate::Reg<u32, _OTG_HS_DIEPDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPDMA5;
///`read()` method returns [otg_hs_diepdma5::R](otg_hs_diepdma5::R) reader structure
impl crate::Readable for OTG_HS_DIEPDMA5 {}
///`write(|w| ..)` method takes [otg_hs_diepdma5::W](otg_hs_diepdma5::W) writer structure
impl crate::Writable for OTG_HS_DIEPDMA5 {}
///OTG_HS device endpoint-5 DMA address register
pub mod otg_hs_diepdma5;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts0](otg_hs_dtxfsts0) module
pub type OTG_HS_DTXFSTS0 = crate::Reg<u32, _OTG_HS_DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS0;
///`read()` method returns [otg_hs_dtxfsts0::R](otg_hs_dtxfsts0::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS0 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts0;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts1](otg_hs_dtxfsts1) module
pub type OTG_HS_DTXFSTS1 = crate::Reg<u32, _OTG_HS_DTXFSTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS1;
///`read()` method returns [otg_hs_dtxfsts1::R](otg_hs_dtxfsts1::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS1 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts1;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts2](otg_hs_dtxfsts2) module
pub type OTG_HS_DTXFSTS2 = crate::Reg<u32, _OTG_HS_DTXFSTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS2;
///`read()` method returns [otg_hs_dtxfsts2::R](otg_hs_dtxfsts2::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS2 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts2;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts3](otg_hs_dtxfsts3) module
pub type OTG_HS_DTXFSTS3 = crate::Reg<u32, _OTG_HS_DTXFSTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS3;
///`read()` method returns [otg_hs_dtxfsts3::R](otg_hs_dtxfsts3::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS3 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts3;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts4](otg_hs_dtxfsts4) module
pub type OTG_HS_DTXFSTS4 = crate::Reg<u32, _OTG_HS_DTXFSTS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS4;
///`read()` method returns [otg_hs_dtxfsts4::R](otg_hs_dtxfsts4::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS4 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts4;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts5](otg_hs_dtxfsts5) module
pub type OTG_HS_DTXFSTS5 = crate::Reg<u32, _OTG_HS_DTXFSTS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS5;
///`read()` method returns [otg_hs_dtxfsts5::R](otg_hs_dtxfsts5::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS5 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts5;
///OTG_HS device endpoint transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz1](otg_hs_dieptsiz1) module
pub type OTG_HS_DIEPTSIZ1 = crate::Reg<u32, _OTG_HS_DIEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ1;
///`read()` method returns [otg_hs_dieptsiz1::R](otg_hs_dieptsiz1::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ1 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz1::W](otg_hs_dieptsiz1::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ1 {}
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz1;
///OTG_HS device endpoint transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz2](otg_hs_dieptsiz2) module
pub type OTG_HS_DIEPTSIZ2 = crate::Reg<u32, _OTG_HS_DIEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ2;
///`read()` method returns [otg_hs_dieptsiz2::R](otg_hs_dieptsiz2::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ2 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz2::W](otg_hs_dieptsiz2::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ2 {}
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz2;
///OTG_HS device endpoint transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz3](otg_hs_dieptsiz3) module
pub type OTG_HS_DIEPTSIZ3 = crate::Reg<u32, _OTG_HS_DIEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ3;
///`read()` method returns [otg_hs_dieptsiz3::R](otg_hs_dieptsiz3::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ3 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz3::W](otg_hs_dieptsiz3::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ3 {}
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz3;
///OTG_HS device endpoint transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz4](otg_hs_dieptsiz4) module
pub type OTG_HS_DIEPTSIZ4 = crate::Reg<u32, _OTG_HS_DIEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ4;
///`read()` method returns [otg_hs_dieptsiz4::R](otg_hs_dieptsiz4::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ4 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz4::W](otg_hs_dieptsiz4::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ4 {}
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz4;
///OTG_HS device endpoint transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz5](otg_hs_dieptsiz5) module
pub type OTG_HS_DIEPTSIZ5 = crate::Reg<u32, _OTG_HS_DIEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ5;
///`read()` method returns [otg_hs_dieptsiz5::R](otg_hs_dieptsiz5::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ5 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz5::W](otg_hs_dieptsiz5::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ5 {}
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz5;
///OTG_HS device control OUT endpoint 0 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl0](otg_hs_doepctl0) module
pub type OTG_HS_DOEPCTL0 = crate::Reg<u32, _OTG_HS_DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL0;
///`read()` method returns [otg_hs_doepctl0::R](otg_hs_doepctl0::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL0 {}
///`write(|w| ..)` method takes [otg_hs_doepctl0::W](otg_hs_doepctl0::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL0 {}
///OTG_HS device control OUT endpoint 0 control register
pub mod otg_hs_doepctl0;
///OTG device endpoint-1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl1](otg_hs_doepctl1) module
pub type OTG_HS_DOEPCTL1 = crate::Reg<u32, _OTG_HS_DOEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL1;
///`read()` method returns [otg_hs_doepctl1::R](otg_hs_doepctl1::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL1 {}
///`write(|w| ..)` method takes [otg_hs_doepctl1::W](otg_hs_doepctl1::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL1 {}
///OTG device endpoint-1 control register
pub mod otg_hs_doepctl1;
///OTG device endpoint-2 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl2](otg_hs_doepctl2) module
pub type OTG_HS_DOEPCTL2 = crate::Reg<u32, _OTG_HS_DOEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL2;
///`read()` method returns [otg_hs_doepctl2::R](otg_hs_doepctl2::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL2 {}
///`write(|w| ..)` method takes [otg_hs_doepctl2::W](otg_hs_doepctl2::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL2 {}
///OTG device endpoint-2 control register
pub mod otg_hs_doepctl2;
///OTG device endpoint-3 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl3](otg_hs_doepctl3) module
pub type OTG_HS_DOEPCTL3 = crate::Reg<u32, _OTG_HS_DOEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL3;
///`read()` method returns [otg_hs_doepctl3::R](otg_hs_doepctl3::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL3 {}
///`write(|w| ..)` method takes [otg_hs_doepctl3::W](otg_hs_doepctl3::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL3 {}
///OTG device endpoint-3 control register
pub mod otg_hs_doepctl3;
///OTG_HS device endpoint-0 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint0](otg_hs_doepint0) module
pub type OTG_HS_DOEPINT0 = crate::Reg<u32, _OTG_HS_DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT0;
///`read()` method returns [otg_hs_doepint0::R](otg_hs_doepint0::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT0 {}
///`write(|w| ..)` method takes [otg_hs_doepint0::W](otg_hs_doepint0::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT0 {}
///OTG_HS device endpoint-0 interrupt register
pub mod otg_hs_doepint0;
///OTG_HS device endpoint-1 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint1](otg_hs_doepint1) module
pub type OTG_HS_DOEPINT1 = crate::Reg<u32, _OTG_HS_DOEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT1;
///`read()` method returns [otg_hs_doepint1::R](otg_hs_doepint1::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT1 {}
///`write(|w| ..)` method takes [otg_hs_doepint1::W](otg_hs_doepint1::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT1 {}
///OTG_HS device endpoint-1 interrupt register
pub mod otg_hs_doepint1;
///OTG_HS device endpoint-2 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint2](otg_hs_doepint2) module
pub type OTG_HS_DOEPINT2 = crate::Reg<u32, _OTG_HS_DOEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT2;
///`read()` method returns [otg_hs_doepint2::R](otg_hs_doepint2::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT2 {}
///`write(|w| ..)` method takes [otg_hs_doepint2::W](otg_hs_doepint2::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT2 {}
///OTG_HS device endpoint-2 interrupt register
pub mod otg_hs_doepint2;
///OTG_HS device endpoint-3 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint3](otg_hs_doepint3) module
pub type OTG_HS_DOEPINT3 = crate::Reg<u32, _OTG_HS_DOEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT3;
///`read()` method returns [otg_hs_doepint3::R](otg_hs_doepint3::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT3 {}
///`write(|w| ..)` method takes [otg_hs_doepint3::W](otg_hs_doepint3::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT3 {}
///OTG_HS device endpoint-3 interrupt register
pub mod otg_hs_doepint3;
///OTG_HS device endpoint-4 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint4](otg_hs_doepint4) module
pub type OTG_HS_DOEPINT4 = crate::Reg<u32, _OTG_HS_DOEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT4;
///`read()` method returns [otg_hs_doepint4::R](otg_hs_doepint4::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT4 {}
///`write(|w| ..)` method takes [otg_hs_doepint4::W](otg_hs_doepint4::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT4 {}
///OTG_HS device endpoint-4 interrupt register
pub mod otg_hs_doepint4;
///OTG_HS device endpoint-5 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint5](otg_hs_doepint5) module
pub type OTG_HS_DOEPINT5 = crate::Reg<u32, _OTG_HS_DOEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT5;
///`read()` method returns [otg_hs_doepint5::R](otg_hs_doepint5::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT5 {}
///`write(|w| ..)` method takes [otg_hs_doepint5::W](otg_hs_doepint5::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT5 {}
///OTG_HS device endpoint-5 interrupt register
pub mod otg_hs_doepint5;
///OTG_HS device endpoint-6 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint6](otg_hs_doepint6) module
pub type OTG_HS_DOEPINT6 = crate::Reg<u32, _OTG_HS_DOEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT6;
///`read()` method returns [otg_hs_doepint6::R](otg_hs_doepint6::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT6 {}
///`write(|w| ..)` method takes [otg_hs_doepint6::W](otg_hs_doepint6::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT6 {}
///OTG_HS device endpoint-6 interrupt register
pub mod otg_hs_doepint6;
///OTG_HS device endpoint-7 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepint7](otg_hs_doepint7) module
pub type OTG_HS_DOEPINT7 = crate::Reg<u32, _OTG_HS_DOEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPINT7;
///`read()` method returns [otg_hs_doepint7::R](otg_hs_doepint7::R) reader structure
impl crate::Readable for OTG_HS_DOEPINT7 {}
///`write(|w| ..)` method takes [otg_hs_doepint7::W](otg_hs_doepint7::W) writer structure
impl crate::Writable for OTG_HS_DOEPINT7 {}
///OTG_HS device endpoint-7 interrupt register
pub mod otg_hs_doepint7;
///OTG_HS device endpoint-0 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz0](otg_hs_doeptsiz0) module
pub type OTG_HS_DOEPTSIZ0 = crate::Reg<u32, _OTG_HS_DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ0;
///`read()` method returns [otg_hs_doeptsiz0::R](otg_hs_doeptsiz0::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ0 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz0::W](otg_hs_doeptsiz0::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ0 {}
///OTG_HS device endpoint-0 transfer size register
pub mod otg_hs_doeptsiz0;
///OTG_HS device endpoint-1 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz1](otg_hs_doeptsiz1) module
pub type OTG_HS_DOEPTSIZ1 = crate::Reg<u32, _OTG_HS_DOEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ1;
///`read()` method returns [otg_hs_doeptsiz1::R](otg_hs_doeptsiz1::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ1 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz1::W](otg_hs_doeptsiz1::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ1 {}
///OTG_HS device endpoint-1 transfer size register
pub mod otg_hs_doeptsiz1;
///OTG_HS device endpoint-2 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz2](otg_hs_doeptsiz2) module
pub type OTG_HS_DOEPTSIZ2 = crate::Reg<u32, _OTG_HS_DOEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ2;
///`read()` method returns [otg_hs_doeptsiz2::R](otg_hs_doeptsiz2::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ2 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz2::W](otg_hs_doeptsiz2::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ2 {}
///OTG_HS device endpoint-2 transfer size register
pub mod otg_hs_doeptsiz2;
///OTG_HS device endpoint-3 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz3](otg_hs_doeptsiz3) module
pub type OTG_HS_DOEPTSIZ3 = crate::Reg<u32, _OTG_HS_DOEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ3;
///`read()` method returns [otg_hs_doeptsiz3::R](otg_hs_doeptsiz3::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ3 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz3::W](otg_hs_doeptsiz3::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ3 {}
///OTG_HS device endpoint-3 transfer size register
pub mod otg_hs_doeptsiz3;
///OTG_HS device endpoint-4 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz4](otg_hs_doeptsiz4) module
pub type OTG_HS_DOEPTSIZ4 = crate::Reg<u32, _OTG_HS_DOEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ4;
///`read()` method returns [otg_hs_doeptsiz4::R](otg_hs_doeptsiz4::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ4 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz4::W](otg_hs_doeptsiz4::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ4 {}
///OTG_HS device endpoint-4 transfer size register
pub mod otg_hs_doeptsiz4;
///OTG_HS device endpoint transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz6](otg_hs_dieptsiz6) module
pub type OTG_HS_DIEPTSIZ6 = crate::Reg<u32, _OTG_HS_DIEPTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ6;
///`read()` method returns [otg_hs_dieptsiz6::R](otg_hs_dieptsiz6::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ6 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz6::W](otg_hs_dieptsiz6::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ6 {}
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz6;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts6](otg_hs_dtxfsts6) module
pub type OTG_HS_DTXFSTS6 = crate::Reg<u32, _OTG_HS_DTXFSTS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS6;
///`read()` method returns [otg_hs_dtxfsts6::R](otg_hs_dtxfsts6::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS6 {}
///`write(|w| ..)` method takes [otg_hs_dtxfsts6::W](otg_hs_dtxfsts6::W) writer structure
impl crate::Writable for OTG_HS_DTXFSTS6 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts6;
///OTG_HS device endpoint transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dieptsiz7](otg_hs_dieptsiz7) module
pub type OTG_HS_DIEPTSIZ7 = crate::Reg<u32, _OTG_HS_DIEPTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTSIZ7;
///`read()` method returns [otg_hs_dieptsiz7::R](otg_hs_dieptsiz7::R) reader structure
impl crate::Readable for OTG_HS_DIEPTSIZ7 {}
///`write(|w| ..)` method takes [otg_hs_dieptsiz7::W](otg_hs_dieptsiz7::W) writer structure
impl crate::Writable for OTG_HS_DIEPTSIZ7 {}
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz7;
///OTG_HS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_dtxfsts7](otg_hs_dtxfsts7) module
pub type OTG_HS_DTXFSTS7 = crate::Reg<u32, _OTG_HS_DTXFSTS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DTXFSTS7;
///`read()` method returns [otg_hs_dtxfsts7::R](otg_hs_dtxfsts7::R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS7 {}
///`write(|w| ..)` method takes [otg_hs_dtxfsts7::W](otg_hs_dtxfsts7::W) writer structure
impl crate::Writable for OTG_HS_DTXFSTS7 {}
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts7;
///OTG device endpoint-4 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl4](otg_hs_doepctl4) module
pub type OTG_HS_DOEPCTL4 = crate::Reg<u32, _OTG_HS_DOEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL4;
///`read()` method returns [otg_hs_doepctl4::R](otg_hs_doepctl4::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL4 {}
///`write(|w| ..)` method takes [otg_hs_doepctl4::W](otg_hs_doepctl4::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL4 {}
///OTG device endpoint-4 control register
pub mod otg_hs_doepctl4;
///OTG device endpoint-5 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl5](otg_hs_doepctl5) module
pub type OTG_HS_DOEPCTL5 = crate::Reg<u32, _OTG_HS_DOEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL5;
///`read()` method returns [otg_hs_doepctl5::R](otg_hs_doepctl5::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL5 {}
///`write(|w| ..)` method takes [otg_hs_doepctl5::W](otg_hs_doepctl5::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL5 {}
///OTG device endpoint-5 control register
pub mod otg_hs_doepctl5;
///OTG device endpoint-6 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl6](otg_hs_doepctl6) module
pub type OTG_HS_DOEPCTL6 = crate::Reg<u32, _OTG_HS_DOEPCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL6;
///`read()` method returns [otg_hs_doepctl6::R](otg_hs_doepctl6::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL6 {}
///`write(|w| ..)` method takes [otg_hs_doepctl6::W](otg_hs_doepctl6::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL6 {}
///OTG device endpoint-6 control register
pub mod otg_hs_doepctl6;
///OTG device endpoint-7 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doepctl7](otg_hs_doepctl7) module
pub type OTG_HS_DOEPCTL7 = crate::Reg<u32, _OTG_HS_DOEPCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPCTL7;
///`read()` method returns [otg_hs_doepctl7::R](otg_hs_doepctl7::R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL7 {}
///`write(|w| ..)` method takes [otg_hs_doepctl7::W](otg_hs_doepctl7::W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL7 {}
///OTG device endpoint-7 control register
pub mod otg_hs_doepctl7;
///OTG_HS device endpoint-5 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz5](otg_hs_doeptsiz5) module
pub type OTG_HS_DOEPTSIZ5 = crate::Reg<u32, _OTG_HS_DOEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ5;
///`read()` method returns [otg_hs_doeptsiz5::R](otg_hs_doeptsiz5::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ5 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz5::W](otg_hs_doeptsiz5::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ5 {}
///OTG_HS device endpoint-5 transfer size register
pub mod otg_hs_doeptsiz5;
///OTG_HS device endpoint-6 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz6](otg_hs_doeptsiz6) module
pub type OTG_HS_DOEPTSIZ6 = crate::Reg<u32, _OTG_HS_DOEPTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ6;
///`read()` method returns [otg_hs_doeptsiz6::R](otg_hs_doeptsiz6::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ6 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz6::W](otg_hs_doeptsiz6::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ6 {}
///OTG_HS device endpoint-6 transfer size register
pub mod otg_hs_doeptsiz6;
///OTG_HS device endpoint-7 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hs_doeptsiz7](otg_hs_doeptsiz7) module
pub type OTG_HS_DOEPTSIZ7 = crate::Reg<u32, _OTG_HS_DOEPTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPTSIZ7;
///`read()` method returns [otg_hs_doeptsiz7::R](otg_hs_doeptsiz7::R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ7 {}
///`write(|w| ..)` method takes [otg_hs_doeptsiz7::W](otg_hs_doeptsiz7::W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ7 {}
///OTG_HS device endpoint-7 transfer size register
pub mod otg_hs_doeptsiz7;
