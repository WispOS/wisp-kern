#![no_std]

extern crate alloc;

mod hardware;
mod native_drivers;
mod sysio;
mod workspace;

#[test]
fn universe_construction() {
    use crate::native_drivers::{
        ps2_keyb::KeyboardHandle,
        vga::{VgaHandle, VgaMode},
    };

    let vga_handle = VgaHandle::new(VgaMode::Default);

    let keyb_handle = KeyboardHandle::default_ps2();

    let buffered_key = keyb_handle.internal_key_buffer[0];
}
