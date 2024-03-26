use crate::prelude::*;

pub struct Writer {
    pub column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
impl Default for Writer {
    fn default() -> Self {
        Self {
            column_position: 0,
            color_code: ColorCode::default(),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }
}
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_char(byte as char),
                _ => self.write_char(0xfe as char),
            }?
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        if c == '\n' || self.current_line_full() {
            self.new_line()
        } else {
            self.set_current_char(ScreenChar {
                ascii_character: c as u8,
                color_code: self.color_code.clone().into(),
            });
            self.advance_char_position();
        };
        Ok(())
    }
}
impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.color_code.set_background(color);
        self
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.color_code.set_background(color);
    }

    pub fn background_color(&self) -> &Color {
        self.color_code.background()
    }

    pub fn with_foreground_color(mut self, color: Color) -> Self {
        self.color_code.set_foreground(color);
        self
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.color_code.set_foreground(color);
    }

    pub fn foreground(&self) -> &Color {
        self.color_code.foreground()
    }

    pub fn with_char_position(mut self, value: usize) -> Self {
        self.column_position = value;
        self
    }

    pub fn set_char_position(&mut self, value: usize) {
        self.column_position = value;
    }

    pub fn char_position(&self) -> &usize {
        &self.column_position
    }

    pub fn char_position_mut(&mut self) -> &mut usize {
        &mut self.column_position
    }

    pub fn advance_char_position(&mut self) {
        self.column_position += 1;
    }

    pub fn reset_char_position(&mut self) {
        self.column_position = 0;
    }

    pub fn current_row(&self) -> &[Volatile<ScreenChar>] {
        let row = Buffer::HEIGHT - 1;

        self.buffer.chars[row].as_ref()
    }

    pub fn current_row_mut(&mut self) -> &mut [Volatile<ScreenChar>] {
        let row = Buffer::HEIGHT - 1;

        self.buffer.chars[row].as_mut()
    }

    pub fn get_row(&self, row: usize) -> &[Volatile<ScreenChar>] {
        self.buffer.chars[row].as_ref()
    }

    pub fn get_row_mut(&mut self, row: usize) -> &mut [Volatile<ScreenChar>] {
        self.buffer.chars[row].as_mut()
    }

    pub fn set_current_row(&mut self, value: &[ScreenChar]) {
        let current_row = self.current_row_mut();
        current_row
            .iter_mut()
            .zip(value)
            .for_each(|(col, new_value)| col.write(*new_value));
    }

    pub fn set_current_row_volatile(&mut self, value: &mut [Volatile<ScreenChar>]) {
        let current_row = self.current_row_mut();
        current_row
            .iter_mut()
            .zip(value)
            .for_each(|(col, new_value)| col.write(new_value.read()));
    }

    pub fn clear_current_row(&mut self) {
        let blank = [ScreenChar::default(); Buffer::WIDTH];
        self.set_current_row(&blank);
    }

    pub fn set_row(&mut self, row: usize, value: &[ScreenChar]) {
        let row = self.get_row_mut(row);
        row.iter_mut()
            .zip(value)
            .for_each(|(col, value)| col.write(*value));
    }

    pub fn set_row_volatile(&mut self, row: usize, value: &[Volatile<ScreenChar>]) {
        let current_row = self.get_row_mut(row);
        current_row
            .iter_mut()
            .zip(value)
            .for_each(|(col, new_value)| col.write(new_value.read()));
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = [ScreenChar::default(); Buffer::WIDTH];
        self.set_row(row, &blank);
    }

    pub fn current_char(&self) -> ScreenChar {
        let row = Buffer::HEIGHT - 1;
        let col = self.column_position;

        self.buffer.chars[row][col].read()
    }

    pub fn current_char_mut(&mut self) -> &mut Volatile<ScreenChar> {
        let row = Buffer::HEIGHT - 1;
        let col = self.column_position;

        &mut self.buffer.chars[row][col]
    }

    pub fn get_char(&self, row: usize, column: usize) -> ScreenChar {
        self.buffer.chars[row][column].read()
    }

    pub fn get_char_mut(&mut self, row: usize, column: usize) -> &mut Volatile<ScreenChar> {
        &mut self.buffer.chars[row][column]
    }

    pub fn set_current_char(&mut self, new_character: ScreenChar) {
        let character = self.current_char_mut();

        character.write(new_character);
    }

    pub fn set_char(&mut self, row: usize, column: usize, value: ScreenChar) {
        self.get_char_mut(row, column).write(value)
    }

    pub fn current_line_full(&self) -> bool {
        self.column_position >= Buffer::WIDTH
    }

    fn new_line(&mut self) {
        for row in 1..Buffer::HEIGHT {
            for col in 0..Buffer::WIDTH {
                let character = self.get_char(row, col);
                self.get_char_mut(row - 1, col).write(character);
            }
        }
        self.clear_current_row();
        self.column_position = 0;
    }
}
