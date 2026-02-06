// use alloc::vec;

use crate::hardware::{HardwareHandle, interfaces::HWInterface};

/// A (probably) PS/2 keyboard
pub struct KeyboardHandle {
    /// An internal key buffer.
    // TODO: Replace this with a Key struct possibly put in Device Types
    pub internal_key_buffer: [char; 128],
}
impl KeyboardHandle {
    /// Default ps2 keyboard setup
    pub fn default_ps2() -> Self {
        Self {
            internal_key_buffer: [' '; 128],
        }
    }
}

impl HardwareHandle for KeyboardHandle {
    fn resettable(&mut self) -> bool {
        true
    }

    // fn interfaces(&mut self) -> alloc::vec::Vec<HWInterface> {
    //     vec![HWInterface::ps2_interface()]
    // }

    // fn types(&mut self) -> vec::Vec<crate::hardware::device_types::HWTypes> {
    //     todo!()
    // }
}
