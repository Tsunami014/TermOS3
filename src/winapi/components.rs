use crate::winapi::buffer::{
    Writer, WINDOW_WIDTH,
    ColorCode, Color, DEFAULT_BG,
};
use crate::println_at;

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[allow(dead_code)]
pub trait Element {
    fn unload(&mut self) {}
    fn on_key(&mut self, _focus: bool, _c: char) {}
    fn tick(&mut self, _focus: bool) {}
    fn redraw(&self, _focus: bool, _writr: &mut spin::MutexGuard<'_, Writer>) {}
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    Left = 0,
    Middle = 1,
    Right = 2,
}
fn spacing(align: Alignment, len: usize, width: usize) -> usize {
    match align {
        Alignment::Left => 0,
        Alignment::Middle => (width - len) / 2,
        Alignment::Right => width - len,
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
    fn redraw(&self, focus: bool, w: &mut spin::MutexGuard<'_, Writer>) {
        if focus {
            w.set_colour(ColorCode::new(Color::Yellow, DEFAULT_BG));
        }
        let bytes = self.text.as_bytes();
        for i in (0..bytes.len()).step_by(WINDOW_WIDTH) {
            let end = (i + WINDOW_WIDTH).min(bytes.len());
            let part = &self.text[i..end];

            let spaces = spacing(self.align, end - i, WINDOW_WIDTH);
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
    fn wrap_lines(text: &str) -> Vec<String> {
        let mut out = Vec::new();

        for line in text.split('\n') {
            if line.is_empty() {
                out.push(String::new());
                continue;
            }

            let mut i = 0;
            while i < line.len() {
                let end = (i + (WINDOW_WIDTH-2)).min(line.len());
                out.push(line[i..end].to_string());
                i = end;
            }
        }
        out
    }
}
impl Element for Input {
    fn tick(&mut self, _focus: bool) {
        self.cursor = (self.cursor + 1) % 10;
    }
    fn on_key(&mut self, focus: bool, c: char) {
        if !focus { return; };
        //self.text += &((c as u8).to_string() + " "); // For finding char codes
        if c == 8 as char {
            self.text.pop();
        } else {
            self.text += &c.to_string()
        }
    }
    fn redraw(&self, focus: bool, w: &mut spin::MutexGuard<'_, Writer>) {
        if focus {
            w.set_colour(ColorCode::new(Color::Yellow, DEFAULT_BG));
        }
        let txt = if focus {
            if self.cursor >= 5 {
                self.text.clone() + "_"
            } else {
                self.text.clone() + " "
            }
        } else { self.text.clone() };
        let lines = Input::wrap_lines(&txt);
        let boxwid = lines.iter().map(|l| l.len()).max().unwrap_or(0); 
        let boxspaces = spacing(self.align, boxwid + 2, WINDOW_WIDTH);
        if self.boxed {
            w.write_string(&" ".repeat(boxspaces));
            w.write_byte(0xDA);
            for _col in 0..boxwid {
                w.write_byte(0xC4);
            }
            w.write_byte(0xBF);
            w.write_byte(b'\n');
        }
        for line in &lines {
            if self.boxed {
                let lineln = line.len();
                let left_box_align = spacing(self.align, lineln, boxwid);
                w.write_string(&" ".repeat(boxspaces));
                w.write_byte(0xB3);
                w.write_string(&" ".repeat(left_box_align));
                w.write_string(line);
                w.write_string(&" ".repeat(boxwid - lineln - left_box_align));
                w.write_byte(0xB3);
                w.write_byte(b'\n');
            } else {
                let spaces = spacing(self.align, line.len(), WINDOW_WIDTH);
                println_at!(w, "{}{}", " ".repeat(spaces), line);
            }
        }
        if self.boxed {
            w.write_string(&" ".repeat(boxspaces));
            w.write_byte(0xC0);
            for _col in 0..boxwid {
                w.write_byte(0xC4);
            }
            w.write_byte(0xD9);
            w.write_byte(b'\n');
        }
    }
}
