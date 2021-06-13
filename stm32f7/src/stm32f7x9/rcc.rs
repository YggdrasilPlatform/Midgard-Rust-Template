///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - clock control register
    pub cr: CR,
    ///0x04 - PLL configuration register
    pub pllcfgr: PLLCFGR,
    ///0x08 - clock configuration register
    pub cfgr: CFGR,
    ///0x0c - clock interrupt register
    pub cir: CIR,
    ///0x10 - AHB1 peripheral reset register
    pub ahb1rstr: AHB1RSTR,
    ///0x14 - AHB2 peripheral reset register
    pub ahb2rstr: AHB2RSTR,
    ///0x18 - AHB3 peripheral reset register
    pub ahb3rstr: AHB3RSTR,
    _reserved7: [u8; 4usize],
    ///0x20 - APB1 peripheral reset register
    pub apb1rstr: APB1RSTR,
    ///0x24 - APB2 peripheral reset register
    pub apb2rstr: APB2RSTR,
    _reserved9: [u8; 8usize],
    ///0x30 - AHB1 peripheral clock register
    pub ahb1enr: AHB1ENR,
    ///0x34 - AHB2 peripheral clock enable register
    pub ahb2enr: AHB2ENR,
    ///0x38 - AHB3 peripheral clock enable register
    pub ahb3enr: AHB3ENR,
    _reserved12: [u8; 4usize],
    ///0x40 - APB1 peripheral clock enable register
    pub apb1enr: APB1ENR,
    ///0x44 - APB2 peripheral clock enable register
    pub apb2enr: APB2ENR,
    _reserved14: [u8; 8usize],
    ///0x50 - AHB1 peripheral clock enable in low power mode register
    pub ahb1lpenr: AHB1LPENR,
    ///0x54 - AHB2 peripheral clock enable in low power mode register
    pub ahb2lpenr: AHB2LPENR,
    ///0x58 - AHB3 peripheral clock enable in low power mode register
    pub ahb3lpenr: AHB3LPENR,
    _reserved17: [u8; 4usize],
    ///0x60 - APB1 peripheral clock enable in low power mode register
    pub apb1lpenr: APB1LPENR,
    ///0x64 - APB2 peripheral clock enabled in low power mode register
    pub apb2lpenr: APB2LPENR,
    _reserved19: [u8; 8usize],
    ///0x70 - Backup domain control register
    pub bdcr: BDCR,
    ///0x74 - clock control & status register
    pub csr: CSR,
    _reserved21: [u8; 8usize],
    ///0x80 - spread spectrum clock generation register
    pub sscgr: SSCGR,
    ///0x84 - PLLI2S configuration register
    pub plli2scfgr: PLLI2SCFGR,
    ///0x88 - PLL configuration register
    pub pllsaicfgr: PLLSAICFGR,
    ///0x8c - dedicated clocks configuration register
    pub dckcfgr1: DCKCFGR1,
    ///0x90 - dedicated clocks configuration register
    pub dckcfgr2: DCKCFGR2,
}
///clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](cr) module
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
///`read()` method returns [cr::R](cr::R) reader structure
impl crate::Readable for CR {}
///`write(|w| ..)` method takes [cr::W](cr::W) writer structure
impl crate::Writable for CR {}
///clock control register
pub mod cr;
///PLL configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllcfgr](pllcfgr) module
pub type PLLCFGR = crate::Reg<u32, _PLLCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFGR;
///`read()` method returns [pllcfgr::R](pllcfgr::R) reader structure
impl crate::Readable for PLLCFGR {}
///`write(|w| ..)` method takes [pllcfgr::W](pllcfgr::W) writer structure
impl crate::Writable for PLLCFGR {}
///PLL configuration register
pub mod pllcfgr;
///clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](cfgr) module
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
///`read()` method returns [cfgr::R](cfgr::R) reader structure
impl crate::Readable for CFGR {}
///`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure
impl crate::Writable for CFGR {}
///clock configuration register
pub mod cfgr;
///clock interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cir](cir) module
pub type CIR = crate::Reg<u32, _CIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIR;
///`read()` method returns [cir::R](cir::R) reader structure
impl crate::Readable for CIR {}
///`write(|w| ..)` method takes [cir::W](cir::W) writer structure
impl crate::Writable for CIR {}
///clock interrupt register
pub mod cir;
///AHB1 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1rstr](ahb1rstr) module
pub type AHB1RSTR = crate::Reg<u32, _AHB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1RSTR;
///`read()` method returns [ahb1rstr::R](ahb1rstr::R) reader structure
impl crate::Readable for AHB1RSTR {}
///`write(|w| ..)` method takes [ahb1rstr::W](ahb1rstr::W) writer structure
impl crate::Writable for AHB1RSTR {}
///AHB1 peripheral reset register
pub mod ahb1rstr;
///AHB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2rstr](ahb2rstr) module
pub type AHB2RSTR = crate::Reg<u32, _AHB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2RSTR;
///`read()` method returns [ahb2rstr::R](ahb2rstr::R) reader structure
impl crate::Readable for AHB2RSTR {}
///`write(|w| ..)` method takes [ahb2rstr::W](ahb2rstr::W) writer structure
impl crate::Writable for AHB2RSTR {}
///AHB2 peripheral reset register
pub mod ahb2rstr;
///AHB3 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3rstr](ahb3rstr) module
pub type AHB3RSTR = crate::Reg<u32, _AHB3RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3RSTR;
///`read()` method returns [ahb3rstr::R](ahb3rstr::R) reader structure
impl crate::Readable for AHB3RSTR {}
///`write(|w| ..)` method takes [ahb3rstr::W](ahb3rstr::W) writer structure
impl crate::Writable for AHB3RSTR {}
///AHB3 peripheral reset register
pub mod ahb3rstr;
///APB1 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1rstr](apb1rstr) module
pub type APB1RSTR = crate::Reg<u32, _APB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR;
///`read()` method returns [apb1rstr::R](apb1rstr::R) reader structure
impl crate::Readable for APB1RSTR {}
///`write(|w| ..)` method takes [apb1rstr::W](apb1rstr::W) writer structure
impl crate::Writable for APB1RSTR {}
///APB1 peripheral reset register
pub mod apb1rstr;
///APB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2rstr](apb2rstr) module
pub type APB2RSTR = crate::Reg<u32, _APB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RSTR;
///`read()` method returns [apb2rstr::R](apb2rstr::R) reader structure
impl crate::Readable for APB2RSTR {}
///`write(|w| ..)` method takes [apb2rstr::W](apb2rstr::W) writer structure
impl crate::Writable for APB2RSTR {}
///APB2 peripheral reset register
pub mod apb2rstr;
///AHB1 peripheral clock register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1enr](ahb1enr) module
pub type AHB1ENR = crate::Reg<u32, _AHB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1ENR;
///`read()` method returns [ahb1enr::R](ahb1enr::R) reader structure
impl crate::Readable for AHB1ENR {}
///`write(|w| ..)` method takes [ahb1enr::W](ahb1enr::W) writer structure
impl crate::Writable for AHB1ENR {}
///AHB1 peripheral clock register
pub mod ahb1enr;
///AHB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2enr](ahb2enr) module
pub type AHB2ENR = crate::Reg<u32, _AHB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2ENR;
///`read()` method returns [ahb2enr::R](ahb2enr::R) reader structure
impl crate::Readable for AHB2ENR {}
///`write(|w| ..)` method takes [ahb2enr::W](ahb2enr::W) writer structure
impl crate::Writable for AHB2ENR {}
///AHB2 peripheral clock enable register
pub mod ahb2enr;
///AHB3 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3enr](ahb3enr) module
pub type AHB3ENR = crate::Reg<u32, _AHB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3ENR;
///`read()` method returns [ahb3enr::R](ahb3enr::R) reader structure
impl crate::Readable for AHB3ENR {}
///`write(|w| ..)` method takes [ahb3enr::W](ahb3enr::W) writer structure
impl crate::Writable for AHB3ENR {}
///AHB3 peripheral clock enable register
pub mod ahb3enr;
///APB1 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1enr](apb1enr) module
pub type APB1ENR = crate::Reg<u32, _APB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR;
///`read()` method returns [apb1enr::R](apb1enr::R) reader structure
impl crate::Readable for APB1ENR {}
///`write(|w| ..)` method takes [apb1enr::W](apb1enr::W) writer structure
impl crate::Writable for APB1ENR {}
///APB1 peripheral clock enable register
pub mod apb1enr;
///APB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2enr](apb2enr) module
pub type APB2ENR = crate::Reg<u32, _APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2ENR;
///`read()` method returns [apb2enr::R](apb2enr::R) reader structure
impl crate::Readable for APB2ENR {}
///`write(|w| ..)` method takes [apb2enr::W](apb2enr::W) writer structure
impl crate::Writable for APB2ENR {}
///APB2 peripheral clock enable register
pub mod apb2enr;
///AHB1 peripheral clock enable in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1lpenr](ahb1lpenr) module
pub type AHB1LPENR = crate::Reg<u32, _AHB1LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1LPENR;
///`read()` method returns [ahb1lpenr::R](ahb1lpenr::R) reader structure
impl crate::Readable for AHB1LPENR {}
///`write(|w| ..)` method takes [ahb1lpenr::W](ahb1lpenr::W) writer structure
impl crate::Writable for AHB1LPENR {}
///AHB1 peripheral clock enable in low power mode register
pub mod ahb1lpenr;
///AHB2 peripheral clock enable in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2lpenr](ahb2lpenr) module
pub type AHB2LPENR = crate::Reg<u32, _AHB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2LPENR;
///`read()` method returns [ahb2lpenr::R](ahb2lpenr::R) reader structure
impl crate::Readable for AHB2LPENR {}
///`write(|w| ..)` method takes [ahb2lpenr::W](ahb2lpenr::W) writer structure
impl crate::Writable for AHB2LPENR {}
///AHB2 peripheral clock enable in low power mode register
pub mod ahb2lpenr;
///AHB3 peripheral clock enable in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3lpenr](ahb3lpenr) module
pub type AHB3LPENR = crate::Reg<u32, _AHB3LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3LPENR;
///`read()` method returns [ahb3lpenr::R](ahb3lpenr::R) reader structure
impl crate::Readable for AHB3LPENR {}
///`write(|w| ..)` method takes [ahb3lpenr::W](ahb3lpenr::W) writer structure
impl crate::Writable for AHB3LPENR {}
///AHB3 peripheral clock enable in low power mode register
pub mod ahb3lpenr;
///APB1 peripheral clock enable in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1lpenr](apb1lpenr) module
pub type APB1LPENR = crate::Reg<u32, _APB1LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LPENR;
///`read()` method returns [apb1lpenr::R](apb1lpenr::R) reader structure
impl crate::Readable for APB1LPENR {}
///`write(|w| ..)` method takes [apb1lpenr::W](apb1lpenr::W) writer structure
impl crate::Writable for APB1LPENR {}
///APB1 peripheral clock enable in low power mode register
pub mod apb1lpenr;
///APB2 peripheral clock enabled in low power mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2lpenr](apb2lpenr) module
pub type APB2LPENR = crate::Reg<u32, _APB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2LPENR;
///`read()` method returns [apb2lpenr::R](apb2lpenr::R) reader structure
impl crate::Readable for APB2LPENR {}
///`write(|w| ..)` method takes [apb2lpenr::W](apb2lpenr::W) writer structure
impl crate::Writable for APB2LPENR {}
///APB2 peripheral clock enabled in low power mode register
pub mod apb2lpenr;
///Backup domain control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdcr](bdcr) module
pub type BDCR = crate::Reg<u32, _BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCR;
///`read()` method returns [bdcr::R](bdcr::R) reader structure
impl crate::Readable for BDCR {}
///`write(|w| ..)` method takes [bdcr::W](bdcr::W) writer structure
impl crate::Writable for BDCR {}
///Backup domain control register
pub mod bdcr;
///clock control & status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](csr) module
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
///`read()` method returns [csr::R](csr::R) reader structure
impl crate::Readable for CSR {}
///`write(|w| ..)` method takes [csr::W](csr::W) writer structure
impl crate::Writable for CSR {}
///clock control & status register
pub mod csr;
///spread spectrum clock generation register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sscgr](sscgr) module
pub type SSCGR = crate::Reg<u32, _SSCGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCGR;
///`read()` method returns [sscgr::R](sscgr::R) reader structure
impl crate::Readable for SSCGR {}
///`write(|w| ..)` method takes [sscgr::W](sscgr::W) writer structure
impl crate::Writable for SSCGR {}
///spread spectrum clock generation register
pub mod sscgr;
///PLLI2S configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [plli2scfgr](plli2scfgr) module
pub type PLLI2SCFGR = crate::Reg<u32, _PLLI2SCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLI2SCFGR;
///`read()` method returns [plli2scfgr::R](plli2scfgr::R) reader structure
impl crate::Readable for PLLI2SCFGR {}
///`write(|w| ..)` method takes [plli2scfgr::W](plli2scfgr::W) writer structure
impl crate::Writable for PLLI2SCFGR {}
///PLLI2S configuration register
pub mod plli2scfgr;
///PLL configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllsaicfgr](pllsaicfgr) module
pub type PLLSAICFGR = crate::Reg<u32, _PLLSAICFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSAICFGR;
///`read()` method returns [pllsaicfgr::R](pllsaicfgr::R) reader structure
impl crate::Readable for PLLSAICFGR {}
///`write(|w| ..)` method takes [pllsaicfgr::W](pllsaicfgr::W) writer structure
impl crate::Writable for PLLSAICFGR {}
///PLL configuration register
pub mod pllsaicfgr;
///dedicated clocks configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dckcfgr1](dckcfgr1) module
pub type DCKCFGR1 = crate::Reg<u32, _DCKCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCKCFGR1;
///`read()` method returns [dckcfgr1::R](dckcfgr1::R) reader structure
impl crate::Readable for DCKCFGR1 {}
///`write(|w| ..)` method takes [dckcfgr1::W](dckcfgr1::W) writer structure
impl crate::Writable for DCKCFGR1 {}
///dedicated clocks configuration register
pub mod dckcfgr1;
///dedicated clocks configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dckcfgr2](dckcfgr2) module
pub type DCKCFGR2 = crate::Reg<u32, _DCKCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCKCFGR2;
///`read()` method returns [dckcfgr2::R](dckcfgr2::R) reader structure
impl crate::Readable for DCKCFGR2 {}
///`write(|w| ..)` method takes [dckcfgr2::W](dckcfgr2::W) writer structure
impl crate::Writable for DCKCFGR2 {}
///dedicated clocks configuration register
pub mod dckcfgr2;
