#![no_std]
#![no_main]

mod global_allocator;

use global_allocator::init_heap;
use panic_halt as _; 
use cortex_m_rt::entry;
#[allow(unused)]
use stm32f4xx_hal::{pac, prelude::*, gpio};

#[allow(unused)]
use core::fmt::Write;

#[entry]
fn main() -> ! {
    init_heap();
    
    loop {
        // your code goes here
    }
}
