use kudos::keyboard::KeyEvent;
use crate::winapi::{
    window::{Window, WindowCore},
    buffer::Buffer,
    components,
};

extern crate alloc;
use alloc::{vec, vec::Vec};
use alloc::boxed::Box;
use alloc::sync::Arc;
use spin::Mutex;

pub struct MainW {
    core: WindowCore,
    elms: Vec<Box<dyn components::Element + Send + Sync>>,
    active: usize,
}
impl MainW {
    pub fn new() -> Self {
        let mut this = Self {
            core: WindowCore::new(),
            elms: vec![
                Box::new(components::Label::new_str("Testing!")),
                Box::new(components::Input::new()),
                Box::new(components::Label::new_str("Another input box!")),
                Box::new(components::Input::new()),
            ],
            active: 0,
        };
        this.redraw();
        this
    }
}
impl Window for MainW {
    fn buffer(&mut self) -> &Arc<Mutex<Buffer>> { self.core.buffer() }
    fn unload(&mut self) { self.core.unload(); }

    fn on_key(&mut self, ev: &KeyEvent) {
        if ev.souper && let Some(c) = ev.unicode {
            if c == 9 as char {
                if ev.shift {
                    self.active -= 1;
                } else {
                    self.active += 1;
                }
                self.active = self.active % self.elms.len();
                self.redraw();
                return;
            }
        }
        for (idx, elm) in self.elms.iter_mut().enumerate() {
            elm.on_key(idx == self.active, ev);
        }
        self.redraw();
    }
    fn tick(&mut self) {
        for (idx, elm) in self.elms.iter_mut().enumerate() {
            elm.tick(idx == self.active);
        }
        self.redraw();
    }
    fn redraw(&mut self) {
        let mut writr = self.core.writer().lock();
        writr.clear();
        for (idx, elm) in self.elms.iter().enumerate() {
            writr.clear_col();
            elm.redraw(idx == self.active, &mut writr);
        }
    }
}
