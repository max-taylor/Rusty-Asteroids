use crossterm::style::Color;
use crossterm::terminal::size;
use uuid::Uuid;

use crate::app::GameState;
use crate::components::{Drawable, DrawableState, Health};
use crate::helpers::{
    get_is_position_outside_dimensions, get_is_position_outside_dimensions_with_offset,
};
use crate::systems::EntityController;
use crate::user_display::{HEART, NUMBER_VECTOR};

use super::element::{parse_str_to_element_array, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND};
use super::{create_map, map_from_str, Element, Map, Point};
use super::{display_controller_error::DisplayControllerError, Layout};

pub struct DisplayController {
    entity_drawable_offset: Point<i64>,
    pub layout: Layout,
    numbers: Vec<Map>,
}

type DisplayControllerResult<T> = Result<T, DisplayControllerError>;

pub fn get_screen_size() -> Point<i64> {
    let (rows, columns) = size().unwrap();

    Point::new(rows as i64, columns as i64)
}

const LIFE_ELEMENT: Element = Element::new('♥', DEFAULT_BACKGROUND, Color::Red);

impl DisplayController {
    /// Creates a new display controller, a display controller fills the entire screen but the provided dimensions will be the controllable area
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The controllable area, if None then the entire screen is used
    ///
    /// ```
    pub fn new(
        dimensions: Point<i64>,
        entity_drawable_offset: Point<i64>,
    ) -> Result<Self, DisplayControllerError> {
        let mut numbers: Vec<Map> = vec![create_map(&Default::default(), None); 10];

        for idx in 0..9 {
            numbers[idx] = map_from_str(NUMBER_VECTOR[idx], Color::Black);
        }

        Ok(DisplayController {
            layout: Layout::new(&dimensions, None),
            // The offset is where all drawing will be done, this is the center of the terminal screen
            entity_drawable_offset,
            numbers,
        })
    }

    pub fn draw_game_state(
        &mut self,
        game_state: &GameState,
        lives: u32,
    ) -> DisplayControllerResult<&mut Self> {
        let heart_map = map_from_str(HEART, Color::Red);

        self.layout
            .draw_map(&heart_map, Point::new(5, 2), &Default::default())?;

        self.layout.draw_map(
            &self.numbers[lives as usize],
            Point::new(20, 2),
            &Default::default(),
        )?;

        // self.layout
        //     .draw_str(HEART, &Point::new(5, 2), None, Some(Color::Red))?;

        // let life_elements = vec![Some(LIFE_ELEMENT); lives as usize];

        // self.layout.draw_element_array(life_elements)?;

        // Convert the score string into an array of elements for simple printing to the display
        // let score_array = parse_str_to_element_array(&game_state.score.to_string(), None, None);

        // self.layout.draw_element_array(
        //     score_array,
        //     &Point::new(self.layout.dimensions.width - 5, 2),
        // )?;

        Ok(self)
    }

    /// This method handles drawing drawable elements, it also skips over the drawing of an element if it is outside the range
    pub fn draw_drawable(
        &mut self,
        drawable_state: &DrawableState,
    ) -> DisplayControllerResult<(&mut Self, bool)> {
        let base_location = drawable_state.location + self.entity_drawable_offset;

        let has_drawn_drawable = self.layout.draw_map(
            &drawable_state.layout.map,
            base_location,
            &self.entity_drawable_offset,
        )?;

        Ok((self, has_drawn_drawable))
    }

    /// Draws the entities within a given entity controller. It also removes items from the entities array if they are outside of the drawable dimensions.
    /// This is primarily used for the Bullet controller and asteroid controller
    pub fn draw_entity_controller_items<T: Drawable + Health>(
        &mut self,
        entity_controller: &mut EntityController<T>,
    ) -> &mut Self {
        // Create an array of delete uuid's and iterate over them after we determine which ones are to be deleted. This is because we can't mutable use the entity_controller within the immutable iterator
        let mut delete_uuids: Vec<Uuid> = vec![];

        for entity in entity_controller.get_all_drawable_states() {
            let result = self.draw_drawable(entity);

            let (_, did_draw) = result.unwrap();

            if !did_draw {
                delete_uuids.push(entity.uuid);
            }
        }

        for uuid in delete_uuids {
            entity_controller.delete_entity(uuid);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::display::{display_controller::get_screen_size, Element, Layout, Point},
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
        let mut display_controller =
            DisplayController::new(get_screen_size(), Default::default()).unwrap();

        let result = display_controller.draw_drawable(&MockDrawble::new().get_drawable_state());

        assert!(result.err().is_none());
    }
}
