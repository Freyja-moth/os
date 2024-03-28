pub use crate::{
    exit::QemuExitCode,
    print, println, serial_print, serial_println,
    vga_buffer::{
        colors::{Color, ColorCode},
        print_macro::WRITER,
        writer::Writer,
        Buffer, ScreenChar,
    },
};
pub(crate) use core::{
    fmt::{Arguments, Write},
    panic::PanicInfo,
};
pub(crate) use lazy_static::lazy_static;
pub(crate) use spin::Mutex as SpinMutex;
pub(crate) use uart_16550::SerialPort;
pub(crate) use volatile::Volatile;
