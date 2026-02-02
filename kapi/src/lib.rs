#![no_std]

extern crate alloc;

mod hardware;
mod native_drivers;
mod workspace;

use native_drivers::vga;

use crate::native_drivers::vga::VgaHandle;

#[test]
fn universe_construction() {
    let a = VgaHandle {
        mode: vga::VgaMode::Default,
        workspace_owner: 18,
    };
}
