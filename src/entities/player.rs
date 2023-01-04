use crossterm::event::{Event, KeyCode};

use super::controller::Controller;

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
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
