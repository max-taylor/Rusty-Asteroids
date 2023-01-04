use crossterm::event::Event;

pub struct GameState {
    running: bool,
    pub keyboard_event: Option<Event>,
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
