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
        Box::new(components::Label::new_str("Search for apps:")),
        Box::new(components::Input::new().with_boxed(false)),
    ]).with_active(1)
    ))
}
