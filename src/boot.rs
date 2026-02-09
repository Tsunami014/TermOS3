use kudos::task::{Task, executor::Executor};
use kudos::keyboard::KeyboardStream;
use futures_util::stream::StreamExt;
use x86_64::instructions::interrupts;

use crate::{
    display::display,
    winapi::window::Window,
    windows,
};

extern crate alloc;
use alloc::sync::Arc;
use spin::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref MainWind: Arc<Mutex<dyn Window>> =
        Arc::new(Mutex::new(windows::test::MainW::new()));
}

async fn main() {
    use kudos::{connect, interrupts::TimerIntSig};
    connect!(TimerIntSig, async |_| {
        interrupts::without_interrupts(|| {
            MainWind.lock().tick();
            display(MainWind.lock().buffer());
        });
    });

    interrupts::without_interrupts(|| {
        MainWind.lock().redraw();
        display(MainWind.lock().buffer());
    });
    let mut kstream = KeyboardStream::new();
    while let Some(ev) = kstream.next().await {
        interrupts::without_interrupts(|| {
            MainWind.lock().on_key(&ev);
            display(MainWind.lock().buffer());
        });
    }
}

/// This function will run when running the main program
pub fn on_boot() {
    let mut executor = Executor::new();
    executor.spawn(Task::new(main()));
    executor.run();
}
