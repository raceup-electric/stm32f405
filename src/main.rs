#![no_std]
#![no_main]

use embassy_executor::Spawner;
#[allow(unused_imports)]
use embassy_futures::join::join;
#[allow(unused_imports)]
use embassy_stm32::{bind_interrupts, peripherals, usb, Config};
use defmt::info;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Run")    
}
