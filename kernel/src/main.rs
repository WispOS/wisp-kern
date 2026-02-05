#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::{
    native_drivers::vga::{VgaHandle, VgaMode},
    sysio::displayio::*,
};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub extern "C" fn _start() -> ! {
    let mut vga_handle = VgaHandle::new(VgaMode::Default);
    vga_handle.refresh();

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
