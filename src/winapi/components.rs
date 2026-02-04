use crate::winapi::buffer::Writer;
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

pub struct Label {
    text: String
}
#[allow(dead_code)]
impl Label {
    pub fn new() -> Self {
        Self { text: "".to_string() }
    }
    pub fn new_str(txt: &str) -> Self {
        Self { text: txt.to_string() }
    }
    pub fn new_string(txt: String) -> Self {
        Self { text: txt }
    }
}
impl Element for Label {
    fn redraw(&self, w: &mut spin::MutexGuard<'_, Writer>) {
        println_at!(w, "{}", self.text)
    }
}
