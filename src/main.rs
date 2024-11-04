#![no_std]
#![no_main]

mod board_init;
mod can;

use board_init::init_heap;
use can::CanBase;
use integrity_check_system::{err_map::bst::Bst, ics_bus::ics_can_base::ICSCanBase};
use panic_halt as _; 
use cortex_m_rt::entry;
#[allow(unused)]
use stm32h5xx_hal;


#[entry]
fn main() -> ! {
    //TODO: init clock
    init_heap();
    CanBase::init_can().unwrap();
    let _ics = ICSCanBase::<Bst,can::CanBase>::new(0x600, 2, CanBase::send);

    let p = stm32h5xx_hal::pac::Peripherals::take().unwrap();
    p.RCC.ahb2enr().modify(|_,w| {w.gpioaen().set_bit()}); //enable GPIOA clock
    p.RCC.ahb2enr().modify(|_,w| {w.adc12en().set_bit()}); //enable ADC1 clock

    p.GPIOA.moder().modify(|_,w| {w.mode0().analog()}); //set PA0 to analog mode

    if p.ADC1.cr().read().aden().bit_is_set(){
        p.ADC1.cr().modify(|_,w| {w.addis().clear_bit()}); //disable ADC if active
    }

    //calibration
    p.ADC1.cr().modify(|_,w| {w.adcal().set_bit()});
    while p.ADC1.cr().read().adcal().bit_is_set() {} //wait completion  calibration

    p.ADC1.cr().modify(|_,w| {w.addis().set_bit()}); //enable ADC
    while p.ADC1.isr().read().adrdy().bit_is_clear() {} //wait until ADC is active

    p.ADC1.sqr1().modify(|_,w|{unsafe{w.sq1().bits(0)}}); //assign ADC1 to channel 0
    p.ADC1.cr().modify(|_,w| {w.adstart().set_bit()}); //start conversion

    loop {
        if p.ADC1.isr().read().eoc().bit_is_set() { //check if ADC has done the conversion in
                                                    //digital
            let _value = p.ADC1.dr().read().bits(); //get converted value

            p.ADC1.isr().modify(|_,w| {w.eoc().clear_bit()}); //clear end conversion flag
        }
    }
}
