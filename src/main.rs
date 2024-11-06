#![no_std]
#![no_main]

mod global_allocator;

use global_allocator::init_heap;
use panic_halt as _; 
use cortex_m_rt::entry;
#[allow(unused)]
use stm32f4xx_hal::{pac, prelude::*, gpio, uart::Serial};

#[allow(unused)]
use core::fmt::Write;

#[entry]
fn main() -> ! {
    init_heap();

    let dp = pac::Peripherals::take().unwrap();
    
    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    let mut delay = dp.TIM1.delay_ms(&clocks);

    let tx_pin = gpioa.pa9;

    let _rx_pin = gpioa.pa10;

    let mut tx: stm32f4xx_hal::uart::Tx<pac::USART1, u8> = Serial::tx(dp.USART1, tx_pin, 115200.bps(), &clocks).unwrap();

    //let mut rx: stm32f4xx_hal::uart::Rx<pac::USART1, u8> = Serial::rx(dp.USART1, rx_pin, 115200.bps(), &clocks).unwrap();

    let mut value: u8 = 0;

    loop {
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {value:02}\r").unwrap();
        value = value.wrapping_add(1);
        delay.delay(2.secs())
    }
}
