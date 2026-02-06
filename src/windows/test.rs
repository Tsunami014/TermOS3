use crate::winapi::window::{Window, WindowCore};
use crate::winapi::buffer::Buffer;
use crate::winapi::components;

extern crate alloc;
use alloc::{vec, vec::Vec};
use alloc::boxed::Box;
use alloc::sync::Arc;
use spin::Mutex;

pub struct MainW {
    core: WindowCore,
    elms: Vec<Box<dyn components::Element + Send + Sync>>
}
impl MainW {
    pub fn new() -> Self {
        let mut this = Self {
            core: WindowCore::new(),
            elms: vec![
                Box::new(components::Label::new_str("Testing!")),
                Box::new(components::Input::new()),
            ]
        };
        this.redraw();
        this
    }
}
impl Window for MainW {
    fn buffer(&mut self) -> &Arc<Mutex<Buffer>> { self.core.buffer() }
    fn unload(&mut self) { self.core.unload(); }

    fn on_key(&mut self, c: char) {
        for e in &mut self.elms {
            e.on_key(false, c);
        }
        self.redraw();
    }
    fn tick(&mut self) {
        for e in &mut self.elms {
            e.tick(false);
        }
        self.redraw();
    }
    fn redraw(&mut self) {
        let mut writr = self.core.writer().lock();
        writr.clear(0);
        for e in &self.elms {
            e.redraw(false, &mut writr);
        }
    }
}
