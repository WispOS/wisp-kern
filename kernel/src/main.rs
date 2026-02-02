#![no_std]
#![no_main]

use core::arch::asm;

entry_point!(kmain);
fn kmain(boot_info: &'static mut BootInfo) -> ! {
    println!("Hello, world!");
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    
}