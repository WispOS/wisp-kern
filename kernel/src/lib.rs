#![no_std]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod hardware;
pub mod idt;
pub mod mailbox;
pub mod native_drivers;
pub mod sysio;
pub mod workspace;

pub fn init() {
    gdt::gdt_init();
    idt::init_idt();
    unsafe { idt::PICS.lock().initialize() };
}
