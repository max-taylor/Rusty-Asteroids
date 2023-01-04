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

pub struct Asteroid {
    pub drawable: DrawableState,
}

impl Asteroid {
    pub fn new(dimensions: &Point<i64>) -> Self {
        let mut rng = rand::thread_rng();

        let location = Point {
            height: 0,
            width: rng.gen_range(0..dimensions.width),
        };

        let map = Layout::from_ascii(ASTEROID);

        let velocity: Point<i64> = Point {
            height: 3,
            width: 0,
        };

        Self {
            drawable: DrawableState::new(map, location, DrawableType::Enemy, Some(velocity)),
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
