#![no_std]
#![no_main]

// modules
mod vga;

// namespacing
use core::panic::PanicInfo;

// entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world!");
    loop {}
}

// panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
