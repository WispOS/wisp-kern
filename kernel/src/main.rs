#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub extern "C" fn _start() -> ! {
    let vga = 0xB8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga.offset(i as isize * 2) = byte;
            *vga.offset(i as isize * 2 + 1) = 0x0f;
        }
    }

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}