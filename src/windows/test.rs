use crate::winapi::{
    window::{Window, ElementWindow},
    components,
};

extern crate alloc;
use alloc::{
    vec,
    boxed::Box,
    sync::Arc,
};
use spin::Mutex;

pub fn window() -> Arc<Mutex<dyn Window>> {
    Arc::new(Mutex::new(ElementWindow::new(vec![
        Box::new(components::Label::new_str("Testing!")),
        Box::new(components::Input::new()),
        Box::new(components::Label::new_str("Another input box!")),
        Box::new(components::Input::new()),
    ])))
}
