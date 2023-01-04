use crossterm::style::Color;

pub const DEFAULT_BACKGROUND: Color = Color::White;
pub const DEFAULT_FOREGROUND: Color = Color::White;

#[derive(Clone, Copy)]
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
}
