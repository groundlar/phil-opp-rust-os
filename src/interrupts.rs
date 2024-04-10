// TODO read blog series on naked functions in Rust!
// https://os.phil-opp.com/edition-1/extra/naked-exceptions/
use crate::gdt;
use crate::print;
use crate::println;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PortValues {
    PS2 = 0x60,
}

impl From<InterruptIndex> for u8 {
    fn from(value: InterruptIndex) -> Self {
        value as u8
    }
}
impl From<InterruptIndex> for usize {
    fn from(value: InterruptIndex) -> Self {
        usize::from(u8::from(value))
    }
}

impl From<PortValues> for u16 {
    fn from(value: PortValues) -> Self {
        value as u16
    }
}

// Remap Programmable Interrupt Controller vector numbers to 32-47 because
// we already use 0-15 for CPU exceptions.
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// TODO Evaluate maintained alternatives like OnceCell / OnceLock / LazyLock,
// see https://github.com/rust-lang-nursery/lazy-static.rs/issues/214
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[u8::from(InterruptIndex::Timer)].set_handler_fn(timer_interrupt_handler);
        idt[u8::from(InterruptIndex::Keyboard)].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// x86 double faults are divergent, no recovery.
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // NOTE: this can deadlock on global WRITER with non-interrupt prints.
    print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Keyboard controller won't fire another interrupt until we read the scancode.
    let mut port = Port::new(PortValues::PS2.into());
    let scancode: u8 = unsafe { port.read() };
    // NOTE: this can deadlock on global WRITER with non-interrupt prints.
    print!("{}", scancode);
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.into());
    }
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
