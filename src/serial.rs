use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial = unsafe { SerialPort::new(0x3f8) };
        serial.init();
        Mutex::new(serial)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("serial print failed");
    });
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! sprintln {
    () => ($crate::sprint!("\n"));
    ($fmt:expr) => ($crate::sprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::sprint!(concat!($fmt, "\n"), $($arg)*));
}
