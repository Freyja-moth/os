use crate::prelude::*;

lazy_static! {
    pub static ref WRITER: SpinMutex<Writer> =
        SpinMutex::new(Writer::default().with_foreground_color(Color::Red));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print_macro::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
