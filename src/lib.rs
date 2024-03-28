#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use crate::prelude::*;

pub mod allocator;
pub mod exit;
pub mod prelude;
pub mod serial;
pub mod testing;
pub mod vga_buffer;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if cfg!(test) {
        serial_println!("[failed]\n");
        serial_println!("Error: {}\n", info);

        QemuExitCode::Failed.send();
    } else {
        println!("{info}");
    }
    loop {}
}
