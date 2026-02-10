use kudos::keyboard::KeyEvent;
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

fn handle_key(ev: &KeyEvent) -> bool {
    if let Some(c) = ev.unicode && c == 10 as char {
        return true;
    }
    false
}

pub fn window() -> Arc<Mutex<dyn Window>> {
    Arc::new(Mutex::new(ElementWindow::new(vec![
        Box::new(components::
            Label::new_str("Search for apps:").with_invis(true)),
        Box::new(components::
            Input::new().with_boxed(false).with_keyhandler(handle_key)),
    ]).with_active(1)
    ))
}
