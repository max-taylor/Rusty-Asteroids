use crate::{
    api::display::{
        element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Layout, Point,
    },
    components::{Drawable, DrawableState, DrawableType},
};

pub struct Bullet {
    pub drawable: DrawableState,
}

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
            height: -2,
            width: 0,
        };

        Self {
            drawable: DrawableState::new(map, location, DrawableType::Enemy, Some(velocity)),
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
