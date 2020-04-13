#![no_std]
#![no_main]

use core::panic::PanicInfo;
use glade::{exit_qemu, sprint, sprintln, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    sprintln!("[Err]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    sprint!("should_fail... ");
    assert!(false);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    sprintln!("[Ok!]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
