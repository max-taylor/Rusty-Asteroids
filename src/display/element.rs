use crossterm::style::Color;

pub struct Element {
    pub value: char,
    pub background: Color,
    pub foreground: Color,
}

impl Element {
    pub fn new() -> Self {
        Self {
            value: ' ',
            background: Color::Black,
            foreground: Color::White,
        }
    }
}
