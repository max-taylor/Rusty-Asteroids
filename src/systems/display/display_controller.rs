use std::io::{self, stdout};

use crossterm::{
    cursor::{position, Hide, MoveTo, Show},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{
    element::{Element, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
    Point,
};

pub struct DisplayController {
    dimensions: Point,
    display: Vec<Vec<Element>>,
    target: io::Stdout,
}

impl DisplayController {
    pub fn new(dimensions: Point) -> DisplayController {
        let (columns, rows) = size().unwrap();

        if dimensions.x > rows as usize || dimensions.y > columns as usize {
            panic!("Invalid dimensions");
        }

        let mut controller = DisplayController {
            display: vec![vec![Element::new(); dimensions.x]; dimensions.y],
            target: stdout(),
            dimensions,
        };

        controller.setup().reset_cursor().draw();

        controller
    }

    fn setup(&mut self) -> &mut Self {
        queue!(self.target, EnterAlternateScreen, Hide).unwrap();

        self
    }

    // pub fn draw_vertical_line(&mut self, value: char) -> &mut Self {
    //     // execute!()
    //     self
    // }

    pub fn reset_cursor(&mut self) -> &mut Self {
        queue!(
            self.target,
            // SetForegroundColor(DEFAULT_FOREGROUND),
            // SetBackgroundColor(DEFAULT_BACKGROUND),
            MoveTo(0, 0)
        )
        .unwrap();

        self
    }

    pub fn print_element(&mut self) {
        let (x, y) = position().unwrap();

        // if x > self.dimensions.x {
        //     queue!(self.target, MoveTo())
        // }

        // self
    }

    pub fn draw(&mut self) -> &mut Self {
        self.reset_cursor();

        for row in self.display.iter() {
            for element in row.iter() {
                queue!(
                    self.target,
                    SetForegroundColor(element.foreground),
                    SetBackgroundColor(element.background),
                    Print(element.value)
                )
                .unwrap();
            }
        }

        self
    }

    pub fn close(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.target, LeaveAlternateScreen, Show).unwrap();
    }
}
