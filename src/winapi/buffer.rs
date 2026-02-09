use kudos::vga_buffer::{
    BUFFER_WIDTH, BUFFER_HEIGHT,
};
#[allow(unused_imports)]
pub use kudos::vga_buffer::{
    Color, ColorCode, DEFAULT_FG, DEFAULT_BG,
};

pub const WINDOW_WIDTH: usize = BUFFER_WIDTH - 2;
pub const WINDOW_HEIGHT: usize = BUFFER_HEIGHT - 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

pub struct Buffer {
    chars: [[ScreenChar; WINDOW_WIDTH]; WINDOW_HEIGHT],
    pub colour: ColorCode,
}
impl Buffer {
    pub fn new() -> Self {
        Self {
            chars: [[ScreenChar{ ascii_char: 0, color_code: ColorCode::default() }; WINDOW_WIDTH]; WINDOW_HEIGHT],
            colour: ColorCode::default(),
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.chars[y][x].ascii_char
    }
    #[inline]
    pub fn get_col(&self, x: usize, y: usize) -> ColorCode {
        self.chars[y][x].color_code
    }
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, ch: u8) {
        self.chars[y][x] = ScreenChar{ ascii_char: ch, color_code: self.colour };
    }

    pub fn clear(&mut self) {
        for row in &mut self.chars {
            row.fill(ScreenChar{ ascii_char: 0, color_code: ColorCode::default() });
        }
    }
    pub fn clear_col(&mut self) {
        self.colour = ColorCode::default();
    }
}

extern crate alloc;
use spin::Mutex;
use alloc::sync::Arc;
pub struct Writer {
    pub column_position: usize,
    pub row_position: usize,
    pub wrap_from: usize,
    pub wrap_at: usize,
    buffer: Arc<Mutex<Buffer>>,
}
#[allow(dead_code)]
impl Writer {
    pub fn new(buf: Arc<Mutex<Buffer>>) -> Self {
        Self {
            column_position: 0, row_position: 0,
            wrap_from: 0, wrap_at: WINDOW_WIDTH,
            buffer: buf,
        }
    }

    fn locked_buf(&self) -> spin::MutexGuard<'_, Buffer> {
        self.buffer.lock()
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= self.wrap_at {
                    self.new_line();
                }
                if self.row_position >= WINDOW_HEIGHT {
                    return;
                }
                let col = self.column_position;
                let row = self.row_position; 
                self.locked_buf().set(col, row, byte);
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn col(&self) -> ColorCode {
        self.locked_buf().colour
    }
    pub fn set_col(&mut self, new_col: ColorCode) {
        self.locked_buf().colour = new_col;
    }

    #[inline]
    fn new_line(&mut self) {
        self.column_position = self.wrap_from;
        self.row_position += 1;
    }

    pub fn clear(&mut self) {
        self.locked_buf().clear();
        self.column_position = 0;
        self.row_position = 0;
    }
    pub fn clear_col(&mut self) {
        self.locked_buf().clear_col();
    }
}

// So the print macros can use write_fmt
use core::fmt;
use core::fmt::Write;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Some macros for ease of use
#[macro_export]
macro_rules! print_at {
    ($writr:expr, $($arg:tt)*) => ($crate::winapi::buffer::_print_at($writr, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_at {
    ($writr:expr) => ($crate::print_at!($writr, "\n"));
    ($writr:expr, $($arg:tt)*) => ($crate::print_at!($writr, "{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print_at(writr: &mut spin::MutexGuard<'_, Writer>, args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        writr.write_fmt(args).unwrap();
    });
}
