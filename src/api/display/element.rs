use crossterm::style::Color;

pub const DEFAULT_BACKGROUND: Color = Color::Black;
pub const DEFAULT_FOREGROUND: Color = Color::Black;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Element {
    pub value: char,
    pub background: Color,
    pub foreground: Color,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            value: ' ',
            background: DEFAULT_BACKGROUND,
            foreground: DEFAULT_FOREGROUND,
        }
    }
}

impl Element {
    pub const fn new(value: char, background: Color, foreground: Color) -> Self {
        Self {
            value,
            background,
            foreground,
        }
    }

    pub fn new_default_colors(value: char) -> Self {
        Self {
            value,
            background: DEFAULT_BACKGROUND,
            foreground: DEFAULT_FOREGROUND,
        }
    }
}
