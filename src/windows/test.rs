use crate::print_at;
use crate::winapi::window::{WindowCtx, WindowLogic};

use crate::winapi::buffer::Buffer;
extern crate alloc;
use spin::Mutex;
use alloc::sync::Arc;

pub struct MainW {
    ctx: WindowCtx,
}
impl MainW {
    pub fn new() -> Self {
        Self {
            ctx: WindowCtx::new(),
        }
    }
}
impl WindowLogic for MainW {
    fn on_key(&mut self, c: char) {
        print_at!(self.ctx.writer().lock(), "{}", c);
    }
    fn buffer(&mut self) -> &Arc<Mutex<Buffer>> {
        self.ctx.buffer()
    }
}
