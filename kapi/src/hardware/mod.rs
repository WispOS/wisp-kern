use crate::hardware::interfaces::HWInterface;

mod interfaces;

// A general purpose hardware description struct.
// ALL HARDWARE should share these common features.
// The trait for hardware to be understood by the kernel.
pub trait HardwareHandle {
    // Resetability of hardware.
    fn resettable() -> bool;
    // fn interfaces() -> Vec<HWInterface>;
}
