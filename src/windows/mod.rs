// Just import the other things in this folder
mod test;

extern crate alloc;
use alloc::sync::Arc;
use spin::Mutex;
use crate::winapi::window::Window;

pub const WINDOWS: &[fn() -> Arc<Mutex<dyn Window>>] = &[
    test::window
];
