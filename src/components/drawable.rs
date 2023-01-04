use crate::api::display::{Layout, Point};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct DrawableState {
    pub layout: Layout,
    pub location: Point<i64>,
    pub velocity: Point<i64>,
    pub drawable_type: DrawableType,
    pub uuid: Uuid,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DrawableType {
    Player,
    // Damage of enemy
    Enemy(u32),
    Border,
    // Damage of ammunition
    Ammunition(u32),
}

impl DrawableState {
    pub fn new(
        layout: Layout,
        location: Point<i64>,
        drawable_type: DrawableType,
        velocity: Option<Point<i64>>,
    ) -> Self {
        Self {
            layout,
            location,
            drawable_type,
            velocity: velocity.unwrap_or(Default::default()),
            uuid: Uuid::new_v4(),
        }
    }
}

const PADDING_OFFSET: i64 = 1;

pub fn update_position_for_drawable_vec(
    drawable_vec: &mut Vec<impl Drawable>,
    game_loop_duration: u128,
) {
    drawable_vec.iter_mut().for_each(|bullet| {
        bullet.update_position(None, game_loop_duration);
    });
}

pub fn get_rated_velocity(velocity: Point<i64>, game_loop_duration: u128) -> Point<i64> {
    // RUST IS SO CLEAN, this seamless cast from i64 to point with .into();
    let rated_velocity = velocity * game_loop_duration.into() / (1000 as i64).into();

    // Use a minimum velocity of 1 if we round to 0
    fn get_safe_rated_value(rated_value: i64, provided_value: i64) -> i64 {
        if rated_value != 0 || provided_value == 0 {
            return rated_value;
        }

        if provided_value > 0 {
            return 1;
        }

        -1
    }

    Point {
        width: get_safe_rated_value(rated_velocity.width, velocity.width),
        height: get_safe_rated_value(rated_velocity.height, velocity.height),
    }
}

pub trait Drawable {
    fn set_position(&mut self, updated_position: Point<i64>) -> &mut Self;

    fn get_drawable_state(&self) -> &DrawableState;

    fn update_position(
        &mut self,
        dimensions: Option<&Point<i64>>,
        game_loop_duration: u128,
    ) -> &mut Self {
        let drawable_state = self.get_drawable_state();

        // Convert location to i64 so we can handle an underflow of the position; if the position goes below (0,0)
        let converted_location: Point<i64> = drawable_state.location.into();

        let mut updated_position: Point<i64> =
            converted_location + get_rated_velocity(drawable_state.velocity, game_loop_duration);

        // Block the player from moving outside the boundary
        if drawable_state.drawable_type == DrawableType::Player {
            if dimensions.is_none() {
                panic!("Missing required dimensions for rendering player");
            }

            if updated_position.height < PADDING_OFFSET {
                updated_position.height = PADDING_OFFSET;
            }

            if updated_position.width < PADDING_OFFSET {
                updated_position.width = PADDING_OFFSET;
            }

            let dimensions = dimensions.unwrap();

            let drawable_dimensions = drawable_state.layout.dimensions;

            let max_height = dimensions.height - drawable_dimensions.height - PADDING_OFFSET;
            let max_width = dimensions.width - drawable_dimensions.width - PADDING_OFFSET;

            if updated_position.height > max_height {
                updated_position.height = max_height;
            }

            if updated_position.width > max_width {
                updated_position.width = max_width;
            }
        }

        self.set_position(updated_position.into())
    }
}
