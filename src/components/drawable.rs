use std::ops::Add;

use crate::api::display::{Layout, Point};

enum Direction {
    Positive,
    Negative,
}

// /// Using a stored direction and a velocity to simply
// struct AbsoluteVelocity {
//   direction: Direction,
//   velocity: u32
// }

#[derive(Debug)]
pub struct Velocity {
    width: i64,
    height: i64,
}

impl Velocity {
    pub fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct DrawableState {
    pub layout: Layout,
    pub location: Point,
    /// velocity in x-y (width, height)
    pub velocity: Velocity,
}

enum Type {
    Player,
    Enemy,
    // Damage of ammunition
    Ammunition(u32),
}

impl DrawableState {
    pub fn new(layout: Layout, location: Point) -> Self {
        Self {
            layout,
            location,
            velocity: Default::default(),
        }
    }
}

pub trait Drawable {
    fn get_drawable_state(&self) -> &DrawableState;

    fn update_position(&mut self) -> &mut Self {
        let drawable_state = self.get_drawable_state();

        let updated_position =
            drawable_state.location.height as i64 + drawable_state.velocity.height;

        self
    }
}
