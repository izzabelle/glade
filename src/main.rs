#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(glade::test_runner)]
#![reexport_test_harness_main = "test_main"]

// namespacing
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use glade::{print, println};

#[cfg(test)]
use glade::{sprint, sprintln};

// entry point
entry_point!(kmain);
fn kmain(boot_info: &'static BootInfo) -> ! {
    glade::init();
    println!("gladeOS");

    use glade::memory::BootInfoFrameAllocator;
    let mut frame_allocaor = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

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
