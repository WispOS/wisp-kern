/// Hardware interface types.
pub enum InterfaceType {
    /// Ram memory interface
    Memory,
    /// Universal Serial Bus
    Usb,
    /// Annoying ass interface.
    Ps2,
}
/// A Singular interface type on a device.
pub struct HWInterface {
    itype: InterfaceType,
}
impl HWInterface {
    ///
    pub fn new_memory_interface(start: u64, end: u64) -> Self {
        Self {
            itype: InterfaceType::Memory,
        }
    }
    ///
    pub fn new_ps2_interface() -> Self {
        Self {
            itype: InterfaceType::Ps2,
        }
    }
}
