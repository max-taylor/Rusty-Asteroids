use crate::api::display::{Layout, Point};

#[derive(Debug)]
pub struct DrawableState {
    pub layout: Layout,
    pub location: Point<u32>,
    pub velocity: Point<i64>,
    pub drawable_type: DrawableType,
}

#[derive(Debug)]
pub enum DrawableType {
    Player,
    Enemy,
    Border,
    // Damage of ammunition
    Ammunition(u32),
}

impl DrawableState {
    pub fn new(layout: Layout, location: Point<u32>, drawable_type: DrawableType) -> Self {
        Self {
            layout,
            location,
            drawable_type,
            velocity: Default::default(),
        }
    }
}

pub trait Drawable {
    fn set_position(&mut self, updated_position: Point<u32>) -> &mut Self;

    fn get_drawable_state(&self) -> &DrawableState;

    fn update_position(&mut self) -> &mut Self {
        let drawable_state = self.get_drawable_state();

        // Convert location to i64 so we can handle an underflow of the position; if the position goes below (0,0)
        let converted_location: Point<i64> = drawable_state.location.into();
        let mut updated_position: Point<i64> = converted_location + drawable_state.velocity;

        // if ()

        if updated_position.height < 0 {
            updated_position.height = 0;
        }

        if updated_position.width < 0 {
            updated_position.width = 0;
        }

        // self.

        self.set_position(updated_position.into())
    }
}
