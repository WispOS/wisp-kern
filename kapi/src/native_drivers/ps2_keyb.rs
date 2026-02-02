// A (probably) PS/2 keyboard
pub struct KeyboardHandle {
    internal_key_buffer: [char; 128],
}
impl HardwareHandle for KeyboardHandle {
    fn resettable() -> bool {
        true
    }
}
