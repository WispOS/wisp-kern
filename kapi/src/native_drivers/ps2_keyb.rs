use alloc::vec;

use crate::hardware::{HardwareHandle, interfaces::HWInterface};

// A (probably) PS/2 keyboard
pub struct KeyboardHandle {
    internal_key_buffer: [char; 128],
}
impl HardwareHandle for KeyboardHandle {
    fn resettable() -> bool {
        true
    }

    fn interfaces() -> alloc::vec::Vec<HWInterface> {
        vec![HWInterface::ps2_interface()]
    }

    fn types() -> vec::Vec<crate::hardware::device_types::HWTypes> {
        todo!()
    }
}
