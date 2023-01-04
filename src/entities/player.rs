use crossterm::event::KeyCode;

use crate::{
    api::display::{Map, Point},
    components::Drawable,
};

use super::{consts::SPACE_SHIP, controller::create_event, Controller};

pub struct Player {
    pub drawable: Drawable,
}

impl Player {
    pub fn new() -> Self {
        let location = Point {
            width: 5,
            height: 5,
        };

        let map = Map::from_ascii(SPACE_SHIP);

        Self {
            drawable: Drawable {
                map,
                location,
                velocity: 1,
            },
        }
    }
}

impl Controller for Player {
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
