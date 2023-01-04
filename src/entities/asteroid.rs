use rand::Rng;

use crate::{
    api::display::{
        element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Layout, Point,
    },
    components::{Drawable, DrawableState, DrawableType},
};

use super::consts::ASTEROID;

const ARROW_ELEMENT: Element = Element::new('^', DEFAULT_BACKGROUND, DEFAULT_FOREGROUND);

pub const ASTEROID_DAMAGE: u32 = 1;

pub struct Asteroid {
    pub drawable: DrawableState,
    pub health: u32,
}

impl Asteroid {
    pub fn new(location: Point<i64>, velocity: Point<i64>) -> Self {
        let map = Layout::from_ascii(ASTEROID);

        Self {
            drawable: DrawableState::new(
                map,
                location,
                DrawableType::Enemy(ASTEROID_DAMAGE),
                Some(velocity),
            ),
            health: 1,
        }
    }
}

impl Drawable for Asteroid {
    fn set_position(&mut self, updated_position: Point<i64>) -> &mut Self {
        self.drawable.location = updated_position;

        self
    }

    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }
}
