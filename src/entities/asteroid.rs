use crate::{
    api::display::{
        element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Layout, Point,
    },
    components::{Drawable, DrawableState, DrawableType},
};

// let spaceship: Vec<Vec<Option<Element>>> = [
//     [None, Some(ARROW_ELEMENT), None],
//     [
//         Some(ARROW_ELEMENT),
//         Some(ARROW_ELEMENT),
//         Some(ARROW_ELEMENT),
//     ],
// ]
// .map(|row| row.to_vec())
// .to_vec();

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

        Self {
            drawable: DrawableState::new(map, Default::default(), DrawableType::Enemy),
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
