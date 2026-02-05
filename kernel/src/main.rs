#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::{
    native_drivers::{
        serial::SerialHandle,
        vga::{VgaHandle, VgaMode},
    },
    sysio::{consoleio::ConsoleIO, displayio::*},
};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut serial_handle = SerialHandle::init(0x3F8);
    for character in "KERNEL PANICKED".chars() {
        serial_handle.ser_port.send(character as u8);
    }
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub extern "C" fn _start() -> ! {
    let mut vga_handle = VgaHandle::new(VgaMode::Default);
    let mut serial_handle = SerialHandle::init(0x3F8);
    // serial_handle.clear();

    loop {
        let chara_ = serial_handle.ser_port.try_receive();
        match chara_ {
            Ok(a) => serial_handle.ser_port.send(a),
            Err(b) => {
                let mut serial_handle = SerialHandle::init(0x3F8);
                serial_handle.clear();
                for character in "KERNEL PANICKED".chars() {
                    panic!();
                    serial_handle.push_char(character as u8);
                }
            }
        }
        // unsafe { core::arch::asm!("hlt") }
    }
}
