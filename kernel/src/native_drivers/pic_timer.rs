// Called when a PIC Timer interrupt fires off.
pub extern "x86-interrupt" fn timer_interrupt_handler() {
    // TODO: Fire off the scheduler to move tasks.
    // TODO: Fire off the scheduler to awake tasks that were slept.
    // TODO: Fire off the scheduler to hit various other timeout signals.
    // TODO: Figure out a way to defer all of the actual hard work here to keep interrupts speedy.
}
