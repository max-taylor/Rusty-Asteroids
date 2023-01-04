use crossterm::terminal::size;

use crate::components::Drawable;
use crate::helpers::get_is_position_outside_dimensions;

use super::Point;
use super::{display_controller_error::DisplayControllerError, Layout};

pub struct DisplayController {
    offset: Point<i64>,
    pub layout: Layout,
}

type DisplayControllerResult<T> = Result<T, DisplayControllerError>;

impl DisplayController {
    /// Creates a new display controller, a display controller fills the entire screen but the provided dimensions will be the controllable area
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The controllable area, if None then the entire screen is used
    ///
    /// ```
    pub fn new(dimensions: Option<&Point<i64>>) -> Result<Self, DisplayControllerError> {
        let (rows, columns) = size().unwrap();

        let screen_size = &Point::new(rows as i64, columns as i64);

        let (dimensions, offset) = match dimensions {
            Some(has_dimensions) => {
                if has_dimensions.height > columns.into() || has_dimensions.width > rows.into() {
                    return Err(DisplayControllerError::DisplayTooSmallForDimensions);
                }
                let offset = (*screen_size - *has_dimensions) / Point::new(2, 2);

                (has_dimensions, offset)
            }
            // If no dimensions provided use the screen_size and set a 0,0 offset
            None => (screen_size, Default::default()),
        };

        Ok(DisplayController {
            layout: Layout::new(&dimensions, None),
            // The offset is where all drawing will be done, this is the center of the terminal screen
            offset,
        })
    }

    /// This method handles drawing drawable elements, it also skips over the drawing of an element if it is outside the range
    ///
    pub fn draw_drawable(
        &mut self,
        drawable: &impl Drawable,
    ) -> DisplayControllerResult<(&mut Self, bool)> {
        let drawable_state = drawable.get_drawable_state();
        let base_location = drawable_state.location + self.offset;
        let mut has_drawn_drawable = false;
        // Iterate over each row in the map
        for (num_row, drawable_row) in drawable_state.layout.map.iter().enumerate() {
            // Then each column in the row
            for num_column in 0..drawable_row.len() {
                if let Some(has_element) = drawable_row[num_column] {
                    let updated_position = base_location
                        .add_width(num_column as i64)
                        .add_height(num_row as i64);

                    // Check if position is outside of dimension range
                    if get_is_position_outside_dimensions(
                        &self.layout.dimensions,
                        &updated_position,
                    ) {
                        continue;
                    }

                    has_drawn_drawable = true;

                    self.layout.draw_item(has_element, &updated_position)?;
                }
            }
        }

        Ok((self, has_drawn_drawable))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::display::{Element, Layout, Point},
        components::{Drawable, DrawableState, DrawableType},
    };

    use super::DisplayController;

    const WIDTH: i64 = 30;
    const HEIGHT: i64 = 20;
    const DIMENSIONS: &Point<i64> = &Point::new(WIDTH, HEIGHT);

    struct MockDrawble {
        drawable_state: DrawableState,
    }

    impl MockDrawble {
        pub fn new() -> Self {
            Self {
                drawable_state: DrawableState::new(
                    Layout::new(&Point::new(2, 2), Some(Element::default())),
                    Point::new(-5, -5),
                    DrawableType::Player,
                    None,
                ),
            }
        }
    }

    impl Drawable for MockDrawble {
        fn set_position(&mut self, updated_position: Point<i64>) -> &mut Self {
            todo!()
        }

        fn get_drawable_state(&self) -> &DrawableState {
            &self.drawable_state
        }
    }

    #[test]
    fn it_can_handle_drawable_outside_dimensions() {
        // let drawable = Drawable::
        let mut display_controller = DisplayController::new(None).unwrap();

        let result = display_controller.draw_drawable(&MockDrawble::new());

        assert!(result.err().is_none());
    }
}
