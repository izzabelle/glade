#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

// modules
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga;

// namespacing
use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

/// initialize the kernel
pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

/// exit qemu with given code
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// hlt while looping to reduce CPU usage
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// test entry point
#[cfg(test)]
entry_point!(test_kmain);
#[cfg(test)]
fn test_kmain(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

// test panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// qemu exit code enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// test harness
pub fn test_runner(tests: &[&dyn Fn()]) {
    sprintln!("Running {} tests", tests.len());
    tests.iter().for_each(|test| test());
    exit_qemu(QemuExitCode::Success);
}

// test panic handler internals
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    sprintln!("[Err]\n");
    sprintln!("what: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}
