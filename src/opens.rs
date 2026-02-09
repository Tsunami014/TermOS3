use crate::winapi::window::Window;
use crate::windows::APPLAUNCHER;

extern crate alloc;
use alloc::{
    vec::Vec,
    sync::Arc,
};
use spin::Mutex;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref OPEN_WINDOWS: Mutex<Vec<Arc<Mutex<dyn Window>>>> =
        Mutex::new(Vec::new());
}
static OPEN_WINDOW_IDX: Mutex<usize> = Mutex::new(usize::MAX);
lazy_static! {
    pub static ref LAUNCHER: Arc<Mutex<dyn Window>> =
        APPLAUNCHER();
}

pub fn get_open() -> Option<Arc<Mutex<dyn Window>>> {
    let idx = *OPEN_WINDOW_IDX.lock();
    if idx == usize::MAX {
        return Some(LAUNCHER.clone());
    }

    let windows = OPEN_WINDOWS.lock();
    windows.get(idx).cloned()
}

pub fn enter_window(win: Arc<Mutex<dyn Window>>) {
    let mut windows = OPEN_WINDOWS.lock();
    windows.push(win);

    let mut idx = OPEN_WINDOW_IDX.lock();
    *idx = windows.len() - 1;
}

pub fn exit_window() {
    let mut idx = OPEN_WINDOW_IDX.lock();
    if *idx == usize::MAX {
        return;
    }

    let mut windows = OPEN_WINDOWS.lock();

    if *idx < windows.len() {
        windows.remove(*idx);
    }
    *idx = usize::MAX;
}
