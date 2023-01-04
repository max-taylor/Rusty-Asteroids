use crossterm::terminal::size;

use crate::components::Drawable;

use super::Point;
use super::{display_controller_error::DisplayControllerError, Layout};

pub struct DisplayController {
    offset: Point,
    // entities: Vec<&Point>,
    pub layout: Layout,
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
    pub fn new(dimensions: &Point, enable_offset: bool) -> Result<Self, DisplayControllerError> {
        let (rows, columns) = size().unwrap();

        if dimensions.height > columns.into() || dimensions.width > rows.into() {
            return Err(DisplayControllerError::DisplayTooSmallForDimensions);
        }

        // Display is the size of the screen
        let screen_size = Point::new(rows as u32, columns as u32);

        let offset = match enable_offset {
            true => (screen_size - *dimensions) / Point::new(2, 2),
            false => Default::default(),
        };

        Ok(DisplayController {
            layout: Layout::new(&screen_size, None),
            // The offset is where all drawing will be done, this is the center of the terminal screen
            offset,
        })
    }

    pub fn draw_drawable(
        &mut self,
        drawable: &impl Drawable,
    ) -> DisplayControllerResult<&mut Self> {
        let drawable_state = drawable.get_drawable_state();
        let base_location = drawable_state.location + self.offset;
        // Iterate over each row in the map
        for (num_row, drawable_row) in drawable_state.layout.map.iter().enumerate() {
            // Then each column in the row
            for num_column in 0..drawable_row.len() {
                if let Some(has_element) = drawable_row[num_column] {
                    let updated_position = base_location
                        .add_width(num_column as u32)
                        .add_height(num_row as u32);

                    self.layout.draw_item(has_element, &updated_position)?;
                }
            }
        }

        Ok(self)
    }
}
