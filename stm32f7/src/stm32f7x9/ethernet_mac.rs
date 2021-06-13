///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Ethernet MAC configuration register
    pub maccr: MACCR,
    ///0x04 - Ethernet MAC frame filter register
    pub macffr: MACFFR,
    ///0x08 - Ethernet MAC hash table high register
    pub machthr: MACHTHR,
    ///0x0c - Ethernet MAC hash table low register
    pub machtlr: MACHTLR,
    ///0x10 - Ethernet MAC MII address register
    pub macmiiar: MACMIIAR,
    ///0x14 - Ethernet MAC MII data register
    pub macmiidr: MACMIIDR,
    ///0x18 - Ethernet MAC flow control register
    pub macfcr: MACFCR,
    ///0x1c - Ethernet MAC VLAN tag register
    pub macvlantr: MACVLANTR,
    _reserved8: [u8; 12usize],
    ///0x2c - Ethernet MAC PMT control and status register
    pub macpmtcsr: MACPMTCSR,
    _reserved9: [u8; 4usize],
    ///0x34 - Ethernet MAC debug register
    pub macdbgr: MACDBGR,
    ///0x38 - Ethernet MAC interrupt status register
    pub macsr: MACSR,
    ///0x3c - Ethernet MAC interrupt mask register
    pub macimr: MACIMR,
    ///0x40 - Ethernet MAC address 0 high register
    pub maca0hr: MACA0HR,
    ///0x44 - Ethernet MAC address 0 low register
    pub maca0lr: MACA0LR,
    ///0x48 - Ethernet MAC address 1 high register
    pub maca1hr: MACA1HR,
    ///0x4c - Ethernet MAC address1 low register
    pub maca1lr: MACA1LR,
    ///0x50 - Ethernet MAC address 2 high register
    pub maca2hr: MACA2HR,
    ///0x54 - Ethernet MAC address 2 low register
    pub maca2lr: MACA2LR,
    ///0x58 - Ethernet MAC address 3 high register
    pub maca3hr: MACA3HR,
    ///0x5c - Ethernet MAC address 3 low register
    pub maca3lr: MACA3LR,
    ///0x60 - Ethernet MAC remote wakeup frame filter register
    pub macrwuffer: MACRWUFFER,
}
///Ethernet MAC configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maccr](maccr) module
pub type MACCR = crate::Reg<u32, _MACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACCR;
///`read()` method returns [maccr::R](maccr::R) reader structure
impl crate::Readable for MACCR {}
///`write(|w| ..)` method takes [maccr::W](maccr::W) writer structure
impl crate::Writable for MACCR {}
///Ethernet MAC configuration register
pub mod maccr;
///Ethernet MAC frame filter register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macffr](macffr) module
pub type MACFFR = crate::Reg<u32, _MACFFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACFFR;
///`read()` method returns [macffr::R](macffr::R) reader structure
impl crate::Readable for MACFFR {}
///`write(|w| ..)` method takes [macffr::W](macffr::W) writer structure
impl crate::Writable for MACFFR {}
///Ethernet MAC frame filter register
pub mod macffr;
///Ethernet MAC hash table high register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [machthr](machthr) module
pub type MACHTHR = crate::Reg<u32, _MACHTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHTHR;
///`read()` method returns [machthr::R](machthr::R) reader structure
impl crate::Readable for MACHTHR {}
///`write(|w| ..)` method takes [machthr::W](machthr::W) writer structure
impl crate::Writable for MACHTHR {}
///Ethernet MAC hash table high register
pub mod machthr;
///Ethernet MAC hash table low register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [machtlr](machtlr) module
pub type MACHTLR = crate::Reg<u32, _MACHTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHTLR;
///`read()` method returns [machtlr::R](machtlr::R) reader structure
impl crate::Readable for MACHTLR {}
///`write(|w| ..)` method takes [machtlr::W](machtlr::W) writer structure
impl crate::Writable for MACHTLR {}
///Ethernet MAC hash table low register
pub mod machtlr;
///Ethernet MAC MII address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macmiiar](macmiiar) module
pub type MACMIIAR = crate::Reg<u32, _MACMIIAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACMIIAR;
///`read()` method returns [macmiiar::R](macmiiar::R) reader structure
impl crate::Readable for MACMIIAR {}
///`write(|w| ..)` method takes [macmiiar::W](macmiiar::W) writer structure
impl crate::Writable for MACMIIAR {}
///Ethernet MAC MII address register
pub mod macmiiar;
///Ethernet MAC MII data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macmiidr](macmiidr) module
pub type MACMIIDR = crate::Reg<u32, _MACMIIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACMIIDR;
///`read()` method returns [macmiidr::R](macmiidr::R) reader structure
impl crate::Readable for MACMIIDR {}
///`write(|w| ..)` method takes [macmiidr::W](macmiidr::W) writer structure
impl crate::Writable for MACMIIDR {}
///Ethernet MAC MII data register
pub mod macmiidr;
///Ethernet MAC flow control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macfcr](macfcr) module
pub type MACFCR = crate::Reg<u32, _MACFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACFCR;
///`read()` method returns [macfcr::R](macfcr::R) reader structure
impl crate::Readable for MACFCR {}
///`write(|w| ..)` method takes [macfcr::W](macfcr::W) writer structure
impl crate::Writable for MACFCR {}
///Ethernet MAC flow control register
pub mod macfcr;
///Ethernet MAC VLAN tag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macvlantr](macvlantr) module
pub type MACVLANTR = crate::Reg<u32, _MACVLANTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACVLANTR;
///`read()` method returns [macvlantr::R](macvlantr::R) reader structure
impl crate::Readable for MACVLANTR {}
///`write(|w| ..)` method takes [macvlantr::W](macvlantr::W) writer structure
impl crate::Writable for MACVLANTR {}
///Ethernet MAC VLAN tag register
pub mod macvlantr;
///Ethernet MAC PMT control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macpmtcsr](macpmtcsr) module
pub type MACPMTCSR = crate::Reg<u32, _MACPMTCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPMTCSR;
///`read()` method returns [macpmtcsr::R](macpmtcsr::R) reader structure
impl crate::Readable for MACPMTCSR {}
///`write(|w| ..)` method takes [macpmtcsr::W](macpmtcsr::W) writer structure
impl crate::Writable for MACPMTCSR {}
///Ethernet MAC PMT control and status register
pub mod macpmtcsr;
///Ethernet MAC debug register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macdbgr](macdbgr) module
pub type MACDBGR = crate::Reg<u32, _MACDBGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACDBGR;
///`read()` method returns [macdbgr::R](macdbgr::R) reader structure
impl crate::Readable for MACDBGR {}
///Ethernet MAC debug register
pub mod macdbgr;
///Ethernet MAC interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macsr](macsr) module
pub type MACSR = crate::Reg<u32, _MACSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSR;
///`read()` method returns [macsr::R](macsr::R) reader structure
impl crate::Readable for MACSR {}
///`write(|w| ..)` method takes [macsr::W](macsr::W) writer structure
impl crate::Writable for MACSR {}
///Ethernet MAC interrupt status register
pub mod macsr;
///Ethernet MAC interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macimr](macimr) module
pub type MACIMR = crate::Reg<u32, _MACIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACIMR;
///`read()` method returns [macimr::R](macimr::R) reader structure
impl crate::Readable for MACIMR {}
///`write(|w| ..)` method takes [macimr::W](macimr::W) writer structure
impl crate::Writable for MACIMR {}
///Ethernet MAC interrupt mask register
pub mod macimr;
///Ethernet MAC address 0 high register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca0hr](maca0hr) module
pub type MACA0HR = crate::Reg<u32, _MACA0HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA0HR;
///`read()` method returns [maca0hr::R](maca0hr::R) reader structure
impl crate::Readable for MACA0HR {}
///`write(|w| ..)` method takes [maca0hr::W](maca0hr::W) writer structure
impl crate::Writable for MACA0HR {}
///Ethernet MAC address 0 high register
pub mod maca0hr;
///Ethernet MAC address 0 low register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca0lr](maca0lr) module
pub type MACA0LR = crate::Reg<u32, _MACA0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA0LR;
///`read()` method returns [maca0lr::R](maca0lr::R) reader structure
impl crate::Readable for MACA0LR {}
///`write(|w| ..)` method takes [maca0lr::W](maca0lr::W) writer structure
impl crate::Writable for MACA0LR {}
///Ethernet MAC address 0 low register
pub mod maca0lr;
///Ethernet MAC address 1 high register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca1hr](maca1hr) module
pub type MACA1HR = crate::Reg<u32, _MACA1HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA1HR;
///`read()` method returns [maca1hr::R](maca1hr::R) reader structure
impl crate::Readable for MACA1HR {}
///`write(|w| ..)` method takes [maca1hr::W](maca1hr::W) writer structure
impl crate::Writable for MACA1HR {}
///Ethernet MAC address 1 high register
pub mod maca1hr;
///Ethernet MAC address1 low register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca1lr](maca1lr) module
pub type MACA1LR = crate::Reg<u32, _MACA1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA1LR;
///`read()` method returns [maca1lr::R](maca1lr::R) reader structure
impl crate::Readable for MACA1LR {}
///`write(|w| ..)` method takes [maca1lr::W](maca1lr::W) writer structure
impl crate::Writable for MACA1LR {}
///Ethernet MAC address1 low register
pub mod maca1lr;
///Ethernet MAC address 2 high register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca2hr](maca2hr) module
pub type MACA2HR = crate::Reg<u32, _MACA2HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA2HR;
///`read()` method returns [maca2hr::R](maca2hr::R) reader structure
impl crate::Readable for MACA2HR {}
///`write(|w| ..)` method takes [maca2hr::W](maca2hr::W) writer structure
impl crate::Writable for MACA2HR {}
///Ethernet MAC address 2 high register
pub mod maca2hr;
///Ethernet MAC address 2 low register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca2lr](maca2lr) module
pub type MACA2LR = crate::Reg<u32, _MACA2LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA2LR;
///`read()` method returns [maca2lr::R](maca2lr::R) reader structure
impl crate::Readable for MACA2LR {}
///`write(|w| ..)` method takes [maca2lr::W](maca2lr::W) writer structure
impl crate::Writable for MACA2LR {}
///Ethernet MAC address 2 low register
pub mod maca2lr;
///Ethernet MAC address 3 high register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca3hr](maca3hr) module
pub type MACA3HR = crate::Reg<u32, _MACA3HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA3HR;
///`read()` method returns [maca3hr::R](maca3hr::R) reader structure
impl crate::Readable for MACA3HR {}
///`write(|w| ..)` method takes [maca3hr::W](maca3hr::W) writer structure
impl crate::Writable for MACA3HR {}
///Ethernet MAC address 3 high register
pub mod maca3hr;
///Ethernet MAC address 3 low register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca3lr](maca3lr) module
pub type MACA3LR = crate::Reg<u32, _MACA3LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA3LR;
///`read()` method returns [maca3lr::R](maca3lr::R) reader structure
impl crate::Readable for MACA3LR {}
///`write(|w| ..)` method takes [maca3lr::W](maca3lr::W) writer structure
impl crate::Writable for MACA3LR {}
///Ethernet MAC address 3 low register
pub mod maca3lr;
///Ethernet MAC remote wakeup frame filter register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macrwuffer](macrwuffer) module
pub type MACRWUFFER = crate::Reg<u32, _MACRWUFFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACRWUFFER;
///`read()` method returns [macrwuffer::R](macrwuffer::R) reader structure
impl crate::Readable for MACRWUFFER {}
///`write(|w| ..)` method takes [macrwuffer::W](macrwuffer::W) writer structure
impl crate::Writable for MACRWUFFER {}
///Ethernet MAC remote wakeup frame filter register
pub mod macrwuffer;
