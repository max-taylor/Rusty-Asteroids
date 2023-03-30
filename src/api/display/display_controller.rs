use crossterm::style::Color;
use crossterm::terminal::size;
use uuid::Uuid;

use crate::app::GameState;
use crate::components::{Drawable, DrawableState, Health};
use crate::systems::EntityController;
use crate::user_display::{HEART, NUMBER_VECTOR, X};

use super::element::DEFAULT_BACKGROUND;
use super::{create_map, map_from_str, Element, Map, Point};
use super::{display_controller_error::DisplayControllerError, Layout};

pub struct DisplayController {
    entity_drawable_offset: Point<i64>,
    pub layout: Layout,
    numbers: Vec<Map>,
    screen_size: Point<i64>,
}

type DisplayControllerResult<T> = Result<T, DisplayControllerError>;

pub const GAME_DETAILS_BOX_WIDTH: u64 = 43;
const BOX_PADDING: u64 = 1;

pub const MINIMUM_SCREEN_WIDTH: u64 = (GAME_DETAILS_BOX_WIDTH + BOX_PADDING) * 2;

pub fn get_screen_size() -> Point<i64> {
    let (rows, columns) = size().unwrap();

    Point::new(rows as i64, columns as i64)
}

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

        for idx in 0..10 {
            numbers[idx] = map_from_str(NUMBER_VECTOR[idx], Color::Black);
        }

        Ok(DisplayController {
            layout: Layout::new(&dimensions, None),
            // The offset is where all drawing will be done, this is the center of the terminal screen
            entity_drawable_offset,
            numbers,
            screen_size: get_screen_size(),
        })
    }

    pub fn draw_game_state(
        &mut self,
        game_state: &GameState,
        lives: u32,
    ) -> DisplayControllerResult<&mut Self> {
        self.draw_lives(lives)?;
        self.draw_score(game_state.score)?;
        // self.layout.draw_map(
        //     &self.numbers[lives as usize],
        //     Point::new(20, 2),
        //     &Default::default(),
        // )?;

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

    fn draw_lives(&mut self, lives: u32) -> DisplayControllerResult<()> {
        let heart_map = map_from_str(HEART, Color::Red);

        self.layout.draw_rect(
            &Point::new(BOX_PADDING as i64, 0),
            &Point::new(GAME_DETAILS_BOX_WIDTH as i64, 8),
            Element::new('❤', DEFAULT_BACKGROUND, Color::Red),
        )?;

        self.layout
            .draw_map(&heart_map, Point::new(5, 2), &Default::default())?;

        self.layout.draw_map(
            &map_from_str(X, Color::Black),
            Point::new(19, 3),
            &Default::default(),
        )?;

        self.draw_u32(lives, Point::new(27, 2))?;

        self.draw_str("Lives", DEFAULT_BACKGROUND, Color::Red, Point::new(19, 1))?;

        Ok(())
    }

    fn draw_score(&mut self, score: u64) -> DisplayControllerResult<()> {
        let start_position = Point {
            height: 0,
            width: self.screen_size.width - GAME_DETAILS_BOX_WIDTH as i64,
        };

        self.layout.draw_rect(
            &start_position,
            &Point::new(GAME_DETAILS_BOX_WIDTH as i64, 8),
            Element::new('⦿', DEFAULT_BACKGROUND, Color::Cyan),
        )?;

        self.draw_u32(score as u32, Point::new(start_position.width + 5, 2))?;

        self.draw_str(
            "Score",
            DEFAULT_BACKGROUND,
            Color::Cyan,
            Point::new(start_position.width + 19, 1),
        )?;

        // self.draw_str("Score", DEFAULT_BACKGROUND, Color::Red, Point::new(80, 1))?;

        Ok(())
    }

    pub fn draw_u32(
        &mut self,
        numbers: u32,
        start_position: Point<i64>,
    ) -> DisplayControllerResult<()> {
        let mut location = start_position;
        for char in numbers.to_string().chars() {
            let number_val: u32 = char.to_digit(10).unwrap();

            self.layout.draw_map(
                &map_from_str(NUMBER_VECTOR[number_val as usize], Color::Red),
                location,
                &Default::default(),
            )?;

            location = location.add_width(7);
        }

        Ok(())
    }

    pub fn draw_str(
        &mut self,
        str: &str,
        background: Color,
        foreground: Color,
        start_position: Point<i64>,
    ) -> DisplayControllerResult<()> {
        let elements: Vec<Option<Element>> = str
            .chars()
            .map(|char| Some(Element::new(char, background, foreground)))
            .collect();

        self.layout.draw_element_array(elements, &start_position)?;

        Ok(())
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
