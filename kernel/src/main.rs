#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

// The starting point for the kernel.
pub fn kernel_start() {}

pub fn kernel_panic() {}
pub fn kernel_exit() {}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
