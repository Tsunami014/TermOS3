use crate::winapi::buffer::{Writer, WINDOW_WIDTH};
use crate::println_at;

extern crate alloc;
use alloc::string::{String, ToString};

#[allow(dead_code)]
pub trait Element {
    fn unload(&mut self) {}
    fn on_key(&mut self, _c: char) {}
    fn redraw(&self, _writr: &mut spin::MutexGuard<'_, Writer>) {}
}
//print_at!(self.core.writer().lock(), "{}", c);

#[allow(dead_code)]
pub enum Alignment {
    Left = 0,
    Middle = 1,
    Right = 2,
}

pub struct Label {
    text: String,
    pub align: Alignment
}
#[allow(dead_code)]
impl Label {
    pub fn new() -> Self {
        Self { text: "".to_string(), align: Alignment::Middle }
    }
    pub fn new_str(txt: &str) -> Self {
        Self { text: txt.to_string(), align: Alignment::Middle }
    }
    pub fn new_string(txt: String) -> Self {
        Self { text: txt, align: Alignment::Middle }
    }
}
impl Element for Label {
    fn redraw(&self, w: &mut spin::MutexGuard<'_, Writer>) {
        let bytes = self.text.as_bytes();
        for i in (0..bytes.len()).step_by(WINDOW_WIDTH) {
            let end = (i + WINDOW_WIDTH).min(bytes.len());
            let part = &self.text[i..end];

            let len = end - i;
            let spaces = match self.align {
                Alignment::Left => 0,
                Alignment::Middle => (WINDOW_WIDTH - len) / 2,
                Alignment::Right => WINDOW_WIDTH - len,
            };

            println_at!(w, "{}{}", " ".repeat(spaces), part)
        }
    }
}
