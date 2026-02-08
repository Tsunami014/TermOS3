use kudos::task::{Task, executor::Executor, keyboard::ScancodeStream};
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::instructions::interrupts;

use crate::display::display;
use crate::winapi::{
    window::Window,
    kbd::KeyMods,
};
use crate::windows;

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

    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(ScancodeSet1::new(),
        layouts::Us104Key, HandleControl::Ignore);

    interrupts::without_interrupts(|| {
        MainWind.lock().redraw();
        display(MainWind.lock().buffer());
    });
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            let mods = KeyMods::from(keyboard.get_modifiers());
            if let Some(DecodedKey::Unicode(c)) = keyboard.process_keyevent(key_event) {
                interrupts::without_interrupts(|| {
                    MainWind.lock().on_key(c, mods);
                    display(MainWind.lock().buffer());
                });
            }
        }
    }
}

/// This function will run when running the main program
pub fn on_boot() {
    let mut executor = Executor::new();
    executor.spawn(Task::new(main()));
    executor.run();
}
