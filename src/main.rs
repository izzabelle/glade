#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(glade::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;

// namespacing
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use glade::{print, println};

#[cfg(test)]
use glade::{sprint, sprintln};

// entry point
entry_point!(kmain);
fn kmain(boot_info: &'static BootInfo) -> ! {
    use glade::allocator;
    use glade::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    glade::init();
    println!("gladeOS");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(42);

    #[cfg(test)]
    test_main();

    println!("still running");
    glade::hlt_loop();
}

// panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    glade::hlt_loop();
}

// test panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    glade::test_panic_handler(info);
}

// just to make sure it's running ok
#[test_case]
fn trivial_assertion() {
    sprint!("trivial_assertion... ");
    assert_eq!(1, 1);
    sprintln!("[Ok!]");
}
