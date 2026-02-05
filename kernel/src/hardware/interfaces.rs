pub enum InterfaceType {
    //
    Memory,
    Usb,
    // Annoying ass interface.
    Ps2,
}
// A Singular interface type on a device.
pub struct HWInterface {
    itype: InterfaceType,
}
impl HWInterface {
    pub fn memory_interface(start: u64, end: u64) -> Self {
        Self {
            itype: InterfaceType::Memory,
        }
    }
    pub fn ps2_interface() -> Self {
        Self {
            itype: InterfaceType::Ps2,
        }
    }
}
