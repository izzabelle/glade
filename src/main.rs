#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(glade::test_runner)]
#![reexport_test_harness_main = "test_main"]

// namespacing
use core::panic::PanicInfo;
use glade::{print, println};

#[cfg(test)]
use glade::{sprint, sprintln};

// entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    glade::init();
    println!("gladeOS");

    #[cfg(test)]
    test_main();

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
