use crate::{
    api::display::{
        element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Layout, Point,
    },
    components::{get_updated_health, Drawable, DrawableState, DrawableType, Health},
};

pub struct Bullet {
    pub drawable: DrawableState,
    pub health: u32,
}

pub const BULLET_DAMAGE: u32 = 1;

const ARROW_ELEMENT: Element = Element::new('^', DEFAULT_BACKGROUND, DEFAULT_FOREGROUND);

impl Bullet {
    pub fn new(location: Point<i64>) -> Self {
        let asteroid: Vec<Vec<Option<Element>>> = [
            [None, Some(ARROW_ELEMENT), None],
            [None, Some(ARROW_ELEMENT), None],
            [
                Some(ARROW_ELEMENT),
                Some(ARROW_ELEMENT),
                Some(ARROW_ELEMENT),
            ],
        ]
        .map(|row| row.to_vec())
        .to_vec();

        let map = Layout::from_map(asteroid, Some(ARROW_ELEMENT));

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
