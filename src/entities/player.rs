use crossterm::event::KeyCode;

use crate::{
    api::display::{Map, Point},
    components::{Drawable, DrawableState},
};

use super::{consts::SPACE_SHIP, controller::create_event, Controller};

pub struct Player {
    pub drawable: DrawableState,
}

impl Player {
    pub fn new() -> Self {
        let location = Point {
            width: 5,
            height: 5,
        };

        let map = Map::from_ascii(SPACE_SHIP);

        Self {
            drawable: DrawableState {
                map,
                location,
                velocity: 1,
            },
        }
    }
}

impl Drawable for Player {
    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }
}

impl Controller for Player {
    // TODO: Collision detection for the boundaries here
    fn up(&mut self) -> &mut Self {
        self.drawable.location.height -= self.drawable.velocity;

        self
    }

    fn down(&mut self) -> &mut Self {
        self.drawable.location.height += self.drawable.velocity;

        self
    }

    fn left(&mut self) -> &mut Self {
        self.drawable.location.width -= self.drawable.velocity;

        self
    }

    fn right(&mut self) -> &mut Self {
        self.drawable.location.width += self.drawable.velocity;

        self
    }

    fn additional_event_logic(&mut self, event: &crossterm::event::Event) -> &mut Self {
        if event == &create_event(KeyCode::Enter) {
            // Spawn bullet
        }

        self
    }
}
