use std::io::{self, stdout, ErrorKind, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
    ErrorKind as CrosstermError,
};

use super::Map;
use super::{
    element::{Element, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
    Point,
};

pub struct DisplayController<'dimensions> {
    dimensions: &'dimensions Point,
    display: Map<'dimensions>,
    default_element: Element,
    pub target: io::Stdout,
}

pub enum Direction {
    Vertical,
    Horizontal,
}

const BORDER_ELEMENT: Element = Element::new('x', Color::Black, Color::Black);
const PADDING: Point = Point::new(10, 10);

// pub enum DisplayControllerError {
//     IndexOutOfRange,
// }

impl<'dimensions> DisplayController<'dimensions> {
    /// Creates a new display controller, a display controller fills the entire screen but the provided dimensions will be the controllable area
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The controllable area
    ///
    /// ```
    pub fn new(dimensions: &'dimensions Point) -> Result<Self, CrosstermError> {
        let (columns, rows) = size().unwrap();

        if dimensions.x > rows.into() || dimensions.y > columns.into() {
            panic!("Invalid dimensions");
        }

        // let tmp_dimensions = Point::new(rows as u32, columns as u32);

        let mut controller = DisplayController {
            display: Map::new(&dimensions),
            target: stdout(),
            dimensions: &dimensions,
            default_element: Element::default(),
        };

        controller.setup().draw_borders()?.print_display().flush();

        Ok(controller)
    }

    fn draw_borders(&mut self) -> Result<&mut Self, CrosstermError> {
        self.draw_rect(
            &Point::new(0, 0),
            self.dimensions,
            Element::new('x', Color::Blue, Color::Green),
        )
    }

    fn setup(&mut self) -> &mut Self {
        queue!(self.target, EnterAlternateScreen, Hide).unwrap();

        self
    }

    /// Flushing the target publishes all queued writes
    fn flush(&mut self) -> &mut Self {
        self.target.flush().unwrap();

        self
    }

    pub fn reset_cursor(&mut self) -> &mut Self {
        queue!(
            self.target,
            SetForegroundColor(DEFAULT_FOREGROUND),
            SetBackgroundColor(DEFAULT_BACKGROUND),
            MoveTo(0, 0)
        )
        .unwrap();

        self
    }

    fn draw_rect(
        &mut self,
        start_position: &Point,
        dimensions: &Point,
        element: Element,
    ) -> Result<&mut Self, CrosstermError> {
        self.draw_line(element, dimensions.x, start_position, Direction::Horizontal)?
            .draw_line(
                element,
                dimensions.x,
                &start_position.addY(dimensions.y),
                Direction::Horizontal,
            )?
            .draw_line(element, dimensions.y, start_position, Direction::Vertical)?
            .draw_line(
                element,
                dimensions.y,
                &start_position.addX(dimensions.x),
                Direction::Vertical,
            )?;

        Ok(self)
    }

    // TODO: Add docs describing that the line draws from top->bottom
    pub fn draw_line(
        &mut self,
        element: Element,
        len: u32,
        start_position: &Point,
        direction: Direction,
    ) -> Result<&mut Self, CrosstermError> {
        for position_change in 0..len {
            let new_position = match direction {
                Direction::Horizontal => start_position.addX(position_change),
                Direction::Vertical => start_position.addY(position_change),
            };

            self.draw_item(element, &new_position)?;
        }

        Ok(self)
    }

    fn draw_item(
        &mut self,
        element: Element,
        position: &Point,
    ) -> Result<&mut Self, CrosstermError> {
        // if position.x > self.dimensions.x || position.y > self.dimensions.y {
        //     panic!("Out of range requested");
        // }

        let row = self
            .display
            .map
            .get(position.x as usize)
            .ok_or(ErrorKind::OutOfMemory)?;

        // // dbg!(self.display.map.len());

        // if row.is_none() {
        //     dbg!(position.x);
        // }

        // match row[position.y as usize] {
        //     Some(mut existing_element) => {
        //         // existing_element = element;
        //     }
        //     None => {}
        // }

        // *row[position.y as usize] = Some(element);

        // let mut existing_element = row.get(position.y as usize).unwrap().as_ref();

        // existing_element = Some(&element);

        Ok(self)

        // // dbg!(col);

        // // match col.get(position.y).as_mut() {
        // //     Some(&mut existing_item) => *existing_item = *element,
        // // }

        // let existing_item = &mut col.get(position.y).unwrap();

        // // Dereference the value so we assign to it
        // *existing_item = element;
    }

    pub fn print_display(&mut self) -> &mut Self {
        self.reset_cursor();

        for row in self.display.map.iter() {
            for element in row.iter() {
                match element {
                    Some(element) => {
                        DisplayController::print_element(&mut self.target, element, None);
                    }
                    None => {
                        DisplayController::print_element(
                            &mut self.target,
                            &self.default_element,
                            None,
                        );
                    }
                }
            }
        }

        self
    }

    pub fn print_element(
        target: &mut io::Stdout,
        element: &Element,
        move_to: Option<&Point>,
    ) -> Result<(), CrosstermError> {
        if let Some(move_to_destination) = move_to {
            queue!(
                target,
                MoveTo(move_to_destination.x as u16, move_to_destination.y as u16)
            )?;
        };

        queue!(
            target,
            SetForegroundColor(element.foreground),
            SetBackgroundColor(element.background),
            Print(element.value)
        )?;

        Ok(())
    }

    pub fn close(target: &mut io::Stdout) {
        disable_raw_mode().unwrap();
        execute!(target, LeaveAlternateScreen, Show).unwrap();
    }
}
