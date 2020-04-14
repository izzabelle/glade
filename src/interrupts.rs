// namespacing
use crate::{print, println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[cfg(test)]
use crate::{sprint, sprintln};

// IDT for the kernel
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

/// initialize the IDT
pub fn init_idt() {
    IDT.load();
}

/// interrupt for a breakpoint
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n {:#?}", stack_frame);
}

// breakpoint test
#[test_case]
fn test_breakpoint_exception() {
    sprint!("test_breakpoint_exception... ");
    x86_64::instructions::interrupts::int3();
    sprintln!("[Ok!]");
}
