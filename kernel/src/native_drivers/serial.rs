use uart_16550::*;

use crate::sysio::consoleio::{ConsoleIO, ConsoleIOResult};

pub struct SerialHandle {
    pub ser_port: SerialPort,
}
impl SerialHandle {
    pub fn init(serial_address: u16) -> Self {
        let mut serial_port = unsafe { SerialPort::new(serial_address) };
        serial_port.init();
        serial_port.send(b'a');
        serial_port.try_receive();
        Self {
            ser_port: serial_port,
        }
    }
}

impl ConsoleIO for SerialHandle {
    fn clear(&mut self) {}

    fn get_char(&mut self) -> ConsoleIOResult {
        // Now the serial port is ready to be used. To send a byte:
        let ret = self.ser_port.try_receive();
        match ret {
            Ok(character) => return ConsoleIOResult::Character(character),
            Err(_) => return ConsoleIOResult::ActionWouldBlock,
        }

        // // To receive a byte:
        // let data = serial_port.receive();
        // serial_port.send(data);
    }

    fn push_char(&mut self, character: u8) -> ConsoleIOResult {
        self.ser_port.send(character);
        ConsoleIOResult::SendSuccess
    }
}
