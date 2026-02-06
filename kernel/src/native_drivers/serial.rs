use uart_16550::*;
/// Kernel handle
pub struct SerialHandle {
    ///
    pub ser_port: SerialPort,
}
impl SerialHandle {
    /// Initialize a serial handle
    pub fn init(serial_address: u16) -> Self {
        let mut serial_port = unsafe { SerialPort::new(serial_address) };
        serial_port.init();
        Self {
            ser_port: serial_port,
        }
    }
}
