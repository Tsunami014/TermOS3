#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[cfg(feature = "bootloader")]
use bootloader::{BootInfo, entry_point};

#[cfg(feature = "bootloader")]
entry_point!(bl_main);
#[cfg(feature = "bootloader")]
fn bl_main(boot_info: &'static BootInfo) -> ! {
    kudos::init(boot_info, true);
    main()
}
#[cfg(not(feature = "bootloader"))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main()
}

fn main() -> ! {
    kudos::printlgln!("Hello world!");

    kudos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kudos::real_panic_handler(info)
}
