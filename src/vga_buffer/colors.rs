#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct ColorCode {
    foreground: Color,
    background: Color,
}
impl From<ColorCode> for u8 {
    fn from(value: ColorCode) -> Self {
        (value.background as u8) << 4 | value.foreground as u8
    }
}
impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    pub fn with_foreground(mut self, foreground: Color) -> Self {
        self.foreground = foreground;
        self
    }
    pub fn set_foreground(&mut self, foreground: Color) {
        self.foreground = foreground;
    }
    pub fn foreground(&self) -> &Color {
        &self.foreground
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }
    pub fn background(&self) -> &Color {
        &self.background
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum Color {
    #[default]
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}
