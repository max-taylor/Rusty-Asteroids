use crossterm::event::Event;

use crate::components::Drawable;

pub struct GameState {
    running: bool,
    pub keyboard_event: Option<Event>,
    // pub drawables_in_frame: Vec<dyn Drawable>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            running: false,
            keyboard_event: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start_game(&mut self) -> &mut Self {
        self.running = true;

        self
    }
}
