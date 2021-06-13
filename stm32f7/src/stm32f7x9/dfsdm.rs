///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DFSDM channel configuration 0 register 1
    pub dfsdm_chcfg0r1: DFSDM_CHCFG0R1,
    ///0x04 - DFSDM channel configuration 1 register 1
    pub dfsdm_chcfg1r1: DFSDM_CHCFG1R1,
    ///0x08 - DFSDM channel configuration 2 register 1
    pub dfsdm_chcfg2r1: DFSDM_CHCFG2R1,
    ///0x0c - DFSDM channel configuration 3 register 1
    pub dfsdm_chcfg3r1: DFSDM_CHCFG3R1,
    ///0x10 - DFSDM channel configuration 4 register 1
    pub dfsdm_chcfg4r1: DFSDM_CHCFG4R1,
    ///0x14 - DFSDM channel configuration 5 register 1
    pub dfsdm_chcfg5r1: DFSDM_CHCFG5R1,
    ///0x18 - DFSDM channel configuration 6 register 1
    pub dfsdm_chcfg6r1: DFSDM_CHCFG6R1,
    ///0x1c - DFSDM channel configuration 7 register 1
    pub dfsdm_chcfg7r1: DFSDM_CHCFG7R1,
    ///0x20 - DFSDM channel configuration 0 register 2
    pub dfsdm_chcfg0r2: DFSDM_CHCFG0R2,
    ///0x24 - DFSDM channel configuration 1 register 2
    pub dfsdm_chcfg1r2: DFSDM_CHCFG1R2,
    ///0x28 - DFSDM channel configuration 2 register 2
    pub dfsdm_chcfg2r2: DFSDM_CHCFG2R2,
    ///0x2c - DFSDM channel configuration 3 register 2
    pub dfsdm_chcfg3r2: DFSDM_CHCFG3R2,
    ///0x30 - DFSDM channel configuration 4 register 2
    pub dfsdm_chcfg4r2: DFSDM_CHCFG4R2,
    ///0x34 - DFSDM channel configuration 5 register 2
    pub dfsdm_chcfg5r2: DFSDM_CHCFG5R2,
    ///0x38 - DFSDM channel configuration 6 register 2
    pub dfsdm_chcfg6r2: DFSDM_CHCFG6R2,
    ///0x3c - DFSDM channel configuration 7 register 2
    pub dfsdm_chcfg7r2: DFSDM_CHCFG7R2,
    ///0x40 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd0r: DFSDM_AWSCD0R,
    ///0x44 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd1r: DFSDM_AWSCD1R,
    ///0x48 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd2r: DFSDM_AWSCD2R,
    ///0x4c - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd3r: DFSDM_AWSCD3R,
    ///0x50 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd4r: DFSDM_AWSCD4R,
    ///0x54 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd5r: DFSDM_AWSCD5R,
    ///0x58 - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd6r: DFSDM_AWSCD6R,
    ///0x5c - DFSDM analog watchdog and short-circuit detector register
    pub dfsdm_awscd7r: DFSDM_AWSCD7R,
    ///0x60 - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat0r: DFSDM_CHWDAT0R,
    ///0x64 - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat1r: DFSDM_CHWDAT1R,
    ///0x68 - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat2r: DFSDM_CHWDAT2R,
    ///0x6c - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat3r: DFSDM_CHWDAT3R,
    ///0x70 - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat4r: DFSDM_CHWDAT4R,
    ///0x74 - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat5r: DFSDM_CHWDAT5R,
    ///0x78 - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat6r: DFSDM_CHWDAT6R,
    ///0x7c - DFSDM channel watchdog filter data register
    pub dfsdm_chwdat7r: DFSDM_CHWDAT7R,
    ///0x80 - DFSDM channel data input register
    pub dfsdm_chdatin0r: DFSDM_CHDATIN0R,
    ///0x84 - DFSDM channel data input register
    pub dfsdm_chdatin1r: DFSDM_CHDATIN1R,
    ///0x88 - DFSDM channel data input register
    pub dfsdm_chdatin2r: DFSDM_CHDATIN2R,
    ///0x8c - DFSDM channel data input register
    pub dfsdm_chdatin3r: DFSDM_CHDATIN3R,
    ///0x90 - DFSDM channel data input register
    pub dfsdm_chdatin4r: DFSDM_CHDATIN4R,
    ///0x94 - DFSDM channel data input register
    pub dfsdm_chdatin5r: DFSDM_CHDATIN5R,
    ///0x98 - DFSDM channel data input register
    pub dfsdm_chdatin6r: DFSDM_CHDATIN6R,
    ///0x9c - DFSDM channel data input register
    pub dfsdm_chdatin7r: DFSDM_CHDATIN7R,
    ///0xa0 - DFSDM control register 1
    pub dfsdm0_cr1: DFSDM0_CR1,
    ///0xa4 - DFSDM control register 1
    pub dfsdm1_cr1: DFSDM1_CR1,
    ///0xa8 - DFSDM control register 1
    pub dfsdm2_cr1: DFSDM2_CR1,
    ///0xac - DFSDM control register 1
    pub dfsdm3_cr1: DFSDM3_CR1,
    ///0xb0 - DFSDM control register 2
    pub dfsdm0_cr2: DFSDM0_CR2,
    ///0xb4 - DFSDM control register 2
    pub dfsdm1_cr2: DFSDM1_CR2,
    ///0xb8 - DFSDM control register 2
    pub dfsdm2_cr2: DFSDM2_CR2,
    ///0xbc - DFSDM control register 2
    pub dfsdm3_cr2: DFSDM3_CR2,
    ///0xc0 - DFSDM interrupt and status register
    pub dfsdm0_isr: DFSDM0_ISR,
    ///0xc4 - DFSDM interrupt and status register
    pub dfsdm1_isr: DFSDM1_ISR,
    ///0xc8 - DFSDM interrupt and status register
    pub dfsdm2_isr: DFSDM2_ISR,
    ///0xcc - DFSDM interrupt and status register
    pub dfsdm3_isr: DFSDM3_ISR,
    ///0xd0 - DFSDM interrupt flag clear register
    pub dfsdm0_icr: DFSDM0_ICR,
    ///0xd4 - DFSDM interrupt flag clear register
    pub dfsdm1_icr: DFSDM1_ICR,
    ///0xd8 - DFSDM interrupt flag clear register
    pub dfsdm2_icr: DFSDM2_ICR,
    ///0xdc - DFSDM interrupt flag clear register
    pub dfsdm3_icr: DFSDM3_ICR,
    ///0xe0 - DFSDM injected channel group selection register
    pub dfsdm0_jchgr: DFSDM0_JCHGR,
    ///0xe4 - DFSDM injected channel group selection register
    pub dfsdm1_jchgr: DFSDM1_JCHGR,
    ///0xe8 - DFSDM injected channel group selection register
    pub dfsdm2_jchgr: DFSDM2_JCHGR,
    ///0xec - DFSDM injected channel group selection register
    pub dfsdm3_jchgr: DFSDM3_JCHGR,
    ///0xf0 - DFSDM filter control register
    pub dfsdm0_fcr: DFSDM0_FCR,
    ///0xf4 - DFSDM filter control register
    pub dfsdm1_fcr: DFSDM1_FCR,
    ///0xf8 - DFSDM filter control register
    pub dfsdm2_fcr: DFSDM2_FCR,
    ///0xfc - DFSDM filter control register
    pub dfsdm3_fcr: DFSDM3_FCR,
    ///0x100 - DFSDM data register for injected group
    pub dfsdm0_jdatar: DFSDM0_JDATAR,
    ///0x104 - DFSDM data register for injected group
    pub dfsdm1_jdatar: DFSDM1_JDATAR,
    ///0x108 - DFSDM data register for injected group
    pub dfsdm2_jdatar: DFSDM2_JDATAR,
    ///0x10c - DFSDM data register for injected group
    pub dfsdm3_jdatar: DFSDM3_JDATAR,
    ///0x110 - DFSDM data register for the regular channel
    pub dfsdm0_rdatar: DFSDM0_RDATAR,
    ///0x114 - DFSDM data register for the regular channel
    pub dfsdm1_rdatar: DFSDM1_RDATAR,
    ///0x118 - DFSDM data register for the regular channel
    pub dfsdm2_rdatar: DFSDM2_RDATAR,
    ///0x11c - DFSDM data register for the regular channel
    pub dfsdm3_rdatar: DFSDM3_RDATAR,
    ///0x120 - DFSDM analog watchdog high threshold register
    pub dfsdm0_awhtr: DFSDM0_AWHTR,
    ///0x124 - DFSDM analog watchdog high threshold register
    pub dfsdm1_awhtr: DFSDM1_AWHTR,
    ///0x128 - DFSDM analog watchdog high threshold register
    pub dfsdm2_awhtr: DFSDM2_AWHTR,
    ///0x12c - DFSDM analog watchdog high threshold register
    pub dfsdm3_awhtr: DFSDM3_AWHTR,
    ///0x130 - DFSDM analog watchdog low threshold register
    pub dfsdm0_awltr: DFSDM0_AWLTR,
    ///0x134 - DFSDM analog watchdog low threshold register
    pub dfsdm1_awltr: DFSDM1_AWLTR,
    ///0x138 - DFSDM analog watchdog low threshold register
    pub dfsdm2_awltr: DFSDM2_AWLTR,
    ///0x13c - DFSDM analog watchdog low threshold register
    pub dfsdm3_awltr: DFSDM3_AWLTR,
    ///0x140 - DFSDM analog watchdog status register
    pub dfsdm0_awsr: DFSDM0_AWSR,
    ///0x144 - DFSDM analog watchdog status register
    pub dfsdm1_awsr: DFSDM1_AWSR,
    ///0x148 - DFSDM analog watchdog status register
    pub dfsdm2_awsr: DFSDM2_AWSR,
    ///0x14c - DFSDM analog watchdog status register
    pub dfsdm3_awsr: DFSDM3_AWSR,
    ///0x150 - DFSDM analog watchdog clear flag register
    pub dfsdm0_awcfr: DFSDM0_AWCFR,
    ///0x154 - DFSDM analog watchdog clear flag register
    pub dfsdm1_awcfr: DFSDM1_AWCFR,
    ///0x158 - DFSDM analog watchdog clear flag register
    pub dfsdm2_awcfr: DFSDM2_AWCFR,
    ///0x15c - DFSDM analog watchdog clear flag register
    pub dfsdm3_awcfr: DFSDM3_AWCFR,
    ///0x160 - DFSDM Extremes detector maximum register
    pub dfsdm0_exmax: DFSDM0_EXMAX,
    ///0x164 - DFSDM Extremes detector maximum register
    pub dfsdm1_exmax: DFSDM1_EXMAX,
    ///0x168 - DFSDM Extremes detector maximum register
    pub dfsdm2_exmax: DFSDM2_EXMAX,
    ///0x16c - DFSDM Extremes detector maximum register
    pub dfsdm3_exmax: DFSDM3_EXMAX,
    ///0x170 - DFSDM Extremes detector minimum register
    pub dfsdm0_exmin: DFSDM0_EXMIN,
    ///0x174 - DFSDM Extremes detector minimum register
    pub dfsdm1_exmin: DFSDM1_EXMIN,
    ///0x178 - DFSDM Extremes detector minimum register
    pub dfsdm2_exmin: DFSDM2_EXMIN,
    ///0x17c - DFSDM Extremes detector minimum register
    pub dfsdm3_exmin: DFSDM3_EXMIN,
    ///0x180 - DFSDM conversion timer register
    pub dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    ///0x184 - DFSDM conversion timer register
    pub dfsdm1_cnvtimr: DFSDM1_CNVTIMR,
    ///0x188 - DFSDM conversion timer register
    pub dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    ///0x18c - DFSDM conversion timer register
    pub dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
}
///DFSDM channel configuration 0 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg0r1](dfsdm_chcfg0r1) module
pub type DFSDM_CHCFG0R1 = crate::Reg<u32, _DFSDM_CHCFG0R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG0R1;
///`read()` method returns [dfsdm_chcfg0r1::R](dfsdm_chcfg0r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG0R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg0r1::W](dfsdm_chcfg0r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG0R1 {}
///DFSDM channel configuration 0 register 1
pub mod dfsdm_chcfg0r1;
///DFSDM channel configuration 1 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg1r1](dfsdm_chcfg1r1) module
pub type DFSDM_CHCFG1R1 = crate::Reg<u32, _DFSDM_CHCFG1R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG1R1;
///`read()` method returns [dfsdm_chcfg1r1::R](dfsdm_chcfg1r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG1R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg1r1::W](dfsdm_chcfg1r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG1R1 {}
///DFSDM channel configuration 1 register 1
pub mod dfsdm_chcfg1r1;
///DFSDM channel configuration 2 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg2r1](dfsdm_chcfg2r1) module
pub type DFSDM_CHCFG2R1 = crate::Reg<u32, _DFSDM_CHCFG2R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG2R1;
///`read()` method returns [dfsdm_chcfg2r1::R](dfsdm_chcfg2r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG2R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg2r1::W](dfsdm_chcfg2r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG2R1 {}
///DFSDM channel configuration 2 register 1
pub mod dfsdm_chcfg2r1;
///DFSDM channel configuration 3 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg3r1](dfsdm_chcfg3r1) module
pub type DFSDM_CHCFG3R1 = crate::Reg<u32, _DFSDM_CHCFG3R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG3R1;
///`read()` method returns [dfsdm_chcfg3r1::R](dfsdm_chcfg3r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG3R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg3r1::W](dfsdm_chcfg3r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG3R1 {}
///DFSDM channel configuration 3 register 1
pub mod dfsdm_chcfg3r1;
///DFSDM channel configuration 4 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg4r1](dfsdm_chcfg4r1) module
pub type DFSDM_CHCFG4R1 = crate::Reg<u32, _DFSDM_CHCFG4R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG4R1;
///`read()` method returns [dfsdm_chcfg4r1::R](dfsdm_chcfg4r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG4R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg4r1::W](dfsdm_chcfg4r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG4R1 {}
///DFSDM channel configuration 4 register 1
pub mod dfsdm_chcfg4r1;
///DFSDM channel configuration 5 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg5r1](dfsdm_chcfg5r1) module
pub type DFSDM_CHCFG5R1 = crate::Reg<u32, _DFSDM_CHCFG5R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG5R1;
///`read()` method returns [dfsdm_chcfg5r1::R](dfsdm_chcfg5r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG5R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg5r1::W](dfsdm_chcfg5r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG5R1 {}
///DFSDM channel configuration 5 register 1
pub mod dfsdm_chcfg5r1;
///DFSDM channel configuration 6 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg6r1](dfsdm_chcfg6r1) module
pub type DFSDM_CHCFG6R1 = crate::Reg<u32, _DFSDM_CHCFG6R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG6R1;
///`read()` method returns [dfsdm_chcfg6r1::R](dfsdm_chcfg6r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG6R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg6r1::W](dfsdm_chcfg6r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG6R1 {}
///DFSDM channel configuration 6 register 1
pub mod dfsdm_chcfg6r1;
///DFSDM channel configuration 7 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg7r1](dfsdm_chcfg7r1) module
pub type DFSDM_CHCFG7R1 = crate::Reg<u32, _DFSDM_CHCFG7R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG7R1;
///`read()` method returns [dfsdm_chcfg7r1::R](dfsdm_chcfg7r1::R) reader structure
impl crate::Readable for DFSDM_CHCFG7R1 {}
///`write(|w| ..)` method takes [dfsdm_chcfg7r1::W](dfsdm_chcfg7r1::W) writer structure
impl crate::Writable for DFSDM_CHCFG7R1 {}
///DFSDM channel configuration 7 register 1
pub mod dfsdm_chcfg7r1;
///DFSDM channel configuration 0 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg0r2](dfsdm_chcfg0r2) module
pub type DFSDM_CHCFG0R2 = crate::Reg<u32, _DFSDM_CHCFG0R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG0R2;
///`read()` method returns [dfsdm_chcfg0r2::R](dfsdm_chcfg0r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG0R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg0r2::W](dfsdm_chcfg0r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG0R2 {}
///DFSDM channel configuration 0 register 2
pub mod dfsdm_chcfg0r2;
///DFSDM channel configuration 1 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg1r2](dfsdm_chcfg1r2) module
pub type DFSDM_CHCFG1R2 = crate::Reg<u32, _DFSDM_CHCFG1R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG1R2;
///`read()` method returns [dfsdm_chcfg1r2::R](dfsdm_chcfg1r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG1R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg1r2::W](dfsdm_chcfg1r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG1R2 {}
///DFSDM channel configuration 1 register 2
pub mod dfsdm_chcfg1r2;
///DFSDM channel configuration 2 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg2r2](dfsdm_chcfg2r2) module
pub type DFSDM_CHCFG2R2 = crate::Reg<u32, _DFSDM_CHCFG2R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG2R2;
///`read()` method returns [dfsdm_chcfg2r2::R](dfsdm_chcfg2r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG2R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg2r2::W](dfsdm_chcfg2r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG2R2 {}
///DFSDM channel configuration 2 register 2
pub mod dfsdm_chcfg2r2;
///DFSDM channel configuration 3 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg3r2](dfsdm_chcfg3r2) module
pub type DFSDM_CHCFG3R2 = crate::Reg<u32, _DFSDM_CHCFG3R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG3R2;
///`read()` method returns [dfsdm_chcfg3r2::R](dfsdm_chcfg3r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG3R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg3r2::W](dfsdm_chcfg3r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG3R2 {}
///DFSDM channel configuration 3 register 2
pub mod dfsdm_chcfg3r2;
///DFSDM channel configuration 4 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg4r2](dfsdm_chcfg4r2) module
pub type DFSDM_CHCFG4R2 = crate::Reg<u32, _DFSDM_CHCFG4R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG4R2;
///`read()` method returns [dfsdm_chcfg4r2::R](dfsdm_chcfg4r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG4R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg4r2::W](dfsdm_chcfg4r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG4R2 {}
///DFSDM channel configuration 4 register 2
pub mod dfsdm_chcfg4r2;
///DFSDM channel configuration 5 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg5r2](dfsdm_chcfg5r2) module
pub type DFSDM_CHCFG5R2 = crate::Reg<u32, _DFSDM_CHCFG5R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG5R2;
///`read()` method returns [dfsdm_chcfg5r2::R](dfsdm_chcfg5r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG5R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg5r2::W](dfsdm_chcfg5r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG5R2 {}
///DFSDM channel configuration 5 register 2
pub mod dfsdm_chcfg5r2;
///DFSDM channel configuration 6 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg6r2](dfsdm_chcfg6r2) module
pub type DFSDM_CHCFG6R2 = crate::Reg<u32, _DFSDM_CHCFG6R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG6R2;
///`read()` method returns [dfsdm_chcfg6r2::R](dfsdm_chcfg6r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG6R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg6r2::W](dfsdm_chcfg6r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG6R2 {}
///DFSDM channel configuration 6 register 2
pub mod dfsdm_chcfg6r2;
///DFSDM channel configuration 7 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chcfg7r2](dfsdm_chcfg7r2) module
pub type DFSDM_CHCFG7R2 = crate::Reg<u32, _DFSDM_CHCFG7R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG7R2;
///`read()` method returns [dfsdm_chcfg7r2::R](dfsdm_chcfg7r2::R) reader structure
impl crate::Readable for DFSDM_CHCFG7R2 {}
///`write(|w| ..)` method takes [dfsdm_chcfg7r2::W](dfsdm_chcfg7r2::W) writer structure
impl crate::Writable for DFSDM_CHCFG7R2 {}
///DFSDM channel configuration 7 register 2
pub mod dfsdm_chcfg7r2;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd0r](dfsdm_awscd0r) module
pub type DFSDM_AWSCD0R = crate::Reg<u32, _DFSDM_AWSCD0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD0R;
///`read()` method returns [dfsdm_awscd0r::R](dfsdm_awscd0r::R) reader structure
impl crate::Readable for DFSDM_AWSCD0R {}
///`write(|w| ..)` method takes [dfsdm_awscd0r::W](dfsdm_awscd0r::W) writer structure
impl crate::Writable for DFSDM_AWSCD0R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd0r;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd1r](dfsdm_awscd1r) module
pub type DFSDM_AWSCD1R = crate::Reg<u32, _DFSDM_AWSCD1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD1R;
///`read()` method returns [dfsdm_awscd1r::R](dfsdm_awscd1r::R) reader structure
impl crate::Readable for DFSDM_AWSCD1R {}
///`write(|w| ..)` method takes [dfsdm_awscd1r::W](dfsdm_awscd1r::W) writer structure
impl crate::Writable for DFSDM_AWSCD1R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd1r;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd2r](dfsdm_awscd2r) module
pub type DFSDM_AWSCD2R = crate::Reg<u32, _DFSDM_AWSCD2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD2R;
///`read()` method returns [dfsdm_awscd2r::R](dfsdm_awscd2r::R) reader structure
impl crate::Readable for DFSDM_AWSCD2R {}
///`write(|w| ..)` method takes [dfsdm_awscd2r::W](dfsdm_awscd2r::W) writer structure
impl crate::Writable for DFSDM_AWSCD2R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd2r;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd3r](dfsdm_awscd3r) module
pub type DFSDM_AWSCD3R = crate::Reg<u32, _DFSDM_AWSCD3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD3R;
///`read()` method returns [dfsdm_awscd3r::R](dfsdm_awscd3r::R) reader structure
impl crate::Readable for DFSDM_AWSCD3R {}
///`write(|w| ..)` method takes [dfsdm_awscd3r::W](dfsdm_awscd3r::W) writer structure
impl crate::Writable for DFSDM_AWSCD3R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd3r;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd4r](dfsdm_awscd4r) module
pub type DFSDM_AWSCD4R = crate::Reg<u32, _DFSDM_AWSCD4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD4R;
///`read()` method returns [dfsdm_awscd4r::R](dfsdm_awscd4r::R) reader structure
impl crate::Readable for DFSDM_AWSCD4R {}
///`write(|w| ..)` method takes [dfsdm_awscd4r::W](dfsdm_awscd4r::W) writer structure
impl crate::Writable for DFSDM_AWSCD4R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd4r;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd5r](dfsdm_awscd5r) module
pub type DFSDM_AWSCD5R = crate::Reg<u32, _DFSDM_AWSCD5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD5R;
///`read()` method returns [dfsdm_awscd5r::R](dfsdm_awscd5r::R) reader structure
impl crate::Readable for DFSDM_AWSCD5R {}
///`write(|w| ..)` method takes [dfsdm_awscd5r::W](dfsdm_awscd5r::W) writer structure
impl crate::Writable for DFSDM_AWSCD5R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd5r;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd6r](dfsdm_awscd6r) module
pub type DFSDM_AWSCD6R = crate::Reg<u32, _DFSDM_AWSCD6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD6R;
///`read()` method returns [dfsdm_awscd6r::R](dfsdm_awscd6r::R) reader structure
impl crate::Readable for DFSDM_AWSCD6R {}
///`write(|w| ..)` method takes [dfsdm_awscd6r::W](dfsdm_awscd6r::W) writer structure
impl crate::Writable for DFSDM_AWSCD6R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd6r;
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd7r](dfsdm_awscd7r) module
pub type DFSDM_AWSCD7R = crate::Reg<u32, _DFSDM_AWSCD7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD7R;
///`read()` method returns [dfsdm_awscd7r::R](dfsdm_awscd7r::R) reader structure
impl crate::Readable for DFSDM_AWSCD7R {}
///`write(|w| ..)` method takes [dfsdm_awscd7r::W](dfsdm_awscd7r::W) writer structure
impl crate::Writable for DFSDM_AWSCD7R {}
///DFSDM analog watchdog and short-circuit detector register
pub mod dfsdm_awscd7r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat0r](dfsdm_chwdat0r) module
pub type DFSDM_CHWDAT0R = crate::Reg<u32, _DFSDM_CHWDAT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT0R;
///`read()` method returns [dfsdm_chwdat0r::R](dfsdm_chwdat0r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT0R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat0r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat1r](dfsdm_chwdat1r) module
pub type DFSDM_CHWDAT1R = crate::Reg<u32, _DFSDM_CHWDAT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT1R;
///`read()` method returns [dfsdm_chwdat1r::R](dfsdm_chwdat1r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT1R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat1r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat2r](dfsdm_chwdat2r) module
pub type DFSDM_CHWDAT2R = crate::Reg<u32, _DFSDM_CHWDAT2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT2R;
///`read()` method returns [dfsdm_chwdat2r::R](dfsdm_chwdat2r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT2R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat2r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat3r](dfsdm_chwdat3r) module
pub type DFSDM_CHWDAT3R = crate::Reg<u32, _DFSDM_CHWDAT3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT3R;
///`read()` method returns [dfsdm_chwdat3r::R](dfsdm_chwdat3r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT3R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat3r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat4r](dfsdm_chwdat4r) module
pub type DFSDM_CHWDAT4R = crate::Reg<u32, _DFSDM_CHWDAT4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT4R;
///`read()` method returns [dfsdm_chwdat4r::R](dfsdm_chwdat4r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT4R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat4r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat5r](dfsdm_chwdat5r) module
pub type DFSDM_CHWDAT5R = crate::Reg<u32, _DFSDM_CHWDAT5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT5R;
///`read()` method returns [dfsdm_chwdat5r::R](dfsdm_chwdat5r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT5R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat5r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat6r](dfsdm_chwdat6r) module
pub type DFSDM_CHWDAT6R = crate::Reg<u32, _DFSDM_CHWDAT6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT6R;
///`read()` method returns [dfsdm_chwdat6r::R](dfsdm_chwdat6r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT6R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat6r;
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat7r](dfsdm_chwdat7r) module
pub type DFSDM_CHWDAT7R = crate::Reg<u32, _DFSDM_CHWDAT7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT7R;
///`read()` method returns [dfsdm_chwdat7r::R](dfsdm_chwdat7r::R) reader structure
impl crate::Readable for DFSDM_CHWDAT7R {}
///DFSDM channel watchdog filter data register
pub mod dfsdm_chwdat7r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin0r](dfsdm_chdatin0r) module
pub type DFSDM_CHDATIN0R = crate::Reg<u32, _DFSDM_CHDATIN0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN0R;
///`read()` method returns [dfsdm_chdatin0r::R](dfsdm_chdatin0r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN0R {}
///`write(|w| ..)` method takes [dfsdm_chdatin0r::W](dfsdm_chdatin0r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN0R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin0r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin1r](dfsdm_chdatin1r) module
pub type DFSDM_CHDATIN1R = crate::Reg<u32, _DFSDM_CHDATIN1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN1R;
///`read()` method returns [dfsdm_chdatin1r::R](dfsdm_chdatin1r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN1R {}
///`write(|w| ..)` method takes [dfsdm_chdatin1r::W](dfsdm_chdatin1r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN1R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin1r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin2r](dfsdm_chdatin2r) module
pub type DFSDM_CHDATIN2R = crate::Reg<u32, _DFSDM_CHDATIN2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN2R;
///`read()` method returns [dfsdm_chdatin2r::R](dfsdm_chdatin2r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN2R {}
///`write(|w| ..)` method takes [dfsdm_chdatin2r::W](dfsdm_chdatin2r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN2R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin2r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin3r](dfsdm_chdatin3r) module
pub type DFSDM_CHDATIN3R = crate::Reg<u32, _DFSDM_CHDATIN3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN3R;
///`read()` method returns [dfsdm_chdatin3r::R](dfsdm_chdatin3r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN3R {}
///`write(|w| ..)` method takes [dfsdm_chdatin3r::W](dfsdm_chdatin3r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN3R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin3r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin4r](dfsdm_chdatin4r) module
pub type DFSDM_CHDATIN4R = crate::Reg<u32, _DFSDM_CHDATIN4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN4R;
///`read()` method returns [dfsdm_chdatin4r::R](dfsdm_chdatin4r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN4R {}
///`write(|w| ..)` method takes [dfsdm_chdatin4r::W](dfsdm_chdatin4r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN4R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin4r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin5r](dfsdm_chdatin5r) module
pub type DFSDM_CHDATIN5R = crate::Reg<u32, _DFSDM_CHDATIN5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN5R;
///`read()` method returns [dfsdm_chdatin5r::R](dfsdm_chdatin5r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN5R {}
///`write(|w| ..)` method takes [dfsdm_chdatin5r::W](dfsdm_chdatin5r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN5R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin5r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin6r](dfsdm_chdatin6r) module
pub type DFSDM_CHDATIN6R = crate::Reg<u32, _DFSDM_CHDATIN6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN6R;
///`read()` method returns [dfsdm_chdatin6r::R](dfsdm_chdatin6r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN6R {}
///`write(|w| ..)` method takes [dfsdm_chdatin6r::W](dfsdm_chdatin6r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN6R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin6r;
///DFSDM channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chdatin7r](dfsdm_chdatin7r) module
pub type DFSDM_CHDATIN7R = crate::Reg<u32, _DFSDM_CHDATIN7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN7R;
///`read()` method returns [dfsdm_chdatin7r::R](dfsdm_chdatin7r::R) reader structure
impl crate::Readable for DFSDM_CHDATIN7R {}
///`write(|w| ..)` method takes [dfsdm_chdatin7r::W](dfsdm_chdatin7r::W) writer structure
impl crate::Writable for DFSDM_CHDATIN7R {}
///DFSDM channel data input register
pub mod dfsdm_chdatin7r;
///DFSDM control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_cr1](dfsdm0_cr1) module
pub type DFSDM0_CR1 = crate::Reg<u32, _DFSDM0_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CR1;
///`read()` method returns [dfsdm0_cr1::R](dfsdm0_cr1::R) reader structure
impl crate::Readable for DFSDM0_CR1 {}
///`write(|w| ..)` method takes [dfsdm0_cr1::W](dfsdm0_cr1::W) writer structure
impl crate::Writable for DFSDM0_CR1 {}
///DFSDM control register 1
pub mod dfsdm0_cr1;
///DFSDM control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_cr1](dfsdm1_cr1) module
pub type DFSDM1_CR1 = crate::Reg<u32, _DFSDM1_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CR1;
///`read()` method returns [dfsdm1_cr1::R](dfsdm1_cr1::R) reader structure
impl crate::Readable for DFSDM1_CR1 {}
///`write(|w| ..)` method takes [dfsdm1_cr1::W](dfsdm1_cr1::W) writer structure
impl crate::Writable for DFSDM1_CR1 {}
///DFSDM control register 1
pub mod dfsdm1_cr1;
///DFSDM control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_cr1](dfsdm2_cr1) module
pub type DFSDM2_CR1 = crate::Reg<u32, _DFSDM2_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CR1;
///`read()` method returns [dfsdm2_cr1::R](dfsdm2_cr1::R) reader structure
impl crate::Readable for DFSDM2_CR1 {}
///`write(|w| ..)` method takes [dfsdm2_cr1::W](dfsdm2_cr1::W) writer structure
impl crate::Writable for DFSDM2_CR1 {}
///DFSDM control register 1
pub mod dfsdm2_cr1;
///DFSDM control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_cr1](dfsdm3_cr1) module
pub type DFSDM3_CR1 = crate::Reg<u32, _DFSDM3_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CR1;
///`read()` method returns [dfsdm3_cr1::R](dfsdm3_cr1::R) reader structure
impl crate::Readable for DFSDM3_CR1 {}
///`write(|w| ..)` method takes [dfsdm3_cr1::W](dfsdm3_cr1::W) writer structure
impl crate::Writable for DFSDM3_CR1 {}
///DFSDM control register 1
pub mod dfsdm3_cr1;
///DFSDM control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_cr2](dfsdm0_cr2) module
pub type DFSDM0_CR2 = crate::Reg<u32, _DFSDM0_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CR2;
///`read()` method returns [dfsdm0_cr2::R](dfsdm0_cr2::R) reader structure
impl crate::Readable for DFSDM0_CR2 {}
///`write(|w| ..)` method takes [dfsdm0_cr2::W](dfsdm0_cr2::W) writer structure
impl crate::Writable for DFSDM0_CR2 {}
///DFSDM control register 2
pub mod dfsdm0_cr2;
///DFSDM control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_cr2](dfsdm1_cr2) module
pub type DFSDM1_CR2 = crate::Reg<u32, _DFSDM1_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CR2;
///`read()` method returns [dfsdm1_cr2::R](dfsdm1_cr2::R) reader structure
impl crate::Readable for DFSDM1_CR2 {}
///`write(|w| ..)` method takes [dfsdm1_cr2::W](dfsdm1_cr2::W) writer structure
impl crate::Writable for DFSDM1_CR2 {}
///DFSDM control register 2
pub mod dfsdm1_cr2;
///DFSDM control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_cr2](dfsdm2_cr2) module
pub type DFSDM2_CR2 = crate::Reg<u32, _DFSDM2_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CR2;
///`read()` method returns [dfsdm2_cr2::R](dfsdm2_cr2::R) reader structure
impl crate::Readable for DFSDM2_CR2 {}
///`write(|w| ..)` method takes [dfsdm2_cr2::W](dfsdm2_cr2::W) writer structure
impl crate::Writable for DFSDM2_CR2 {}
///DFSDM control register 2
pub mod dfsdm2_cr2;
///DFSDM control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_cr2](dfsdm3_cr2) module
pub type DFSDM3_CR2 = crate::Reg<u32, _DFSDM3_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CR2;
///`read()` method returns [dfsdm3_cr2::R](dfsdm3_cr2::R) reader structure
impl crate::Readable for DFSDM3_CR2 {}
///`write(|w| ..)` method takes [dfsdm3_cr2::W](dfsdm3_cr2::W) writer structure
impl crate::Writable for DFSDM3_CR2 {}
///DFSDM control register 2
pub mod dfsdm3_cr2;
///DFSDM interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_isr](dfsdm0_isr) module
pub type DFSDM0_ISR = crate::Reg<u32, _DFSDM0_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_ISR;
///`read()` method returns [dfsdm0_isr::R](dfsdm0_isr::R) reader structure
impl crate::Readable for DFSDM0_ISR {}
///DFSDM interrupt and status register
pub mod dfsdm0_isr;
///DFSDM interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_isr](dfsdm1_isr) module
pub type DFSDM1_ISR = crate::Reg<u32, _DFSDM1_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_ISR;
///`read()` method returns [dfsdm1_isr::R](dfsdm1_isr::R) reader structure
impl crate::Readable for DFSDM1_ISR {}
///DFSDM interrupt and status register
pub mod dfsdm1_isr;
///DFSDM interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_isr](dfsdm2_isr) module
pub type DFSDM2_ISR = crate::Reg<u32, _DFSDM2_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_ISR;
///`read()` method returns [dfsdm2_isr::R](dfsdm2_isr::R) reader structure
impl crate::Readable for DFSDM2_ISR {}
///DFSDM interrupt and status register
pub mod dfsdm2_isr;
///DFSDM interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_isr](dfsdm3_isr) module
pub type DFSDM3_ISR = crate::Reg<u32, _DFSDM3_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_ISR;
///`read()` method returns [dfsdm3_isr::R](dfsdm3_isr::R) reader structure
impl crate::Readable for DFSDM3_ISR {}
///DFSDM interrupt and status register
pub mod dfsdm3_isr;
///DFSDM interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_icr](dfsdm0_icr) module
pub type DFSDM0_ICR = crate::Reg<u32, _DFSDM0_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_ICR;
///`read()` method returns [dfsdm0_icr::R](dfsdm0_icr::R) reader structure
impl crate::Readable for DFSDM0_ICR {}
///`write(|w| ..)` method takes [dfsdm0_icr::W](dfsdm0_icr::W) writer structure
impl crate::Writable for DFSDM0_ICR {}
///DFSDM interrupt flag clear register
pub mod dfsdm0_icr;
///DFSDM interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_icr](dfsdm1_icr) module
pub type DFSDM1_ICR = crate::Reg<u32, _DFSDM1_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_ICR;
///`read()` method returns [dfsdm1_icr::R](dfsdm1_icr::R) reader structure
impl crate::Readable for DFSDM1_ICR {}
///`write(|w| ..)` method takes [dfsdm1_icr::W](dfsdm1_icr::W) writer structure
impl crate::Writable for DFSDM1_ICR {}
///DFSDM interrupt flag clear register
pub mod dfsdm1_icr;
///DFSDM interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_icr](dfsdm2_icr) module
pub type DFSDM2_ICR = crate::Reg<u32, _DFSDM2_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_ICR;
///`read()` method returns [dfsdm2_icr::R](dfsdm2_icr::R) reader structure
impl crate::Readable for DFSDM2_ICR {}
///`write(|w| ..)` method takes [dfsdm2_icr::W](dfsdm2_icr::W) writer structure
impl crate::Writable for DFSDM2_ICR {}
///DFSDM interrupt flag clear register
pub mod dfsdm2_icr;
///DFSDM interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_icr](dfsdm3_icr) module
pub type DFSDM3_ICR = crate::Reg<u32, _DFSDM3_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_ICR;
///`read()` method returns [dfsdm3_icr::R](dfsdm3_icr::R) reader structure
impl crate::Readable for DFSDM3_ICR {}
///`write(|w| ..)` method takes [dfsdm3_icr::W](dfsdm3_icr::W) writer structure
impl crate::Writable for DFSDM3_ICR {}
///DFSDM interrupt flag clear register
pub mod dfsdm3_icr;
///DFSDM injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_jchgr](dfsdm0_jchgr) module
pub type DFSDM0_JCHGR = crate::Reg<u32, _DFSDM0_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_JCHGR;
///`read()` method returns [dfsdm0_jchgr::R](dfsdm0_jchgr::R) reader structure
impl crate::Readable for DFSDM0_JCHGR {}
///`write(|w| ..)` method takes [dfsdm0_jchgr::W](dfsdm0_jchgr::W) writer structure
impl crate::Writable for DFSDM0_JCHGR {}
///DFSDM injected channel group selection register
pub mod dfsdm0_jchgr;
///DFSDM injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_jchgr](dfsdm1_jchgr) module
pub type DFSDM1_JCHGR = crate::Reg<u32, _DFSDM1_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_JCHGR;
///`read()` method returns [dfsdm1_jchgr::R](dfsdm1_jchgr::R) reader structure
impl crate::Readable for DFSDM1_JCHGR {}
///`write(|w| ..)` method takes [dfsdm1_jchgr::W](dfsdm1_jchgr::W) writer structure
impl crate::Writable for DFSDM1_JCHGR {}
///DFSDM injected channel group selection register
pub mod dfsdm1_jchgr;
///DFSDM injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_jchgr](dfsdm2_jchgr) module
pub type DFSDM2_JCHGR = crate::Reg<u32, _DFSDM2_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_JCHGR;
///`read()` method returns [dfsdm2_jchgr::R](dfsdm2_jchgr::R) reader structure
impl crate::Readable for DFSDM2_JCHGR {}
///`write(|w| ..)` method takes [dfsdm2_jchgr::W](dfsdm2_jchgr::W) writer structure
impl crate::Writable for DFSDM2_JCHGR {}
///DFSDM injected channel group selection register
pub mod dfsdm2_jchgr;
///DFSDM injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_jchgr](dfsdm3_jchgr) module
pub type DFSDM3_JCHGR = crate::Reg<u32, _DFSDM3_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_JCHGR;
///`read()` method returns [dfsdm3_jchgr::R](dfsdm3_jchgr::R) reader structure
impl crate::Readable for DFSDM3_JCHGR {}
///`write(|w| ..)` method takes [dfsdm3_jchgr::W](dfsdm3_jchgr::W) writer structure
impl crate::Writable for DFSDM3_JCHGR {}
///DFSDM injected channel group selection register
pub mod dfsdm3_jchgr;
///DFSDM filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_fcr](dfsdm0_fcr) module
pub type DFSDM0_FCR = crate::Reg<u32, _DFSDM0_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_FCR;
///`read()` method returns [dfsdm0_fcr::R](dfsdm0_fcr::R) reader structure
impl crate::Readable for DFSDM0_FCR {}
///`write(|w| ..)` method takes [dfsdm0_fcr::W](dfsdm0_fcr::W) writer structure
impl crate::Writable for DFSDM0_FCR {}
///DFSDM filter control register
pub mod dfsdm0_fcr;
///DFSDM filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_fcr](dfsdm1_fcr) module
pub type DFSDM1_FCR = crate::Reg<u32, _DFSDM1_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_FCR;
///`read()` method returns [dfsdm1_fcr::R](dfsdm1_fcr::R) reader structure
impl crate::Readable for DFSDM1_FCR {}
///`write(|w| ..)` method takes [dfsdm1_fcr::W](dfsdm1_fcr::W) writer structure
impl crate::Writable for DFSDM1_FCR {}
///DFSDM filter control register
pub mod dfsdm1_fcr;
///DFSDM filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_fcr](dfsdm2_fcr) module
pub type DFSDM2_FCR = crate::Reg<u32, _DFSDM2_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_FCR;
///`read()` method returns [dfsdm2_fcr::R](dfsdm2_fcr::R) reader structure
impl crate::Readable for DFSDM2_FCR {}
///`write(|w| ..)` method takes [dfsdm2_fcr::W](dfsdm2_fcr::W) writer structure
impl crate::Writable for DFSDM2_FCR {}
///DFSDM filter control register
pub mod dfsdm2_fcr;
///DFSDM filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_fcr](dfsdm3_fcr) module
pub type DFSDM3_FCR = crate::Reg<u32, _DFSDM3_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_FCR;
///`read()` method returns [dfsdm3_fcr::R](dfsdm3_fcr::R) reader structure
impl crate::Readable for DFSDM3_FCR {}
///`write(|w| ..)` method takes [dfsdm3_fcr::W](dfsdm3_fcr::W) writer structure
impl crate::Writable for DFSDM3_FCR {}
///DFSDM filter control register
pub mod dfsdm3_fcr;
///DFSDM data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_jdatar](dfsdm0_jdatar) module
pub type DFSDM0_JDATAR = crate::Reg<u32, _DFSDM0_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_JDATAR;
///`read()` method returns [dfsdm0_jdatar::R](dfsdm0_jdatar::R) reader structure
impl crate::Readable for DFSDM0_JDATAR {}
///DFSDM data register for injected group
pub mod dfsdm0_jdatar;
///DFSDM data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_jdatar](dfsdm1_jdatar) module
pub type DFSDM1_JDATAR = crate::Reg<u32, _DFSDM1_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_JDATAR;
///`read()` method returns [dfsdm1_jdatar::R](dfsdm1_jdatar::R) reader structure
impl crate::Readable for DFSDM1_JDATAR {}
///DFSDM data register for injected group
pub mod dfsdm1_jdatar;
///DFSDM data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_jdatar](dfsdm2_jdatar) module
pub type DFSDM2_JDATAR = crate::Reg<u32, _DFSDM2_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_JDATAR;
///`read()` method returns [dfsdm2_jdatar::R](dfsdm2_jdatar::R) reader structure
impl crate::Readable for DFSDM2_JDATAR {}
///DFSDM data register for injected group
pub mod dfsdm2_jdatar;
///DFSDM data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_jdatar](dfsdm3_jdatar) module
pub type DFSDM3_JDATAR = crate::Reg<u32, _DFSDM3_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_JDATAR;
///`read()` method returns [dfsdm3_jdatar::R](dfsdm3_jdatar::R) reader structure
impl crate::Readable for DFSDM3_JDATAR {}
///DFSDM data register for injected group
pub mod dfsdm3_jdatar;
///DFSDM data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_rdatar](dfsdm0_rdatar) module
pub type DFSDM0_RDATAR = crate::Reg<u32, _DFSDM0_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_RDATAR;
///`read()` method returns [dfsdm0_rdatar::R](dfsdm0_rdatar::R) reader structure
impl crate::Readable for DFSDM0_RDATAR {}
///DFSDM data register for the regular channel
pub mod dfsdm0_rdatar;
///DFSDM data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_rdatar](dfsdm1_rdatar) module
pub type DFSDM1_RDATAR = crate::Reg<u32, _DFSDM1_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_RDATAR;
///`read()` method returns [dfsdm1_rdatar::R](dfsdm1_rdatar::R) reader structure
impl crate::Readable for DFSDM1_RDATAR {}
///DFSDM data register for the regular channel
pub mod dfsdm1_rdatar;
///DFSDM data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_rdatar](dfsdm2_rdatar) module
pub type DFSDM2_RDATAR = crate::Reg<u32, _DFSDM2_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_RDATAR;
///`read()` method returns [dfsdm2_rdatar::R](dfsdm2_rdatar::R) reader structure
impl crate::Readable for DFSDM2_RDATAR {}
///DFSDM data register for the regular channel
pub mod dfsdm2_rdatar;
///DFSDM data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_rdatar](dfsdm3_rdatar) module
pub type DFSDM3_RDATAR = crate::Reg<u32, _DFSDM3_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_RDATAR;
///`read()` method returns [dfsdm3_rdatar::R](dfsdm3_rdatar::R) reader structure
impl crate::Readable for DFSDM3_RDATAR {}
///DFSDM data register for the regular channel
pub mod dfsdm3_rdatar;
///DFSDM analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awhtr](dfsdm0_awhtr) module
pub type DFSDM0_AWHTR = crate::Reg<u32, _DFSDM0_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWHTR;
///`read()` method returns [dfsdm0_awhtr::R](dfsdm0_awhtr::R) reader structure
impl crate::Readable for DFSDM0_AWHTR {}
///`write(|w| ..)` method takes [dfsdm0_awhtr::W](dfsdm0_awhtr::W) writer structure
impl crate::Writable for DFSDM0_AWHTR {}
///DFSDM analog watchdog high threshold register
pub mod dfsdm0_awhtr;
///DFSDM analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awhtr](dfsdm1_awhtr) module
pub type DFSDM1_AWHTR = crate::Reg<u32, _DFSDM1_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWHTR;
///`read()` method returns [dfsdm1_awhtr::R](dfsdm1_awhtr::R) reader structure
impl crate::Readable for DFSDM1_AWHTR {}
///`write(|w| ..)` method takes [dfsdm1_awhtr::W](dfsdm1_awhtr::W) writer structure
impl crate::Writable for DFSDM1_AWHTR {}
///DFSDM analog watchdog high threshold register
pub mod dfsdm1_awhtr;
///DFSDM analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awhtr](dfsdm2_awhtr) module
pub type DFSDM2_AWHTR = crate::Reg<u32, _DFSDM2_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWHTR;
///`read()` method returns [dfsdm2_awhtr::R](dfsdm2_awhtr::R) reader structure
impl crate::Readable for DFSDM2_AWHTR {}
///`write(|w| ..)` method takes [dfsdm2_awhtr::W](dfsdm2_awhtr::W) writer structure
impl crate::Writable for DFSDM2_AWHTR {}
///DFSDM analog watchdog high threshold register
pub mod dfsdm2_awhtr;
///DFSDM analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awhtr](dfsdm3_awhtr) module
pub type DFSDM3_AWHTR = crate::Reg<u32, _DFSDM3_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWHTR;
///`read()` method returns [dfsdm3_awhtr::R](dfsdm3_awhtr::R) reader structure
impl crate::Readable for DFSDM3_AWHTR {}
///`write(|w| ..)` method takes [dfsdm3_awhtr::W](dfsdm3_awhtr::W) writer structure
impl crate::Writable for DFSDM3_AWHTR {}
///DFSDM analog watchdog high threshold register
pub mod dfsdm3_awhtr;
///DFSDM analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awltr](dfsdm0_awltr) module
pub type DFSDM0_AWLTR = crate::Reg<u32, _DFSDM0_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWLTR;
///`read()` method returns [dfsdm0_awltr::R](dfsdm0_awltr::R) reader structure
impl crate::Readable for DFSDM0_AWLTR {}
///`write(|w| ..)` method takes [dfsdm0_awltr::W](dfsdm0_awltr::W) writer structure
impl crate::Writable for DFSDM0_AWLTR {}
///DFSDM analog watchdog low threshold register
pub mod dfsdm0_awltr;
///DFSDM analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awltr](dfsdm1_awltr) module
pub type DFSDM1_AWLTR = crate::Reg<u32, _DFSDM1_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWLTR;
///`read()` method returns [dfsdm1_awltr::R](dfsdm1_awltr::R) reader structure
impl crate::Readable for DFSDM1_AWLTR {}
///`write(|w| ..)` method takes [dfsdm1_awltr::W](dfsdm1_awltr::W) writer structure
impl crate::Writable for DFSDM1_AWLTR {}
///DFSDM analog watchdog low threshold register
pub mod dfsdm1_awltr;
///DFSDM analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awltr](dfsdm2_awltr) module
pub type DFSDM2_AWLTR = crate::Reg<u32, _DFSDM2_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWLTR;
///`read()` method returns [dfsdm2_awltr::R](dfsdm2_awltr::R) reader structure
impl crate::Readable for DFSDM2_AWLTR {}
///`write(|w| ..)` method takes [dfsdm2_awltr::W](dfsdm2_awltr::W) writer structure
impl crate::Writable for DFSDM2_AWLTR {}
///DFSDM analog watchdog low threshold register
pub mod dfsdm2_awltr;
///DFSDM analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awltr](dfsdm3_awltr) module
pub type DFSDM3_AWLTR = crate::Reg<u32, _DFSDM3_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWLTR;
///`read()` method returns [dfsdm3_awltr::R](dfsdm3_awltr::R) reader structure
impl crate::Readable for DFSDM3_AWLTR {}
///`write(|w| ..)` method takes [dfsdm3_awltr::W](dfsdm3_awltr::W) writer structure
impl crate::Writable for DFSDM3_AWLTR {}
///DFSDM analog watchdog low threshold register
pub mod dfsdm3_awltr;
///DFSDM analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awsr](dfsdm0_awsr) module
pub type DFSDM0_AWSR = crate::Reg<u32, _DFSDM0_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWSR;
///`read()` method returns [dfsdm0_awsr::R](dfsdm0_awsr::R) reader structure
impl crate::Readable for DFSDM0_AWSR {}
///DFSDM analog watchdog status register
pub mod dfsdm0_awsr;
///DFSDM analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awsr](dfsdm1_awsr) module
pub type DFSDM1_AWSR = crate::Reg<u32, _DFSDM1_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWSR;
///`read()` method returns [dfsdm1_awsr::R](dfsdm1_awsr::R) reader structure
impl crate::Readable for DFSDM1_AWSR {}
///DFSDM analog watchdog status register
pub mod dfsdm1_awsr;
///DFSDM analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awsr](dfsdm2_awsr) module
pub type DFSDM2_AWSR = crate::Reg<u32, _DFSDM2_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWSR;
///`read()` method returns [dfsdm2_awsr::R](dfsdm2_awsr::R) reader structure
impl crate::Readable for DFSDM2_AWSR {}
///DFSDM analog watchdog status register
pub mod dfsdm2_awsr;
///DFSDM analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awsr](dfsdm3_awsr) module
pub type DFSDM3_AWSR = crate::Reg<u32, _DFSDM3_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWSR;
///`read()` method returns [dfsdm3_awsr::R](dfsdm3_awsr::R) reader structure
impl crate::Readable for DFSDM3_AWSR {}
///DFSDM analog watchdog status register
pub mod dfsdm3_awsr;
///DFSDM analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awcfr](dfsdm0_awcfr) module
pub type DFSDM0_AWCFR = crate::Reg<u32, _DFSDM0_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWCFR;
///`read()` method returns [dfsdm0_awcfr::R](dfsdm0_awcfr::R) reader structure
impl crate::Readable for DFSDM0_AWCFR {}
///`write(|w| ..)` method takes [dfsdm0_awcfr::W](dfsdm0_awcfr::W) writer structure
impl crate::Writable for DFSDM0_AWCFR {}
///DFSDM analog watchdog clear flag register
pub mod dfsdm0_awcfr;
///DFSDM analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awcfr](dfsdm1_awcfr) module
pub type DFSDM1_AWCFR = crate::Reg<u32, _DFSDM1_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWCFR;
///`read()` method returns [dfsdm1_awcfr::R](dfsdm1_awcfr::R) reader structure
impl crate::Readable for DFSDM1_AWCFR {}
///`write(|w| ..)` method takes [dfsdm1_awcfr::W](dfsdm1_awcfr::W) writer structure
impl crate::Writable for DFSDM1_AWCFR {}
///DFSDM analog watchdog clear flag register
pub mod dfsdm1_awcfr;
///DFSDM analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awcfr](dfsdm2_awcfr) module
pub type DFSDM2_AWCFR = crate::Reg<u32, _DFSDM2_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWCFR;
///`read()` method returns [dfsdm2_awcfr::R](dfsdm2_awcfr::R) reader structure
impl crate::Readable for DFSDM2_AWCFR {}
///`write(|w| ..)` method takes [dfsdm2_awcfr::W](dfsdm2_awcfr::W) writer structure
impl crate::Writable for DFSDM2_AWCFR {}
///DFSDM analog watchdog clear flag register
pub mod dfsdm2_awcfr;
///DFSDM analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awcfr](dfsdm3_awcfr) module
pub type DFSDM3_AWCFR = crate::Reg<u32, _DFSDM3_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWCFR;
///`read()` method returns [dfsdm3_awcfr::R](dfsdm3_awcfr::R) reader structure
impl crate::Readable for DFSDM3_AWCFR {}
///`write(|w| ..)` method takes [dfsdm3_awcfr::W](dfsdm3_awcfr::W) writer structure
impl crate::Writable for DFSDM3_AWCFR {}
///DFSDM analog watchdog clear flag register
pub mod dfsdm3_awcfr;
///DFSDM Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_exmax](dfsdm0_exmax) module
pub type DFSDM0_EXMAX = crate::Reg<u32, _DFSDM0_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_EXMAX;
///`read()` method returns [dfsdm0_exmax::R](dfsdm0_exmax::R) reader structure
impl crate::Readable for DFSDM0_EXMAX {}
///DFSDM Extremes detector maximum register
pub mod dfsdm0_exmax;
///DFSDM Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_exmax](dfsdm1_exmax) module
pub type DFSDM1_EXMAX = crate::Reg<u32, _DFSDM1_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_EXMAX;
///`read()` method returns [dfsdm1_exmax::R](dfsdm1_exmax::R) reader structure
impl crate::Readable for DFSDM1_EXMAX {}
///DFSDM Extremes detector maximum register
pub mod dfsdm1_exmax;
///DFSDM Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_exmax](dfsdm2_exmax) module
pub type DFSDM2_EXMAX = crate::Reg<u32, _DFSDM2_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_EXMAX;
///`read()` method returns [dfsdm2_exmax::R](dfsdm2_exmax::R) reader structure
impl crate::Readable for DFSDM2_EXMAX {}
///DFSDM Extremes detector maximum register
pub mod dfsdm2_exmax;
///DFSDM Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_exmax](dfsdm3_exmax) module
pub type DFSDM3_EXMAX = crate::Reg<u32, _DFSDM3_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_EXMAX;
///`read()` method returns [dfsdm3_exmax::R](dfsdm3_exmax::R) reader structure
impl crate::Readable for DFSDM3_EXMAX {}
///DFSDM Extremes detector maximum register
pub mod dfsdm3_exmax;
///DFSDM Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_exmin](dfsdm0_exmin) module
pub type DFSDM0_EXMIN = crate::Reg<u32, _DFSDM0_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_EXMIN;
///`read()` method returns [dfsdm0_exmin::R](dfsdm0_exmin::R) reader structure
impl crate::Readable for DFSDM0_EXMIN {}
///DFSDM Extremes detector minimum register
pub mod dfsdm0_exmin;
///DFSDM Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_exmin](dfsdm1_exmin) module
pub type DFSDM1_EXMIN = crate::Reg<u32, _DFSDM1_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_EXMIN;
///`read()` method returns [dfsdm1_exmin::R](dfsdm1_exmin::R) reader structure
impl crate::Readable for DFSDM1_EXMIN {}
///DFSDM Extremes detector minimum register
pub mod dfsdm1_exmin;
///DFSDM Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_exmin](dfsdm2_exmin) module
pub type DFSDM2_EXMIN = crate::Reg<u32, _DFSDM2_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_EXMIN;
///`read()` method returns [dfsdm2_exmin::R](dfsdm2_exmin::R) reader structure
impl crate::Readable for DFSDM2_EXMIN {}
///DFSDM Extremes detector minimum register
pub mod dfsdm2_exmin;
///DFSDM Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_exmin](dfsdm3_exmin) module
pub type DFSDM3_EXMIN = crate::Reg<u32, _DFSDM3_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_EXMIN;
///`read()` method returns [dfsdm3_exmin::R](dfsdm3_exmin::R) reader structure
impl crate::Readable for DFSDM3_EXMIN {}
///DFSDM Extremes detector minimum register
pub mod dfsdm3_exmin;
///DFSDM conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_cnvtimr](dfsdm0_cnvtimr) module
pub type DFSDM0_CNVTIMR = crate::Reg<u32, _DFSDM0_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CNVTIMR;
///`read()` method returns [dfsdm0_cnvtimr::R](dfsdm0_cnvtimr::R) reader structure
impl crate::Readable for DFSDM0_CNVTIMR {}
///DFSDM conversion timer register
pub mod dfsdm0_cnvtimr;
///DFSDM conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_cnvtimr](dfsdm1_cnvtimr) module
pub type DFSDM1_CNVTIMR = crate::Reg<u32, _DFSDM1_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CNVTIMR;
///`read()` method returns [dfsdm1_cnvtimr::R](dfsdm1_cnvtimr::R) reader structure
impl crate::Readable for DFSDM1_CNVTIMR {}
///DFSDM conversion timer register
pub mod dfsdm1_cnvtimr;
///DFSDM conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_cnvtimr](dfsdm2_cnvtimr) module
pub type DFSDM2_CNVTIMR = crate::Reg<u32, _DFSDM2_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CNVTIMR;
///`read()` method returns [dfsdm2_cnvtimr::R](dfsdm2_cnvtimr::R) reader structure
impl crate::Readable for DFSDM2_CNVTIMR {}
///DFSDM conversion timer register
pub mod dfsdm2_cnvtimr;
///DFSDM conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_cnvtimr](dfsdm3_cnvtimr) module
pub type DFSDM3_CNVTIMR = crate::Reg<u32, _DFSDM3_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CNVTIMR;
///`read()` method returns [dfsdm3_cnvtimr::R](dfsdm3_cnvtimr::R) reader structure
impl crate::Readable for DFSDM3_CNVTIMR {}
///DFSDM conversion timer register
pub mod dfsdm3_cnvtimr;
