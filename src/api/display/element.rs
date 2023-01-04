use crossterm::style::Color;

pub const DEFAULT_BACKGROUND: Color = Color::Blue;
pub const DEFAULT_FOREGROUND: Color = Color::Blue;

#[derive(Clone, Copy, PartialEq)]
pub struct Element {
    pub value: char,
    pub background: Color,
    pub foreground: Color,
}

impl Element {
    pub fn new() -> Self {
        Self {
            value: ' ',
            background: DEFAULT_BACKGROUND,
            foreground: DEFAULT_FOREGROUND,
        }
    }

    pub const fn from(value: char, background: Color, foreground: Color) -> Self {
        Self {
            value,
            background,
            foreground,
        }
    }
}
