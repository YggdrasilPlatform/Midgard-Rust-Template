#![deny(unused_allocation)]
#![deny(unused_comparisons)]
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
///Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_STREAM0();
    fn DMA1_STREAM1();
    fn DMA1_STREAM2();
    fn DMA1_STREAM3();
    fn DMA1_STREAM4();
    fn DMA1_STREAM5();
    fn DMA1_STREAM6();
    fn ADC();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn OTG_FS_WKUP();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn DMA1_STREAM7();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_STREAM0();
    fn DMA2_STREAM1();
    fn DMA2_STREAM2();
    fn DMA2_STREAM3();
    fn DMA2_STREAM4();
    fn ETH();
    fn ETH_WKUP();
    fn CAN2_TX();
    fn CAN2_RX0();
    fn CAN2_RX1();
    fn CAN2_SCE();
    fn OTG_FS();
    fn DMA2_STREAM5();
    fn DMA2_STREAM6();
    fn DMA2_STREAM7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn OTG_HS_EP1_OUT();
    fn OTG_HS_EP1_IN();
    fn OTG_HS_WKUP();
    fn OTG_HS();
    fn DCMI();
    fn CRYP();
    fn HASH_RNG();
    fn FPU();
    fn UART7();
    fn UART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LTDC_ER();
    fn DMA2D();
    fn SAI2();
    fn QUADSPI();
    fn LP_TIMER1();
    fn HDMI_CEC();
    fn I2C4_EV();
    fn I2C4_ER();
    fn SPDIFRX();
    fn DSIHOST();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn SDMMC2();
    fn CAN3_TX();
    fn CAN3_RX0();
    fn CAN3_RX1();
    fn CAN3_SCE();
    fn JPEG();
    fn MDIOS();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 110] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_STREAM0,
    },
    Vector {
        _handler: DMA1_STREAM1,
    },
    Vector {
        _handler: DMA1_STREAM2,
    },
    Vector {
        _handler: DMA1_STREAM3,
    },
    Vector {
        _handler: DMA1_STREAM4,
    },
    Vector {
        _handler: DMA1_STREAM5,
    },
    Vector {
        _handler: DMA1_STREAM6,
    },
    Vector { _handler: ADC },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM9,
    },
    Vector {
        _handler: TIM1_UP_TIM10,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: OTG_FS_WKUP,
    },
    Vector {
        _handler: TIM8_BRK_TIM12,
    },
    Vector {
        _handler: TIM8_UP_TIM13,
    },
    Vector {
        _handler: TIM8_TRG_COM_TIM14,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_STREAM7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_STREAM0,
    },
    Vector {
        _handler: DMA2_STREAM1,
    },
    Vector {
        _handler: DMA2_STREAM2,
    },
    Vector {
        _handler: DMA2_STREAM3,
    },
    Vector {
        _handler: DMA2_STREAM4,
    },
    Vector { _handler: ETH },
    Vector { _handler: ETH_WKUP },
    Vector { _handler: CAN2_TX },
    Vector { _handler: CAN2_RX0 },
    Vector { _handler: CAN2_RX1 },
    Vector { _handler: CAN2_SCE },
    Vector { _handler: OTG_FS },
    Vector {
        _handler: DMA2_STREAM5,
    },
    Vector {
        _handler: DMA2_STREAM6,
    },
    Vector {
        _handler: DMA2_STREAM7,
    },
    Vector { _handler: USART6 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: OTG_HS_EP1_OUT,
    },
    Vector {
        _handler: OTG_HS_EP1_IN,
    },
    Vector {
        _handler: OTG_HS_WKUP,
    },
    Vector { _handler: OTG_HS },
    Vector { _handler: DCMI },
    Vector { _handler: CRYP },
    Vector { _handler: HASH_RNG },
    Vector { _handler: FPU },
    Vector { _handler: UART7 },
    Vector { _handler: UART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LTDC },
    Vector { _handler: LTDC_ER },
    Vector { _handler: DMA2D },
    Vector { _handler: SAI2 },
    Vector { _handler: QUADSPI },
    Vector {
        _handler: LP_TIMER1,
    },
    Vector { _handler: HDMI_CEC },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: SPDIFRX },
    Vector { _handler: DSIHOST },
    Vector {
        _handler: DFSDM1_FLT0,
    },
    Vector {
        _handler: DFSDM1_FLT1,
    },
    Vector {
        _handler: DFSDM1_FLT2,
    },
    Vector {
        _handler: DFSDM1_FLT3,
    },
    Vector { _handler: SDMMC2 },
    Vector { _handler: CAN3_TX },
    Vector { _handler: CAN3_RX0 },
    Vector { _handler: CAN3_RX1 },
    Vector { _handler: CAN3_SCE },
    Vector { _handler: JPEG },
    Vector { _handler: MDIOS },
];
///Enumeration of all the interrupts
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    ///0 - Window Watchdog interrupt
    WWDG = 0,
    ///1 - PVD through EXTI line detection
    PVD = 1,
    ///2 - Tamper and TimeStamp interrupts through the EXTI line
    TAMP_STAMP = 2,
    ///3 - RTC Tamper or TimeStamp /CSS on LSE through EXTI line 19 interrupts
    RTC_WKUP = 3,
    ///4 - Flash global interrupt
    FLASH = 4,
    ///5 - RCC global interrupt
    RCC = 5,
    ///6 - EXTI Line0 interrupt
    EXTI0 = 6,
    ///7 - EXTI Line1 interrupt
    EXTI1 = 7,
    ///8 - EXTI Line2 interrupt
    EXTI2 = 8,
    ///9 - EXTI Line3 interrupt
    EXTI3 = 9,
    ///10 - EXTI Line4 interrupt
    EXTI4 = 10,
    ///11 - DMA1 Stream0 global interrupt
    DMA1_STREAM0 = 11,
    ///12 - DMA1 Stream1 global interrupt
    DMA1_STREAM1 = 12,
    ///13 - DMA1 Stream2 global interrupt
    DMA1_STREAM2 = 13,
    ///14 - DMA1 Stream3 global interrupt
    DMA1_STREAM3 = 14,
    ///15 - DMA1 Stream4 global interrupt
    DMA1_STREAM4 = 15,
    ///16 - DMA1 Stream5 global interrupt
    DMA1_STREAM5 = 16,
    ///17 - DMA1 Stream6 global interrupt
    DMA1_STREAM6 = 17,
    ///18 - ADC1 global interrupt
    ADC = 18,
    ///19 - CAN1 TX interrupts
    CAN1_TX = 19,
    ///20 - CAN1 RX0 interrupts
    CAN1_RX0 = 20,
    ///21 - CAN1 RX1 interrupts
    CAN1_RX1 = 21,
    ///22 - CAN1 SCE interrupt
    CAN1_SCE = 22,
    ///23 - EXTI Line\[9:5\]
    ///interrupts
    EXTI9_5 = 23,
    ///24 - TIM1 Break interrupt and TIM9 global interrupt
    TIM1_BRK_TIM9 = 24,
    ///25 - TIM1_UP_TIM10
    TIM1_UP_TIM10 = 25,
    ///26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
    TIM1_TRG_COM_TIM11 = 26,
    ///27 - TIM1_CC
    TIM1_CC = 27,
    ///28 - TIM2 global interrupt
    TIM2 = 28,
    ///29 - TIM3 global interrupt
    TIM3 = 29,
    ///30 - TIM4 global interrupt
    TIM4 = 30,
    ///31 - I2C1 event interrupt
    I2C1_EV = 31,
    ///32 - I2C1 error interrupt
    I2C1_ER = 32,
    ///33 - I2C2_EV
    I2C2_EV = 33,
    ///34 - I2C2_ER
    I2C2_ER = 34,
    ///35 - SPI1 global interrupt
    SPI1 = 35,
    ///36 - SPI2 global interrupt
    SPI2 = 36,
    ///37 - USART1 global interrupt
    USART1 = 37,
    ///38 - USART2 global interrupt
    USART2 = 38,
    ///39 - USART3 global interrupt
    USART3 = 39,
    ///40 - EXTI Line\[15:10\]
    ///interrupts
    EXTI15_10 = 40,
    ///41 - RTC alarms through EXTI line 18 interrupts
    RTC_ALARM = 41,
    ///42 - USB On-The-Go FS Wakeup through EXTI line interrupt
    OTG_FS_WKUP = 42,
    ///43 - TIM8 Break interrupt and TIM12 global interrupt
    TIM8_BRK_TIM12 = 43,
    ///44 - TIM8 Update interrupt and TIM13 global interrupt
    TIM8_UP_TIM13 = 44,
    ///45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
    TIM8_TRG_COM_TIM14 = 45,
    ///46 - TIM8 Capture Compare interrupt
    TIM8_CC = 46,
    ///47 - DMA1 Stream7 global interrupt
    DMA1_STREAM7 = 47,
    ///48 - FMC global interrupt
    FMC = 48,
    ///49 - SDMMC1 global interrupt
    SDMMC1 = 49,
    ///50 - TIM5 global interrupt
    TIM5 = 50,
    ///51 - SPI3 global interrupt
    SPI3 = 51,
    ///52 - UART4 global interrupt
    UART4 = 52,
    ///53 - UART5 global interrupt
    UART5 = 53,
    ///54 - TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt
    TIM6_DAC = 54,
    ///55 - TIM7 global interrupt
    TIM7 = 55,
    ///56 - DMA2 Stream0 global interrupt
    DMA2_STREAM0 = 56,
    ///57 - DMA2 Stream1 global interrupt
    DMA2_STREAM1 = 57,
    ///58 - DMA2 Stream2 global interrupt
    DMA2_STREAM2 = 58,
    ///59 - DMA2 Stream3 global interrupt
    DMA2_STREAM3 = 59,
    ///60 - DMA2 Stream4 global interrupt
    DMA2_STREAM4 = 60,
    ///61 - Ethernet global interrupt
    ETH = 61,
    ///62 - Ethernet Wakeup through EXTI line
    ETH_WKUP = 62,
    ///63 - CAN2 TX interrupts
    CAN2_TX = 63,
    ///64 - CAN2 RX0 interrupts
    CAN2_RX0 = 64,
    ///65 - CAN2 RX1 interrupts
    CAN2_RX1 = 65,
    ///66 - CAN2 SCE interrupt
    CAN2_SCE = 66,
    ///67 - USB On The Go FS global interrupt
    OTG_FS = 67,
    ///68 - DMA2 Stream5 global interrupt
    DMA2_STREAM5 = 68,
    ///69 - DMA2 Stream6 global interrupt
    DMA2_STREAM6 = 69,
    ///70 - DMA2 Stream7 global interrupt
    DMA2_STREAM7 = 70,
    ///71 - USART6 global interrupt
    USART6 = 71,
    ///72 - I2C3 event interrupt
    I2C3_EV = 72,
    ///73 - I2C3 error interrupt
    I2C3_ER = 73,
    ///74 - USB On The Go HS End Point 1 Out global interrupt
    OTG_HS_EP1_OUT = 74,
    ///75 - USB On The Go HS End Point 1 In global interrupt
    OTG_HS_EP1_IN = 75,
    ///76 - USB On The Go HS Wakeup through EXTI interrupt
    OTG_HS_WKUP = 76,
    ///77 - USB On The Go HS global interrupt
    OTG_HS = 77,
    ///78 - DCMI global interrupt
    DCMI = 78,
    ///79 - CRYP
    CRYP = 79,
    ///80 - Hash and Rng global interrupt
    HASH_RNG = 80,
    ///81 - Floating point unit interrupt
    FPU = 81,
    ///82 - UART 7 global interrupt
    UART7 = 82,
    ///83 - UART 8 global interrupt
    UART8 = 83,
    ///84 - SPI 4 global interrupt
    SPI4 = 84,
    ///85 - SPI 5 global interrupt
    SPI5 = 85,
    ///86 - SPI 6 global interrupt
    SPI6 = 86,
    ///87 - SAI1 global interrupt
    SAI1 = 87,
    ///88 - LTDC global interrupt
    LTDC = 88,
    ///89 - LTDC global error interrupt
    LTDC_ER = 89,
    ///90 - DMA2D global interrupt
    DMA2D = 90,
    ///91 - SAI2 global interrupt
    SAI2 = 91,
    ///92 - QuadSPI global interrupt
    QUADSPI = 92,
    ///93 - LP Timer1 global interrupt
    LP_TIMER1 = 93,
    ///94 - HDMI-CEC global interrupt
    HDMI_CEC = 94,
    ///95 - I2C4 event interrupt
    I2C4_EV = 95,
    ///96 - I2C4 Error interrupt
    I2C4_ER = 96,
    ///97 - SPDIFRX global interrupt
    SPDIFRX = 97,
    ///98 - DSI host global interrupt
    DSIHOST = 98,
    ///99 - DFSDM1 Filter 0 global interrupt
    DFSDM1_FLT0 = 99,
    ///100 - DFSDM1 Filter 1 global interrupt
    DFSDM1_FLT1 = 100,
    ///101 - DFSDM1 Filter 2 global interrupt
    DFSDM1_FLT2 = 101,
    ///102 - DFSDM1 Filter 3 global interrupt
    DFSDM1_FLT3 = 102,
    ///103 - SDMMC2 global interrupt
    SDMMC2 = 103,
    ///104 - CAN3 TX interrupt
    CAN3_TX = 104,
    ///105 - CAN3 RX0 interrupt
    CAN3_RX0 = 105,
    ///106 - CAN3 RX1 interrupt
    CAN3_RX1 = 106,
    ///107 - CAN3 SCE interrupt
    CAN3_SCE = 107,
    ///108 - JPEG global interrupt
    JPEG = 108,
    ///109 - MDIO slave global interrupt
    MDIOS = 109,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
///Random number generator
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x5006_0800 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
///Random number generator
pub mod rng;
///Hash processor
pub struct HASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH {}
impl HASH {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const hash::RegisterBlock {
        0x5006_0400 as *const _
    }
}
impl Deref for HASH {
    type Target = hash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASH::ptr() }
    }
}
///Hash processor
pub mod hash;
///Cryptographic processor
pub struct CRYP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYP {}
impl CRYP {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const cryp::RegisterBlock {
        0x5006_0000 as *const _
    }
}
impl Deref for CRYP {
    type Target = cryp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYP::ptr() }
    }
}
///Cryptographic processor
pub mod cryp;
///Digital camera interface
pub struct DCMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCMI {}
impl DCMI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dcmi::RegisterBlock {
        0x5005_0000 as *const _
    }
}
impl Deref for DCMI {
    type Target = dcmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCMI::ptr() }
    }
}
///Digital camera interface
pub mod dcmi;
///Flexible memory controller
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0xa000_0000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
///Flexible memory controller
pub mod fmc;
///DMA controller
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma2::RegisterBlock {
        0x4002_6400 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
///DMA controller
pub mod dma2;
///DMA controller
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma2::RegisterBlock {
        0x4002_6000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
///Reset and clock control
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x4002_3800 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
///Reset and clock control
pub mod rcc;
///General-purpose I/Os
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_0c00 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
///General-purpose I/Os
pub mod gpiod;
///General-purpose I/Os
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOK {}
impl GPIOK {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_2800 as *const _
    }
}
impl Deref for GPIOK {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOK::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOJ {}
impl GPIOJ {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_2400 as *const _
    }
}
impl Deref for GPIOJ {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOJ::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOI::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_1c00 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_1800 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_1400 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
///General-purpose I/Os
pub mod gpiob;
///General-purpose I/Os
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
///General-purpose I/Os
pub mod gpioa;
///System configuration controller
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
///System configuration controller
pub mod syscfg;
///Serial peripheral interface
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
///Serial peripheral interface
pub mod spi1;
///Serial peripheral interface
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
///Serial peripheral interface
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
///Serial peripheral interface
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3400 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI4::ptr() }
    }
}
///Serial peripheral interface
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI5::ptr() }
    }
}
///Serial peripheral interface
pub struct SPI6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI6 {}
impl SPI6 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_5400 as *const _
    }
}
impl Deref for SPI6 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI6::ptr() }
    }
}
///Analog-to-digital converter
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
///Analog-to-digital converter
pub mod adc1;
///Analog-to-digital converter
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4001_2100 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC2::ptr() }
    }
}
///Analog-to-digital converter
pub struct ADC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC3 {}
impl ADC3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4001_2200 as *const _
    }
}
impl Deref for ADC3 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC3::ptr() }
    }
}
///Digital-to-analog converter
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4000_7400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
///Digital-to-analog converter
pub mod dac;
///Power control
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
///Power control
pub mod pwr;
///Independent watchdog
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG::ptr() }
    }
}
///Independent watchdog
pub mod iwdg;
///Window watchdog
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG::ptr() }
    }
}
///Window watchdog
pub mod wwdg;
///Advanced-timers
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
///Advanced-timers
pub mod tim1;
///Advanced-timers
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM8::ptr() }
    }
}
///General purpose timers
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM2::ptr() }
    }
}
///General purpose timers
pub mod tim2;
///General purpose timers
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim3::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM3::ptr() }
    }
}
///General purpose timers
pub mod tim3;
///General purpose timers
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim4::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM4::ptr() }
    }
}
///General purpose timers
pub mod tim4;
///General purpose timers
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim4::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
///General purpose timers
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM9 {}
impl TIM9 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim9::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TIM9 {
    type Target = tim9::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM9::ptr() }
    }
}
///General purpose timers
pub mod tim9;
///General purpose timers
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM12 {}
impl TIM12 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim9::RegisterBlock {
        0x4000_1800 as *const _
    }
}
impl Deref for TIM12 {
    type Target = tim9::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM12::ptr() }
    }
}
///General-purpose-timers
pub struct TIM10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM10 {}
impl TIM10 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim10::RegisterBlock {
        0x4001_4400 as *const _
    }
}
impl Deref for TIM10 {
    type Target = tim10::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM10::ptr() }
    }
}
///General-purpose-timers
pub mod tim10;
///General-purpose-timers
pub struct TIM11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM11 {}
impl TIM11 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim10::RegisterBlock {
        0x4001_4800 as *const _
    }
}
impl Deref for TIM11 {
    type Target = tim10::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM11::ptr() }
    }
}
///General-purpose-timers
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM13 {}
impl TIM13 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim10::RegisterBlock {
        0x4000_1c00 as *const _
    }
}
impl Deref for TIM13 {
    type Target = tim10::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM13::ptr() }
    }
}
///General-purpose-timers
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim10::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim10::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM14::ptr() }
    }
}
///Basic timers
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM6::ptr() }
    }
}
///Basic timers
pub mod tim6;
///Basic timers
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM7::ptr() }
    }
}
///Ethernet: media access control (MAC)
pub struct ETHERNET_MAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MAC {}
impl ETHERNET_MAC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_mac::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for ETHERNET_MAC {
    type Target = ethernet_mac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETHERNET_MAC::ptr() }
    }
}
///Ethernet: media access control (MAC)
pub mod ethernet_mac;
///Cryptographic processor
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
///Cryptographic processor
pub mod crc;
///Controller area network
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4000_6400 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
///Controller area network
pub mod can1;
///Controller area network
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4000_6800 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN2::ptr() }
    }
}
///Controller area network
pub struct CAN3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN3 {}
impl CAN3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4000_3400 as *const _
    }
}
impl Deref for CAN3 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN3::ptr() }
    }
}
///FLASH
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4002_3c00 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
///FLASH
pub mod flash;
///External interrupt/event controller
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4001_3c00 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
///External interrupt/event controller
pub mod exti;
///LCD-TFT Controller
pub struct LTDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTDC {}
impl LTDC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ltdc::RegisterBlock {
        0x4001_6800 as *const _
    }
}
impl Deref for LTDC {
    type Target = ltdc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LTDC::ptr() }
    }
}
///LCD-TFT Controller
pub mod ltdc;
///Serial audio interface
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5800 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI1::ptr() }
    }
}
///Serial audio interface
pub mod sai1;
///Serial audio interface
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5c00 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI2::ptr() }
    }
}
///DMA2D controller
pub struct DMA2D {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2D {}
impl DMA2D {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma2d::RegisterBlock {
        0x4002_b000 as *const _
    }
}
impl Deref for DMA2D {
    type Target = dma2d::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2D::ptr() }
    }
}
///DMA2D controller
pub mod dma2d;
///QuadSPI interface
pub struct QUADSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI {}
impl QUADSPI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const quadspi::RegisterBlock {
        0xa000_1000 as *const _
    }
}
impl Deref for QUADSPI {
    type Target = quadspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QUADSPI::ptr() }
    }
}
///QuadSPI interface
pub mod quadspi;
///HDMI-CEC controller
pub struct CEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CEC {}
impl CEC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const cec::RegisterBlock {
        0x4000_6c00 as *const _
    }
}
impl Deref for CEC {
    type Target = cec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CEC::ptr() }
    }
}
///HDMI-CEC controller
pub mod cec;
///Receiver Interface
pub struct SPDIFRX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPDIFRX {}
impl SPDIFRX {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spdifrx::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SPDIFRX {
    type Target = spdifrx::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPDIFRX::ptr() }
    }
}
///Receiver Interface
pub mod spdifrx;
///Secure digital input/output interface
pub struct SDMMC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC1 {}
impl SDMMC1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x4001_2c00 as *const _
    }
}
impl Deref for SDMMC1 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC1::ptr() }
    }
}
///Secure digital input/output interface
pub mod sdmmc1;
///Secure digital input/output interface
pub struct SDMMC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC2 {}
impl SDMMC2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x4001_1c00 as *const _
    }
}
impl Deref for SDMMC2 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC2::ptr() }
    }
}
///Low power timer
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_2400 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM1::ptr() }
    }
}
///Low power timer
pub mod lptim1;
///Inter-integrated circuit
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
///Inter-integrated circuit
pub mod i2c1;
///Inter-integrated circuit
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
///Inter-integrated circuit
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5c00 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
///Inter-integrated circuit
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
///Real-time clock
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
///Real-time clock
pub mod rtc;
///Universal synchronous asynchronous receiver transmitter
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1400 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART6::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub mod usart1;
///Universal synchronous asynchronous receiver transmitter
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for UART5 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART5::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4c00 as *const _
    }
}
impl Deref for UART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct UART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART8 {}
impl UART8 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_7c00 as *const _
    }
}
impl Deref for UART8 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART8::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_7800 as *const _
    }
}
impl Deref for UART7 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART7::ptr() }
    }
}
///USB on the go full speed
pub struct OTG_FS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_GLOBAL {}
impl OTG_FS_GLOBAL {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_global::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for OTG_FS_GLOBAL {
    type Target = otg_fs_global::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_GLOBAL::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_global;
///USB on the go high speed
pub struct OTG_HS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_GLOBAL {}
impl OTG_HS_GLOBAL {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_hs_global::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for OTG_HS_GLOBAL {
    type Target = otg_hs_global::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_HS_GLOBAL::ptr() }
    }
}
///USB on the go high speed
pub mod otg_hs_global;
///Management data input/output slave
pub struct MDIOS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDIOS {}
impl MDIOS {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const mdios::RegisterBlock {
        0x4001_7800 as *const _
    }
}
impl Deref for MDIOS {
    type Target = mdios::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MDIOS::ptr() }
    }
}
///Management data input/output slave
pub mod mdios;
///Digital filter for sigma delta modulators
pub struct DFSDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM {}
impl DFSDM {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dfsdm::RegisterBlock {
        0x4001_7400 as *const _
    }
}
impl Deref for DFSDM {
    type Target = dfsdm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DFSDM::ptr() }
    }
}
///Digital filter for sigma delta modulators
pub mod dfsdm;
///JPEG codec
pub struct JPEG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for JPEG {}
impl JPEG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const jpeg::RegisterBlock {
        0x5005_1000 as *const _
    }
}
impl Deref for JPEG {
    type Target = jpeg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*JPEG::ptr() }
    }
}
///JPEG codec
pub mod jpeg;
///Ethernet: MAC management counters
pub struct ETHERNET_MMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MMC {}
impl ETHERNET_MMC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_mmc::RegisterBlock {
        0x4002_8100 as *const _
    }
}
impl Deref for ETHERNET_MMC {
    type Target = ethernet_mmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETHERNET_MMC::ptr() }
    }
}
///Ethernet: MAC management counters
pub mod ethernet_mmc;
///Ethernet: Precision time protocol
pub struct ETHERNET_PTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_PTP {}
impl ETHERNET_PTP {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_ptp::RegisterBlock {
        0x4002_8700 as *const _
    }
}
impl Deref for ETHERNET_PTP {
    type Target = ethernet_ptp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETHERNET_PTP::ptr() }
    }
}
///Ethernet: Precision time protocol
pub mod ethernet_ptp;
///Ethernet: DMA controller operation
pub struct ETHERNET_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_DMA {}
impl ETHERNET_DMA {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_dma::RegisterBlock {
        0x4002_9000 as *const _
    }
}
impl Deref for ETHERNET_DMA {
    type Target = ethernet_dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETHERNET_DMA::ptr() }
    }
}
///Ethernet: DMA controller operation
pub mod ethernet_dma;
///USB on the go full speed
pub struct OTG_FS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_HOST {}
impl OTG_FS_HOST {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_host::RegisterBlock {
        0x5000_0400 as *const _
    }
}
impl Deref for OTG_FS_HOST {
    type Target = otg_fs_host::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_HOST::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_host;
///USB on the go full speed
pub struct OTG_FS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_DEVICE {}
impl OTG_FS_DEVICE {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_device::RegisterBlock {
        0x5000_0800 as *const _
    }
}
impl Deref for OTG_FS_DEVICE {
    type Target = otg_fs_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_DEVICE::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_device;
///USB on the go full speed
pub struct OTG_FS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_PWRCLK {}
impl OTG_FS_PWRCLK {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_pwrclk::RegisterBlock {
        0x5000_0e00 as *const _
    }
}
impl Deref for OTG_FS_PWRCLK {
    type Target = otg_fs_pwrclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_PWRCLK::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_pwrclk;
///USB on the go high speed
pub struct OTG_HS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_HOST {}
impl OTG_HS_HOST {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_hs_host::RegisterBlock {
        0x4004_0400 as *const _
    }
}
impl Deref for OTG_HS_HOST {
    type Target = otg_hs_host::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_HS_HOST::ptr() }
    }
}
///USB on the go high speed
pub mod otg_hs_host;
///USB on the go high speed
pub struct OTG_HS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_DEVICE {}
impl OTG_HS_DEVICE {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_hs_device::RegisterBlock {
        0x4004_0800 as *const _
    }
}
impl Deref for OTG_HS_DEVICE {
    type Target = otg_hs_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_HS_DEVICE::ptr() }
    }
}
///USB on the go high speed
pub mod otg_hs_device;
///USB on the go high speed
pub struct OTG_HS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_PWRCLK {}
impl OTG_HS_PWRCLK {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_hs_pwrclk::RegisterBlock {
        0x4004_0e00 as *const _
    }
}
impl Deref for OTG_HS_PWRCLK {
    type Target = otg_hs_pwrclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_HS_PWRCLK::ptr() }
    }
}
///USB on the go high speed
pub mod otg_hs_pwrclk;
///DSI Host
pub struct DSI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSI {}
impl DSI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dsi::RegisterBlock {
        0x4001_6c00 as *const _
    }
}
impl Deref for DSI {
    type Target = dsi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSI::ptr() }
    }
}
///DSI Host
pub mod dsi;
///SysTick timer
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const stk::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STK::ptr() }
    }
}
///SysTick timer
pub mod stk;
///Nested vectored interrupt controller
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const nvic_stir::RegisterBlock {
        0xe000_ef00 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
///Nested vectored interrupt controller
pub mod nvic_stir;
///Floating point unit CPACR
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fpu_cpacr::RegisterBlock {
        0xe000_ed88 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
///Floating point unit CPACR
pub mod fpu_cpacr;
///System control block ACTLR
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const scb_actrl::RegisterBlock {
        0xe000_e008 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
///System control block ACTLR
pub mod scb_actrl;
///Processor features
pub struct PF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PF {}
impl PF {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pf::RegisterBlock {
        0xe000_ed78 as *const _
    }
}
impl Deref for PF {
    type Target = pf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PF::ptr() }
    }
}
///Processor features
pub mod pf;
///Access control
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ac::RegisterBlock {
        0xe000_ef90 as *const _
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AC::ptr() }
    }
}
///Access control
pub mod ac;
///ADC common registers
pub struct ADC_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC_COMMON {}
impl ADC_COMMON {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc_common::RegisterBlock {
        0x4001_2300 as *const _
    }
}
impl Deref for ADC_COMMON {
    type Target = adc_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC_COMMON::ptr() }
    }
}
///ADC common registers
pub mod adc_common;
///Debug support
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
///Debug support
pub mod dbgmcu;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
///All the peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    ///RNG
    pub RNG: RNG,
    ///HASH
    pub HASH: HASH,
    ///CRYP
    pub CRYP: CRYP,
    ///DCMI
    pub DCMI: DCMI,
    ///FMC
    pub FMC: FMC,
    ///DMA2
    pub DMA2: DMA2,
    ///DMA1
    pub DMA1: DMA1,
    ///RCC
    pub RCC: RCC,
    ///GPIOD
    pub GPIOD: GPIOD,
    ///GPIOC
    pub GPIOC: GPIOC,
    ///GPIOK
    pub GPIOK: GPIOK,
    ///GPIOJ
    pub GPIOJ: GPIOJ,
    ///GPIOI
    pub GPIOI: GPIOI,
    ///GPIOH
    pub GPIOH: GPIOH,
    ///GPIOG
    pub GPIOG: GPIOG,
    ///GPIOF
    pub GPIOF: GPIOF,
    ///GPIOE
    pub GPIOE: GPIOE,
    ///GPIOB
    pub GPIOB: GPIOB,
    ///GPIOA
    pub GPIOA: GPIOA,
    ///SYSCFG
    pub SYSCFG: SYSCFG,
    ///SPI1
    pub SPI1: SPI1,
    ///SPI2
    pub SPI2: SPI2,
    ///SPI3
    pub SPI3: SPI3,
    ///SPI4
    pub SPI4: SPI4,
    ///SPI5
    pub SPI5: SPI5,
    ///SPI6
    pub SPI6: SPI6,
    ///ADC1
    pub ADC1: ADC1,
    ///ADC2
    pub ADC2: ADC2,
    ///ADC3
    pub ADC3: ADC3,
    ///DAC
    pub DAC: DAC,
    ///PWR
    pub PWR: PWR,
    ///IWDG
    pub IWDG: IWDG,
    ///WWDG
    pub WWDG: WWDG,
    ///TIM1
    pub TIM1: TIM1,
    ///TIM8
    pub TIM8: TIM8,
    ///TIM2
    pub TIM2: TIM2,
    ///TIM3
    pub TIM3: TIM3,
    ///TIM4
    pub TIM4: TIM4,
    ///TIM5
    pub TIM5: TIM5,
    ///TIM9
    pub TIM9: TIM9,
    ///TIM12
    pub TIM12: TIM12,
    ///TIM10
    pub TIM10: TIM10,
    ///TIM11
    pub TIM11: TIM11,
    ///TIM13
    pub TIM13: TIM13,
    ///TIM14
    pub TIM14: TIM14,
    ///TIM6
    pub TIM6: TIM6,
    ///TIM7
    pub TIM7: TIM7,
    ///ETHERNET_MAC
    pub ETHERNET_MAC: ETHERNET_MAC,
    ///CRC
    pub CRC: CRC,
    ///CAN1
    pub CAN1: CAN1,
    ///CAN2
    pub CAN2: CAN2,
    ///CAN3
    pub CAN3: CAN3,
    ///FLASH
    pub FLASH: FLASH,
    ///EXTI
    pub EXTI: EXTI,
    ///LTDC
    pub LTDC: LTDC,
    ///SAI1
    pub SAI1: SAI1,
    ///SAI2
    pub SAI2: SAI2,
    ///DMA2D
    pub DMA2D: DMA2D,
    ///QUADSPI
    pub QUADSPI: QUADSPI,
    ///CEC
    pub CEC: CEC,
    ///SPDIFRX
    pub SPDIFRX: SPDIFRX,
    ///SDMMC1
    pub SDMMC1: SDMMC1,
    ///SDMMC2
    pub SDMMC2: SDMMC2,
    ///LPTIM1
    pub LPTIM1: LPTIM1,
    ///I2C1
    pub I2C1: I2C1,
    ///I2C2
    pub I2C2: I2C2,
    ///I2C3
    pub I2C3: I2C3,
    ///I2C4
    pub I2C4: I2C4,
    ///RTC
    pub RTC: RTC,
    ///USART6
    pub USART6: USART6,
    ///USART1
    pub USART1: USART1,
    ///USART3
    pub USART3: USART3,
    ///USART2
    pub USART2: USART2,
    ///UART5
    pub UART5: UART5,
    ///UART4
    pub UART4: UART4,
    ///UART8
    pub UART8: UART8,
    ///UART7
    pub UART7: UART7,
    ///OTG_FS_GLOBAL
    pub OTG_FS_GLOBAL: OTG_FS_GLOBAL,
    ///OTG_HS_GLOBAL
    pub OTG_HS_GLOBAL: OTG_HS_GLOBAL,
    ///MDIOS
    pub MDIOS: MDIOS,
    ///DFSDM
    pub DFSDM: DFSDM,
    ///JPEG
    pub JPEG: JPEG,
    ///ETHERNET_MMC
    pub ETHERNET_MMC: ETHERNET_MMC,
    ///ETHERNET_PTP
    pub ETHERNET_PTP: ETHERNET_PTP,
    ///ETHERNET_DMA
    pub ETHERNET_DMA: ETHERNET_DMA,
    ///OTG_FS_HOST
    pub OTG_FS_HOST: OTG_FS_HOST,
    ///OTG_FS_DEVICE
    pub OTG_FS_DEVICE: OTG_FS_DEVICE,
    ///OTG_FS_PWRCLK
    pub OTG_FS_PWRCLK: OTG_FS_PWRCLK,
    ///OTG_HS_HOST
    pub OTG_HS_HOST: OTG_HS_HOST,
    ///OTG_HS_DEVICE
    pub OTG_HS_DEVICE: OTG_HS_DEVICE,
    ///OTG_HS_PWRCLK
    pub OTG_HS_PWRCLK: OTG_HS_PWRCLK,
    ///DSI
    pub DSI: DSI,
    ///STK
    pub STK: STK,
    ///NVIC_STIR
    pub NVIC_STIR: NVIC_STIR,
    ///FPU_CPACR
    pub FPU_CPACR: FPU_CPACR,
    ///SCB_ACTRL
    pub SCB_ACTRL: SCB_ACTRL,
    ///PF
    pub PF: PF,
    ///AC
    pub AC: AC,
    ///ADC_COMMON
    pub ADC_COMMON: ADC_COMMON,
    ///DBGMCU
    pub DBGMCU: DBGMCU,
}
impl Peripherals {
    ///Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    ///Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            RNG: RNG {
                _marker: PhantomData,
            },
            HASH: HASH {
                _marker: PhantomData,
            },
            CRYP: CRYP {
                _marker: PhantomData,
            },
            DCMI: DCMI {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOK: GPIOK {
                _marker: PhantomData,
            },
            GPIOJ: GPIOJ {
                _marker: PhantomData,
            },
            GPIOI: GPIOI {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SPI4: SPI4 {
                _marker: PhantomData,
            },
            SPI5: SPI5 {
                _marker: PhantomData,
            },
            SPI6: SPI6 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC3: ADC3 {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM9: TIM9 {
                _marker: PhantomData,
            },
            TIM12: TIM12 {
                _marker: PhantomData,
            },
            TIM10: TIM10 {
                _marker: PhantomData,
            },
            TIM11: TIM11 {
                _marker: PhantomData,
            },
            TIM13: TIM13 {
                _marker: PhantomData,
            },
            TIM14: TIM14 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            ETHERNET_MAC: ETHERNET_MAC {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            CAN3: CAN3 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            LTDC: LTDC {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            DMA2D: DMA2D {
                _marker: PhantomData,
            },
            QUADSPI: QUADSPI {
                _marker: PhantomData,
            },
            CEC: CEC {
                _marker: PhantomData,
            },
            SPDIFRX: SPDIFRX {
                _marker: PhantomData,
            },
            SDMMC1: SDMMC1 {
                _marker: PhantomData,
            },
            SDMMC2: SDMMC2 {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART8: UART8 {
                _marker: PhantomData,
            },
            UART7: UART7 {
                _marker: PhantomData,
            },
            OTG_FS_GLOBAL: OTG_FS_GLOBAL {
                _marker: PhantomData,
            },
            OTG_HS_GLOBAL: OTG_HS_GLOBAL {
                _marker: PhantomData,
            },
            MDIOS: MDIOS {
                _marker: PhantomData,
            },
            DFSDM: DFSDM {
                _marker: PhantomData,
            },
            JPEG: JPEG {
                _marker: PhantomData,
            },
            ETHERNET_MMC: ETHERNET_MMC {
                _marker: PhantomData,
            },
            ETHERNET_PTP: ETHERNET_PTP {
                _marker: PhantomData,
            },
            ETHERNET_DMA: ETHERNET_DMA {
                _marker: PhantomData,
            },
            OTG_FS_HOST: OTG_FS_HOST {
                _marker: PhantomData,
            },
            OTG_FS_DEVICE: OTG_FS_DEVICE {
                _marker: PhantomData,
            },
            OTG_FS_PWRCLK: OTG_FS_PWRCLK {
                _marker: PhantomData,
            },
            OTG_HS_HOST: OTG_HS_HOST {
                _marker: PhantomData,
            },
            OTG_HS_DEVICE: OTG_HS_DEVICE {
                _marker: PhantomData,
            },
            OTG_HS_PWRCLK: OTG_HS_PWRCLK {
                _marker: PhantomData,
            },
            DSI: DSI {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
            PF: PF {
                _marker: PhantomData,
            },
            AC: AC {
                _marker: PhantomData,
            },
            ADC_COMMON: ADC_COMMON {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
        }
    }
}
