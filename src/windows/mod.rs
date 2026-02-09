// Just import the other things in this folder
mod test;
mod applaunch;

extern crate alloc;
use alloc::sync::Arc;
use spin::Mutex;
use crate::winapi::window::Window;

type GetWindowFunction = fn() -> Arc<Mutex<dyn Window>>;

pub const WINDOWS: &[GetWindowFunction] = &[
    test::window,
];

pub const APPLAUNCHER: GetWindowFunction = applaunch::window;
