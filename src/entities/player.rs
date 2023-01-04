use crossterm::event::KeyCode;

use crate::{
    api::display::{Map, Point},
    components::Drawable,
};

use super::{consts::SPACE_SHIP, controller::create_event, Controller};

pub struct Player {
    pub drawable: Drawable,
    pub move_speed: u32,
}

impl Player {
    pub fn new(dimensions: &Point) -> Self {
        let location = Point {
            width: 5,
            height: 5,
        };

        let map = Map::from_ascii(SPACE_SHIP);

        Self {
            drawable: Drawable { map, location },
            move_speed: 1,
        }
    }
}

impl Controller for Player {
    fn up(&mut self) -> &mut Self {
        self.drawable.location.height -= self.move_speed;

        self
    }

    fn down(&mut self) -> &mut Self {
        self.drawable.location.height += self.move_speed;

        self
    }

    fn left(&mut self) -> &mut Self {
        self.drawable.location.width -= self.move_speed;

        self
    }

    fn right(&mut self) -> &mut Self {
        self.drawable.location.width += self.move_speed;

        self
    }

    fn additional_event_logic(&mut self, event: &crossterm::event::Event) -> &mut Self {
        if event == &create_event(KeyCode::Enter) {
            // Spawn bullet
        }

        self
    }
}
