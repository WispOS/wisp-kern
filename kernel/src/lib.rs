#![no_std]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod hardware;
pub mod idt;
pub mod sysio;
pub mod native_drivers;
pub mod workspace;

// extern crate alloc;
pub mod idt;

pub fn init() {
    idt::init_idt();
}
