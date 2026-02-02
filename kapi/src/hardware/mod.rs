use crate::hardware::{device_types::HWTypes, interfaces::HWInterface};
use alloc::vec::Vec;

pub mod device_types;
pub mod interfaces;

// A general purpose hardware description struct.
// ALL HARDWARE should share these common features.
// The trait for hardware to be understood by the kernel.
pub trait HardwareHandle {
    // Resetability of hardware.
    fn resettable() -> bool;
    fn interfaces() -> Vec<HWInterface>;
    // Returns the types of a Handle
    fn types() -> Vec<HWTypes>;
}
