use crate::winapi::buffer::{Buffer, Writer};

extern crate alloc;
use core::option::Option;
use spin::Mutex;
use alloc::sync::Arc;

#[allow(dead_code)]
pub trait Window: Send + Sync {
    fn buffer(&mut self) -> &Arc<Mutex<Buffer>>;
    fn unload(&mut self);

    fn on_key(&mut self, _c: char) {}
    fn tick(&mut self) {}
    fn redraw(&mut self) {}
}

pub struct WindowCore {
    pub buf: Option<Arc<Mutex<Buffer>>>,
    pub writr: Option<Mutex<Writer>>,
}
#[allow(dead_code)]
impl WindowCore {
    pub fn new() -> Self {
        let mut this = Self {
            buf: None,
            writr: None
        };
        this.load();
        this
    }
    fn load(&mut self) {
        let buf = Arc::new(Mutex::new(Buffer::new()));
        self.buf = Some(buf.clone());
        self.writr = Some(Mutex::new(Writer::new(buf.clone())));
    }
    pub fn unload(&mut self) {
        self.buf = None;
        self.writr = None;
    }
    pub fn buffer(&mut self) -> &Arc<Mutex<Buffer>> {
        if let None = self.buf {
            self.load();
        }
        self.buf.as_ref().unwrap()
    }
    pub fn writer(&mut self) -> &Mutex<Writer> {
        if let None = self.writr {
            self.load();
        }
        self.writr.as_ref().unwrap()
    }
}
