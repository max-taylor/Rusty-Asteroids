use crossterm::event::KeyCode;

use crate::{
    api::display::{Layout, Point},
    components::{Drawable, DrawableState, DrawableType},
};

use super::{consts::SPACE_SHIP, controller::create_event, Controller};

pub struct Player {
    pub drawable: DrawableState,
}

const MAX_VELOCITY: i64 = 1;

impl Player {
    pub fn new() -> Self {
        let location = Point {
            width: 5,
            height: 5,
        };

        let map = Layout::from_ascii(SPACE_SHIP);

        Self {
            drawable: DrawableState::new(map, location, DrawableType::Player),
        }
    }
}

impl Drawable for Player {
    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }
}

impl Controller for Player {
    fn up(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(0, -MAX_VELOCITY);

        self
    }

    fn down(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(0, MAX_VELOCITY);

        self
    }

    fn left(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(-MAX_VELOCITY, 0);

        self
    }

    fn right(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(MAX_VELOCITY, 0);

        self
    }

    fn additional_event_logic(&mut self, event: &crossterm::event::Event) -> &mut Self {
        if event == &create_event(KeyCode::Enter) {
            // Spawn bullet
        }

        self
    }
}
