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
impl TryFrom<u8> for ColorCode {
    type Error = InvalidColor;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self {
            foreground: (value | 0x0f).try_into()?,
            background: ((value | 0xf0) >> 4).try_into()?,
        })
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

#[derive(Debug)]
pub struct InvalidColor;

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
impl TryFrom<u8> for Color {
    type Error = InvalidColor;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Black),
            1 => Ok(Self::Blue),
            2 => Ok(Self::Green),
            3 => Ok(Self::Cyan),
            4 => Ok(Self::Red),
            5 => Ok(Self::Magenta),
            6 => Ok(Self::Brown),
            7 => Ok(Self::LightGray),
            8 => Ok(Self::DarkGray),
            9 => Ok(Self::LightBlue),
            10 => Ok(Self::LightGreen),
            11 => Ok(Self::LightCyan),
            12 => Ok(Self::LightRed),
            13 => Ok(Self::Pink),
            14 => Ok(Self::Yellow),
            15 => Ok(Self::White),
            _ => Err(InvalidColor),
        }
    }
}
