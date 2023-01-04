use crossterm::event::Event;

pub struct GameState {
    running: bool,
    pub keyboard_event: Option<Event>,
    pub score: u64,
    // pub drawables_in_frame: Vec<dyn Drawable>,
}

pub const ASTEROID_DESTROYED_POINTS: u64 = 1;

impl GameState {
    pub fn new() -> Self {
        Self {
            running: false,
            keyboard_event: None,
            score: 0,
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
