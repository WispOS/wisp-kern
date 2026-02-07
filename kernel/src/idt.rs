use core::ptr::addr_of_mut;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    let idt: &mut InterruptDescriptorTable = unsafe { &mut *addr_of_mut!(IDT) };
}

extern "x86-interrupt" fn breakpoint_handler(
    _stack_frame: InterruptStackFrame)
{
    
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}