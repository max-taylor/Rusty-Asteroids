use std::convert::From;
use std::io::{self, stdout, Write};

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

pub struct DisplayController<'dimensions> {
    dimensions: &'dimensions Point,
    display: Vec<Vec<Element>>,
    target: io::Stdout,
}

impl From<&Point> for MoveTo {
    fn from(point: &Point) -> Self {
        Self(point.x.try_into().unwrap(), point.y.try_into().unwrap())
    }
}

pub enum Direction {
    Vertical,
    Horizontal,
}

const BORDER_ELEMENT: Element = Element::from('x', Color::Black, Color::Black);
const PADDING: Point = Point::new(10, 10);

impl<'dimensions> DisplayController<'dimensions> {
    pub fn new(dimensions: &'dimensions Point) -> Self {
        let (columns, rows) = size().unwrap();

        if dimensions.x > rows as usize || dimensions.y > columns as usize {
            panic!("Invalid dimensions");
        }

        let mut controller = DisplayController {
            display: vec![vec![Element::new(); rows.into()]; columns.into()],
            target: stdout(),
            dimensions: &dimensions,
        };

        controller.setup().draw_borders().draw_display();

        // Publish all queued writes
        controller.target.flush().unwrap();

        controller
    }

    fn draw_borders(&mut self) -> &mut Self {
        self.draw_rect(
            &Point::new(0, 0),
            self.dimensions,
            Element::from('x', Color::Blue, Color::Green),
        )
    }

    fn setup(&mut self) -> &mut Self {
        queue!(self.target, EnterAlternateScreen, Hide).unwrap();

        self
    }

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

    fn draw_rect(
        &mut self,
        start_position: &Point,
        dimensions: &Point,
        element: Element,
    ) -> &mut Self {
        self.draw_line(
            &element,
            dimensions.x,
            start_position,
            Direction::Horizontal,
        )
        .draw_line(
            &element,
            dimensions.x,
            &start_position.addY(dimensions.y),
            Direction::Horizontal,
        )
        .draw_line(&element, dimensions.y, start_position, Direction::Vertical)
        .draw_line(
            &element,
            dimensions.y,
            &start_position.addX(dimensions.x),
            Direction::Vertical,
        )
    }

    // TODO: Add docs describing that the line draws from top->bottom
    pub fn draw_line(
        &mut self,
        element: &Element,
        len: usize,
        start_position: &Point,
        direction: Direction,
    ) -> &mut Self {
        for position_change in 0..len {
            let new_position = match direction {
                Direction::Horizontal => start_position.addX(position_change),
                Direction::Vertical => start_position.addY(position_change),
            };

            self.draw_item(&new_position, element);
        }

        self
    }

    fn draw_item(&mut self, position: &Point, element: &Element) -> &mut Self {
        // if position.x > self.dimensions.x || position.y > self.dimensions.y {
        //     panic!("Out of range requested");
        // }

        // dbg!(position.x);

        let col = self.display.get(position.x).unwrap();

        // dbg!(col);

        // match col.get(position.y).as_mut() {
        //     Some(&mut existing_item) => *existing_item = *element,
        // }

        let existing_item = &mut col.get(position.y).unwrap();

        // Dereference the value so we assign to it
        *existing_item = element;

        // queue!(
        //     self.target,
        //     MoveTo::from(&(*position + PADDING)),
        //     Print(element.value)
        // )
        // .unwrap();

        self
    }

    pub fn draw_display(&mut self) -> &mut Self {
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
