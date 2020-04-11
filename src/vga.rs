// namespacing
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// color byte
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// color pair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorPair(u8);

impl ColorPair {
    // create a new color pair from colors
    fn new(foreground: Color, background: Color) -> Self {
        ColorPair((background as u8) << 4 | (foreground as u8))
    }
}

// character and color to be displayed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    ascii_char: u8,
    color_pair: ColorPair,
}

// vga buffer dimensions
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// vga buffer
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<VgaChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// writer struct
pub struct Writer {
    column: usize,
    color_pair: ColorPair,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// write a byte to the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column;
                let color_pair = self.color_pair;

                self.buffer.chars[row][col].write(VgaChar { ascii_char: byte, color_pair });
                self.column += 1;
            }
        }
    }

    /// write a string to the screen
    pub fn write_string(&mut self, s: &str) {
        s.bytes().for_each(|byte| match byte {
            0x20..=0x7e | b'\n' => self.write_byte(byte),
            _ => self.write_byte(0xfe),
        });
    }

    // create a new line
    fn new_line(&mut self) {
        (1..BUFFER_HEIGHT).for_each(|row| {
            (0..BUFFER_WIDTH).for_each(|col| {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            });
        });
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column = 0;
    }

    // clear a row
    fn clear_row(&mut self, row: usize) {
        let blank = VgaChar { ascii_char: b' ', color_pair: self.color_pair };
        (0..BUFFER_WIDTH).for_each(|col| self.buffer.chars[row][col].write(blank));
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        color_pair: ColorPair::new(Color::Pink, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
