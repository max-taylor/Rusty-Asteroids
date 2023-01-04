use crossterm::terminal::size;

use crate::api::display::{DisplayControllerError, Point};

pub struct Display<'dimensions> {
    dimensions: &'dimensions Point,
    offset: Point,
}

impl<'dimensions> Display<'dimensions> {
    pub fn new(dimensions: &'dimensions Point) -> Result<Self, DisplayControllerError> {
        let (rows, columns) = size().unwrap();

        if dimensions.height > columns.into() || dimensions.width > rows.into() {
            return Err(DisplayControllerError::DisplayTooSmallForDimensions);
        }

        // Display is the size of the screen
        let screen_size = Point::new(rows as u32, columns as u32);

        Ok(Display {
            dimensions,
            // The offset is where all drawing will be done, this is the center of the terminal screen
            offset: (screen_size - *dimensions) / Point::new(2, 2),
        })
    }
}
