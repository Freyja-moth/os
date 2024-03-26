#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod allocator;
pub mod exit;
pub mod prelude;
pub mod serial;
pub mod vga_buffer;

use crate::prelude::*;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    println!("I'm not in a test");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn test_panic_handler(_info: &PanicInfo) -> ! {
    serial_println!("I'm in a test");

    QemuExitCode::Failed.send();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    QemuExitCode::Success.send()
}
