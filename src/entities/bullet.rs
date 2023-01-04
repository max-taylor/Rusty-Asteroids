use crossterm::style::Color;

use crate::{
    api::display::{
        element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Layout, Point,
    },
    components::{get_updated_health, Drawable, DrawableState, DrawableType, Health},
};

use super::consts::{BASIC_BULLET, SPREAD_BULLET};

pub struct Bullet {
    pub drawable: DrawableState,
    pub health: u32,
}

pub const BULLET_DAMAGE: u32 = 1;

const ARROW_ELEMENT: Element = Element::new('^', DEFAULT_BACKGROUND, DEFAULT_FOREGROUND);

impl Bullet {
    pub fn build_basic_bullet(location: Point<i64>) -> Self {
        let map = Layout::from_ascii(BASIC_BULLET, Color::White);

        let velocity: Point<i64> = Point {
            height: -20,
            width: 0,
        };

        Self {
            drawable: DrawableState::new(
                map,
                location,
                DrawableType::Ammunition(BULLET_DAMAGE),
                Some(velocity),
            ),
            health: 1,
        }
    }

    pub fn build_spread_bullet(location: Point<i64>) -> Self {
        let map = Layout::from_ascii(SPREAD_BULLET, Color::White);

        let velocity: Point<i64> = Point {
            height: -20,
            width: 0,
        };

        Self {
            drawable: DrawableState::new(
                map,
                location,
                DrawableType::Ammunition(BULLET_DAMAGE),
                Some(velocity),
            ),
            health: 5,
        }
    }
}

impl Drawable for Bullet {
    fn set_position(&mut self, updated_position: Point<i64>) -> &mut Self {
        self.drawable.location = updated_position;

        self
    }

    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }
}

impl Health for Bullet {
    fn apply_damage(&mut self, damage: u32) -> &mut Self {
        self.health = get_updated_health(self.health, damage);

        self
    }

    fn get_health(&self) -> u32 {
        self.health
    }
}
