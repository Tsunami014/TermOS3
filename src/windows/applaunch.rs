use kudos::keyboard::KeyEvent;
use crate::delegate;
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

struct AppLaunchWind {
    inner: ElementWindow,
}
impl AppLaunchWind {
    pub fn new() -> Self {
        Self {
            inner: ElementWindow::new(
                vec![
                    Box::new(components::
                        Label::new_str("Search for apps:").with_invis(true)),
                    Box::new(components::
                        Input::new().with_boxed(false)),
            ]).with_active(1)
        }
    }
}

impl Window for AppLaunchWind {
    delegate!(inner;
        fn buffer(&mut self) -> &Arc<Mutex<crate::winapi::buffer::Buffer>>;
        fn unload(&mut self);

        fn tick(&mut self);
        fn redraw(&mut self);
    );

    fn on_key(&mut self, ev: &KeyEvent) {
        if let Some(c) = ev.unicode && c == '\n' {
            return;
        }
        self.inner.on_key(ev);
    }
}

pub fn window() -> Arc<Mutex<dyn Window>> {
    Arc::new(Mutex::new(AppLaunchWind::new()))
}
