#![no_std]
#![no_main]

mod board_init;
mod can;

use board_init::init_heap;
use can::{init_can, CanBase};
use integrity_check_system::{err_map::bst::Bst, ics_bus::ics_can_base::ICSCanBase};
use panic_halt as _; 
use cortex_m_rt::entry;
#[allow(unused)]
use stm32h5xx_hal;

#[entry]
fn main() -> ! {
    //TODO: init clock
    init_heap();
    init_can().unwrap();
    let _ics = ICSCanBase::<Bst,can::CanBase>::new(0x600, 2, CanBase::send);

    loop {
        // your code goes here
    }
}
