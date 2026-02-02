use kudos::task::{Task, executor::Executor, keyboard::ScancodeStream};
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::display::display;
use crate::{print_at, println_at, window::buffer::{Buffer, Writer}};

extern crate alloc;
use alloc::sync::Arc;
use spin::Mutex;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref MainWind: Arc<Mutex<Buffer>> =
        Arc::new(Mutex::new(Buffer::new(0)));
}

async fn main() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(ScancodeSet1::new(),
        layouts::Us104Key, HandleControl::Ignore);
    let mut writr: Writer = Writer::new(MainWind.clone());

    display(MainWind.lock());
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(DecodedKey::Unicode(c)) = keyboard.process_keyevent(key_event) {
                print_at!(&mut writr, "{}", c);
                display(MainWind.lock());
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
