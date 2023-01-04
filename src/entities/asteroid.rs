use crate::{
    api::display::{
        element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Layout, Point,
    },
    components::{Drawable, DrawableState, DrawableType},
};

const ARROW_ELEMENT: Element = Element::new('^', DEFAULT_BACKGROUND, DEFAULT_FOREGROUND);

pub struct Asteroid {
    pub drawable: DrawableState,
}

impl Asteroid {
    pub fn new() -> Self {
        let map = Layout::new(
            &Point {
                width: 1,
                height: 1,
            },
            Some(ARROW_ELEMENT),
        );

        let velocity: Point<i64> = Point {
            height: 5,
            width: 0,
        };

        Self {
            drawable: DrawableState::new(
                map,
                Default::default(),
                DrawableType::Enemy,
                Some(velocity),
            ),
        }
    }
}

impl Drawable for Asteroid {
    fn set_position(&mut self, updated_position: Point<u32>) -> &mut Self {
        self.drawable.location = updated_position;

        self
    }

    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }
}
