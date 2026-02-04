use kudos::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT};

pub const WINDOW_WIDTH: usize = BUFFER_WIDTH - 2;
pub const WINDOW_HEIGHT: usize = BUFFER_HEIGHT - 2;

#[repr(transparent)]
pub struct Buffer {
    chars: [[u8; WINDOW_WIDTH]; WINDOW_HEIGHT],
}
impl Buffer {
    pub fn new(fill: u8) -> Self {
        Self {
            chars: [[fill; WINDOW_WIDTH]; WINDOW_HEIGHT],
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.chars[y][x]
    }
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, ch: u8) {
        self.chars[y][x] = ch;
    }

    pub fn clear(&mut self, fill: u8) {
        for row in &mut self.chars {
            row.fill(fill);
        }
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

    #[inline]
    fn new_line(&mut self) {
        self.column_position = self.wrap_from;
        self.row_position += 1;
    }

    pub fn clear(&mut self, fill: u8) {
        self.locked_buf().clear(fill);
        self.column_position = 0;
        self.row_position = 0;
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
