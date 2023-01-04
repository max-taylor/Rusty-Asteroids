use crate::api::display::{Layout, Point};
use uuid::Uuid;

#[derive(Debug)]
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
    Enemy,
    Border,
    // Damage of ammunition
    Ammunition(i64),
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

pub trait Drawable {
    fn set_position(&mut self, updated_position: Point<i64>) -> &mut Self;

    fn get_drawable_state(&self) -> &DrawableState;

    fn update_position(&mut self, dimensions: Option<&Point<i64>>) -> &mut Self {
        let drawable_state = self.get_drawable_state();

        // Convert location to i64 so we can handle an underflow of the position; if the position goes below (0,0)
        let converted_location: Point<i64> = drawable_state.location.into();
        let mut updated_position: Point<i64> = converted_location + drawable_state.velocity;

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
