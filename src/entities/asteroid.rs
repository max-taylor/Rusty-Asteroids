use crossterm::style::Color;

use crate::{
    api::display::{Layout, Point},
    components::{get_updated_health, Drawable, DrawableState, DrawableType, Health},
};

use super::consts::ASTEROID;

pub const ASTEROID_DAMAGE: u32 = 1;

pub struct Asteroid {
    pub drawable: DrawableState,
    pub health: u32,
}

impl Asteroid {
    pub fn new(location: Point<i64>, velocity: Point<i64>) -> Self {
        let map = Layout::from_ascii(ASTEROID, Color::Yellow);

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

impl Health for Asteroid {
    fn apply_damage(&mut self, damage: u32) -> &mut Self {
        self.health = get_updated_health(self.health, damage);

        self
    }

    fn get_health(&self) -> u32 {
        self.health
    }
}
