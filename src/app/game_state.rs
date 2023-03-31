use crossterm::event::Event;

pub struct GameState {
    running: bool,
    pub keyboard_event: Option<Event>,
    pub score: u64,
    pub game_over: bool,
}

pub const ASTEROID_DESTROYED_POINTS: u64 = 1;

impl GameState {
    pub fn new() -> Self {
        Self {
            running: false,
            game_over: false,
            keyboard_event: None,
            score: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop_game(&mut self) -> &mut Self {
        self.running = false;

        self
    }

    pub fn start_game(&mut self) -> &mut Self {
        self.running = true;

        self
    }

    pub fn handle_game_over(&mut self) -> &mut Self {
        self.game_over = true;
        self.stop_game();

        self
    }
}
