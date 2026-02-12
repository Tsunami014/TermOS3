use crate::winapi::{
    buffer::{Buffer, Writer},
    components,
};
use kudos::keyboard::KeyEvent;

extern crate alloc;
use alloc::{
    vec::Vec,
    boxed::Box,
    sync::Arc,
};
use core::option::Option;
use spin::Mutex;

use crate::opens;

#[allow(dead_code)]
pub trait Window: Send + Sync {
    fn buffer(&mut self) -> &Arc<Mutex<Buffer>>;
    fn unload(&mut self);

    fn on_key(&mut self, _ev: &KeyEvent) {}
    fn tick(&mut self) {}
    fn redraw(&mut self) {}
}
#[macro_export]
macro_rules! delegate {
    (
        $field:ident;
        $(
            fn $name:ident ( &mut self $(, $arg:ident : $ty:ty )* ) $(-> $ret:ty)?;
        )*
    ) => {
        $(
            fn $name(&mut self, $( $arg: $ty ),* ) $(-> $ret)? {
                self.$field.$name($( $arg ),*)
            }
        )*
    };
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


type ElementList = Vec<Box<dyn components::Element + Send + Sync>>;
pub struct ElementWindow {
    core: WindowCore,
    elms: ElementList,
    active: usize,
}
impl ElementWindow {
    pub fn new(elms: ElementList) -> Self {
        let mut this = Self {
            core: WindowCore::new(),
            elms: elms,
            active: 0,
        };
        this.redraw();
        this
    }

    pub fn with_active(mut self, active: usize) -> Self {
        self.active = active;
        self
    }

    fn upd_active(&mut self, dir: i8) {
        let orig = self.active;

        loop {
            if dir == 1 {
                self.active = (self.active + 1) % self.elms.len();
            } else {
                if self.active == 0 {
                    self.active = self.elms.len() - 1;
                } else {
                    self.active -= 1;
                }
            }
            if self.active == orig ||
                    !self.elms.get(self.active).expect("If this fails something very bad happened").invisible() {
                break;
            }
        }
    }
}
impl Window for ElementWindow {
    fn buffer(&mut self) -> &Arc<Mutex<Buffer>> { self.core.buffer() }
    fn unload(&mut self) { self.core.unload(); }

    fn on_key(&mut self, ev: &KeyEvent) {
        if ev.souper && let Some(c) = ev.unicode {
            if c == 9 as char {
                let dir: i8 = if ev.shift { -1 } else { 1 };
                self.upd_active(dir);
                self.redraw();
                return;
            } else if c == 'q' {
                opens::exit_window();
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
