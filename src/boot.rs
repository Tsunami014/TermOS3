use kudos::utils::fancy;
use kudos::utils::keys::choice;
use kudos::task::{Task, executor::Executor, keyboard::ScancodeStream};
use kudos::{print, println, printlgln, LogType};

async fn main() {
    let mut scancodes = ScancodeStream::new();
    loop {
        let chararr = ['y', 'n'];
        let c = choice(&mut scancodes, &chararr).await;
        fancy::clear_line();
        if c == 'y' {
            print!("Yes!");
        } else {
            print!("No.");
        }
    }
}

/// This function will run when running the main program
pub fn on_boot() {
    printlgln!("Hello world!");
    println!("Press y/n");

    let mut executor = Executor::new();
    executor.spawn(Task::new(main()));
    executor.run();
}
