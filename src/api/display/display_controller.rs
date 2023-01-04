use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
    ErrorKind as CrosstermError,
};

use crate::components::Drawable;

use super::{display_controller_error::DisplayControllerError, Map};
use super::{
    element::{Element, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
    Point,
};

pub struct DisplayController {
    offset: Point,
    // entities: Vec<&Point>,
    pub display: Map,
    pub target: io::Stdout,
}

type DisplayControllerResult<T> = Result<T, DisplayControllerError>;

// TODO -> This x/y business is annoying, change to rows/columns or make it clearer

impl DisplayController {
    /// Creates a new display controller, a display controller fills the entire screen but the provided dimensions will be the controllable area
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The controllable area
    ///
    /// ```
    pub fn new(dimensions: &Point) -> Result<Self, DisplayControllerError> {
        let (rows, columns) = size().unwrap();

        if dimensions.height > columns.into() || dimensions.width > rows.into() {
            return Err(DisplayControllerError::DisplayTooSmallForDimensions);
        }

        // Display is the size of the screen
        let screen_size = Point::new(rows as u32, columns as u32);

        Ok(DisplayController {
            display: Map::new(&screen_size, None),
            target: stdout(),
            // The offset is where all drawing will be done, this is the center of the terminal screen
            offset: (screen_size - *dimensions) / Point::new(2, 2),
        })

        // controller.setup().print_display()?.flush();
    }

    pub fn start(&mut self) -> &mut Self {
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

    pub fn draw_drawable(&mut self, drawable: &Drawable) -> DisplayControllerResult<&mut Self> {
        let base_location = drawable.location + self.offset;
        // Iterate over each row in the map
        for (num_row, drawable_row) in drawable.map.map.iter().enumerate() {
            // Then each column in the row
            for num_column in 0..drawable_row.len() {
                if let Some(has_element) = drawable_row[num_column] {
                    let updated_position = base_location
                        .add_width(num_column as u32)
                        .add_height(num_row as u32);

                    self.display.draw_item(has_element, &updated_position)?;
                }
            }
        }

        Ok(self)
    }

    pub fn print_element(
        target: &mut io::Stdout,
        element: &Element,
        move_to: Option<&Point>,
    ) -> Result<(), DisplayControllerError> {
        if let Some(move_to_destination) = move_to {
            queue!(
                target,
                MoveTo(
                    move_to_destination.width as u16,
                    move_to_destination.height as u16
                )
            )
            .map_err(DisplayControllerError::from_crossterm_error)?;
        };

        queue!(
            target,
            SetForegroundColor(element.foreground),
            SetBackgroundColor(element.background),
            Print(element.value)
        )
        .map_err(DisplayControllerError::from_crossterm_error)?;

        Ok(())
    }

    pub fn print_display(&mut self) -> Result<&mut Self, DisplayControllerError> {
        self.reset_cursor();

        for row in self.display.map.iter() {
            for element in row.iter() {
                match element {
                    Some(element) => {
                        DisplayController::print_element(&mut self.target, element, None)?;
                    }
                    None => {
                        DisplayController::print_element(
                            &mut self.target,
                            &Default::default(),
                            None,
                        )?;
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn close(target: &mut io::Stdout) -> Result<(), CrosstermError> {
        disable_raw_mode()?;
        execute!(target, LeaveAlternateScreen, Show)?;

        Ok(())
    }
}
