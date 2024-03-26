#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    serial_println!("trivial assertion... ");
    let x = 0;
    assert_eq!(x, 1);
    serial_println!("[ok]");
}
