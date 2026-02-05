// ConsoleIO is a serial like terminal experience

pub enum ConsoleIOResult {
    SendSuccess,
    SerialPortError,
    ActionWouldBlock,
    Character(u8),
}

pub trait ConsoleIO {
    fn clear(&mut self);
    fn get_char(&mut self) -> ConsoleIOResult;
    fn push_char(&mut self, character: u8) -> ConsoleIOResult;
}
