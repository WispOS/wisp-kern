pub mod device_types;
pub mod interfaces;

// use alloc::vec;

use crate::hardware::{device_types::HWTypes, interfaces::HWInterface};

// A general purpose hardware description struct.
// ALL HARDWARE should share these common features.
pub struct HWDesc {
    // Signals wether hardware is software resettable.
    resettable: bool,
}

// The trait for hardware to be understood by the kernel.
pub trait HardwareHandle {
    // Resetability of hardware.
    fn resettable(&mut self) -> bool;
    // fn interfaces(&mut self) -> alloc::vec::Vec<HWInterface>;
    // fn types(&mut self) -> vec::Vec<HWTypes>;
}
