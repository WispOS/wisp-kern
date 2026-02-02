// A general purpose hardware description struct.
// ALL HARDWARE should share these common features.
pub struct HWDesc {
    // Signals wether hardware is software resettable.
    resettable: bool,
}

// The trait for hardware to be understood by the kernel.
pub trait HardwareHandle {
    // Resetability of hardware.
    fn resettable() -> bool;
}
