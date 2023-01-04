use super::element::Element;

use crossterm::{execute, queue, style::Color};

pub struct DisplayController {
    display: Vec<Vec<Element>>,
}

impl DisplayController {
    pub fn new(x: usize, y: usize) -> DisplayController {
        let mut display = vec![vec![' '; x]; y];

        DisplayController { display: vec![] }
    }

    pub fn draw_vertical_line(&mut self, value: char) -> &mut Self {
        // execute!()
        self
    }

    pub fn draw(&mut self) -> &mut Self {
        for row in self.display.iter() {
            for item in row.iter() {}
        }

        self
    }
}
