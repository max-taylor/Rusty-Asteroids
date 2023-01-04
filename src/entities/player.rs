use crossterm::event::{Event, KeyCode};

use crate::components::Position;

use super::controller::Controller;

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }
}

impl Position for Player {
    fn new(position: crate::systems::display::Point, value: char) -> Self {
        todo!()
    }

    fn update_position(&mut self, new_position: crate::systems::display::Point) -> &mut Self {
        todo!()
    }
}

impl Controller for Player {
    fn up(&mut self) -> &mut Self {
        todo!()
    }

    fn down(&mut self) -> &mut Self {
        todo!()
    }

    fn left(&mut self) -> &mut Self {
        todo!()
    }

    fn right(&mut self) -> &mut Self {
        todo!()
    }

    fn handle_event(&mut self, event: crossterm::event::Event) -> &mut Self {
        if event == Event::Key(KeyCode::Up.into()) {
            return self.up();
        }

        self
    }
}
