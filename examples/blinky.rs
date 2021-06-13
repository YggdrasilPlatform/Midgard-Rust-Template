#![no_main]
#![no_std]
#![allow(unused_imports)]

extern crate panic_halt;

use stm32f7;
use stm32f7xx_hal::{delay::Delay, pac, prelude::*};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Take peripheral interfaces
    let peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    
    // Set system clock to 216MHz and create delay function
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);
    
    // Get Heartbeat LED (PA10)
    let gpioa = peripherals.GPIOA.split();
    let mut heartbeat_led = gpioa.pa10.into_push_pull_output();

    // Toggle Heartbeat LED at 1Hz
    loop {
        heartbeat_led.set_high().expect("GPIO can never fail");
        delay.delay_ms(500_u16);
        heartbeat_led.set_low().expect("GPIO can never fail");
        delay.delay_ms(500_u16);
    }
}
