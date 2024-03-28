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
            color_code: ColorCode::default().into(),
        }
    }
}
impl ScreenChar {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_character(mut self, character: char) -> Self {
        self.ascii_character = character as u8;
        self
    }
    pub fn set_character(&mut self, character: char) {
        self.ascii_character = character as u8;
    }
    pub fn character(&self) -> char {
        self.ascii_character as char
    }

    pub fn with_colorcode(mut self, color: ColorCode) -> Self {
        self.color_code = color.into();
        self
    }
    pub fn set_colorcode(&mut self, color: ColorCode) {
        self.color_code = color.into();
    }
    pub fn colorcode(&self) -> ColorCode {
        self.color_code.try_into().unwrap()
    }
}
