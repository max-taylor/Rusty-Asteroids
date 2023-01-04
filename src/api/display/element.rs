use crossterm::style::Color;

pub const DEFAULT_BACKGROUND: Color = Color::Black;
pub const DEFAULT_FOREGROUND: Color = Color::Blue;

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

pub fn parse_str_to_element_array(
    string_array: &str,
    background: Option<Color>,
    foreground: Option<Color>,
) -> Vec<Option<Element>> {
    let background = background.unwrap_or(DEFAULT_BACKGROUND);
    let foreground = foreground.unwrap_or(DEFAULT_FOREGROUND);

    let mut array = vec![None; string_array.len()];

    for (index, character) in string_array.chars().enumerate() {
        if character != ' ' {
            array[index] = Some(Element::new(character, background, foreground));
        }
    }

    array
}

impl Element {
    pub const fn new(value: char, background: Color, foreground: Color) -> Self {
        Self {
            value,
            background,
            foreground,
        }
    }

    // Creating a const version of the default method so it can be called outside methods
    pub const fn default() -> Self {
        Self {
            value: ' ',
            background: DEFAULT_BACKGROUND,
            foreground: DEFAULT_FOREGROUND,
        }
    }

    pub const fn new_default_colors(value: char) -> Self {
        Self {
            value,
            background: DEFAULT_BACKGROUND,
            foreground: DEFAULT_FOREGROUND,
        }
    }
}
