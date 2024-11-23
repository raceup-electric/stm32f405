//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: any STM32F4 with a OTG FS peripheral and a 25MHz HSE crystal
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::{prelude::*, device::UsbRev};

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.MHz()) // Match your hardware
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .require_pll48clk() // Ensures 48 MHz for USB
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    let mut usb_dev = gpioc.pc11.into_push_pull_output();
    usb_dev.set_low();

    let usb = USB::new(
        (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
        (gpioa.pa11, gpioa.pa12),
        &clocks,
    );

    #[allow(static_mut_refs)]
    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x0483, 0x5710))
        .device_class(0x02)         // CDC (Communications Device Class)
        .device_sub_class(0x00)         // Subclass, typically 0x00 for CDC ACM
        .device_protocol(0x00)          // Protocol, 0x00 for default CDC ACM
        .device_release(0x0100)         // Device release version: 1.00
        .self_powered(false)            // The device is bus-powered (not self-powered)
        .supports_remote_wakeup(false)  // The device does not support remote wakeup
        .usb_rev(UsbRev::Usb200)
        .max_packet_size_0(64)
        .unwrap()
        .strings(&[StringDescriptors::default()
            .manufacturer("GDMicroelectronics")
            .product("GD32 DFU Bootloader")
            .serial_number("try")])
        .unwrap()
        .max_power(200)
        .unwrap()
        .build();

    let mut led = gpioc.pc12.into_push_pull_output();
    led.set_high();

    if (clocks.pll48clk()) != core::prelude::v1::Some(48.MHz()) {
        led.set_low();// Indicate error (e.g., turn on a red LED)
    }

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            //led.toggle();
            cortex_m::asm::delay(10);
            continue;

        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_high();
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}