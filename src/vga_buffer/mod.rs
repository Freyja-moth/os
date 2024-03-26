pub mod colors;
pub mod print_macro;
pub mod writer;

use crate::prelude::*;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; Self::WIDTH]; Self::HEIGHT],
}
impl Buffer {
    pub const HEIGHT: usize = 25;
    pub const WIDTH: usize = 80;
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}
impl Default for ScreenChar {
    fn default() -> Self {
        Self {
            ascii_character: b' ',
            color_code: Default::default(),
        }
    }
}
impl ScreenChar {
    pub fn new(character: char, color: ColorCode) -> Self {
        Self {
            ascii_character: character as u8,
            color_code: color.into(),
        }
    }
}
