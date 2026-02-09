use kudos::task::{Task, executor::Executor};
use kudos::keyboard::KeyboardStream;
use futures_util::stream::StreamExt;
use x86_64::instructions::interrupts;

use crate::{
    display::{display, clear_display},
    windows::WINDOWS,
    opens,
};


fn tick() {
    interrupts::without_interrupts(|| {
        if let Some(w) = opens::get_open() {
            let mut win = w.lock();
            win.tick();
            display(win.buffer());
        } else {
            clear_display();
        }
    });
}

async fn main() {
    if let Some(w) = WINDOWS.get(0) {
        opens::add_window(w());
    }

    use kudos::{connect, interrupts::TimerIntSig};
    extern crate alloc;
    connect!(TimerIntSig, async |_| { tick(); });
    tick();

    let mut kstream = KeyboardStream::new();
    while let Some(ev) = kstream.next().await {
        interrupts::without_interrupts(|| {
            if let Some(w) = opens::get_open() {
                let mut win = w.lock();
                win.on_key(&ev);
                display(win.buffer());
            } else {
                clear_display();
            }
        });
    }
}

/// This function will run when running the main program
pub fn on_boot() {
    let mut executor = Executor::new();
    executor.spawn(Task::new(main()));
    executor.run();
}
