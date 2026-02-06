use crate::winapi::buffer::{Writer, WINDOW_WIDTH};
use crate::println_at;

extern crate alloc;
use alloc::string::{String, ToString};

#[allow(dead_code)]
pub trait Element {
    fn unload(&mut self) {}
    fn on_key(&mut self, _focus: bool, _c: char) {}
    fn tick(&mut self, _focus: bool) {}
    fn redraw(&self, _focus: bool, _writr: &mut spin::MutexGuard<'_, Writer>) {}
}
//print_at!(self.core.writer().lock(), "{}", c);

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    Left = 0,
    Middle = 1,
    Right = 2,
}
fn spacing(align: Alignment, len: usize) -> usize {
    match align {
        Alignment::Left => 0,
        Alignment::Middle => (WINDOW_WIDTH - len) / 2,
        Alignment::Right => WINDOW_WIDTH - len,
    }
}

pub struct Label {
    pub text: String,
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
    fn redraw(&self, _focus: bool, w: &mut spin::MutexGuard<'_, Writer>) {
        let bytes = self.text.as_bytes();
        for i in (0..bytes.len()).step_by(WINDOW_WIDTH) {
            let end = (i + WINDOW_WIDTH).min(bytes.len());
            let part = &self.text[i..end];

            let spaces = spacing(self.align, end - i);
            println_at!(w, "{}{}", " ".repeat(spaces), part)
        }
    }
}


pub struct Input {
    pub text: String,
    pub align: Alignment,
    pub boxed: bool,
    cursor: u8,
}
impl Input {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
            align: Alignment::Middle,
            boxed: true,
            cursor: 0
        }
    }
}
impl Element for Input {
    fn tick(&mut self, _focus: bool) {
        self.cursor = (self.cursor + 1) % 10;
    }
    fn on_key(&mut self, _focus: bool, c: char) {
        self.text += &c.to_string()
    }
    fn redraw(&self, focus: bool, w: &mut spin::MutexGuard<'_, Writer>) {
        let txt = if self.cursor >= 5 {
            self.text.clone() + "_"
        } else {
            self.text.clone() + " "
        };
        let mxwid = if self.boxed { WINDOW_WIDTH-2 } else { WINDOW_WIDTH };
        let len = txt.len();
        let reallen = if len > mxwid { WINDOW_WIDTH } else { len };
        let gspaces = if len > mxwid { 0 } else { spacing(self.align, len+2) };
        if self.boxed {
            w.write_string(&" ".repeat(gspaces));
            w.write_byte(0xDA);
            for _col in 0..reallen {
                w.write_byte(0xC4);
            }
            w.write_byte(0xBF);
            w.write_byte(b'\n');
        }
        for i in (0..len).step_by(mxwid) {
            let end = (i + mxwid).min(len);
            let part = &txt[i..end]; 
            
            if self.boxed {
                let plen = end - i;
                if plen < mxwid {
                    let spaces = spacing(self.align, plen+2);
                    w.write_string(&" ".repeat(spaces));
                }
                w.write_byte(0xB3);
                w.write_string(part);
                w.write_byte(0xB3);
                w.write_byte(b'\n');
            } else {
                let spaces = spacing(self.align, end - i);
                println_at!(w, "{}{}", " ".repeat(spaces), part);
            }
        }
        if self.boxed {
            w.write_string(&" ".repeat(gspaces));
            w.write_byte(0xC0);
            for _col in 0..reallen {
                w.write_byte(0xC4);
            }
            w.write_byte(0xD9);
            w.write_byte(b'\n');
        }
    }
}
