#![no_main]
#![no_std]
#![allow(unused_imports)]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f7;

#[entry]
fn main() -> ! {
    cortex_m::asm::nop();

    loop {

    }
}
