use std::io::stdout;

use crossterm::event::{Event, KeyCode};

use crate::{
    api::display::{DisplayController, Output, Point},
    helpers::{get_keyboard_event, get_now},
};

use super::{app::InitialGameState, app_errors::AppResult, App, GameState};

pub struct AppManager {
    dimensions: Point<i64>,
    output: Output,
    app: Option<App>,
    display_controller: DisplayController,
    game_state: GameState,
}

struct Difficulty<'name> {
    name: &'name str,
    level: u32,
}

const DIFFICULTIES: &'static [Difficulty] = &[
    Difficulty {
        name: "Easy",
        level: 1,
    },
    Difficulty {
        name: "Medium",
        level: 2,
    },
    Difficulty {
        name: "Hard",
        level: 3,
    },
];

const GAME_LOOP_DELAY: u64 = 75;

const init_game_state: InitialGameState = InitialGameState { player_health: 5 };

const PLAYER_INITIAL_HEALTH: u32 = 5;

impl AppManager {
    pub fn new(dimensions: Point<i64>) -> AppResult<AppManager> {
        let mut output = Output::new(stdout());

        Ok(AppManager {
            dimensions,
            output,
            app: None,
            display_controller: DisplayController::new(dimensions, Default::default())?,
            game_state: GameState::new(),
        })
    }

    pub fn run(&mut self) -> AppResult<()> {
        self.start_and_run_game()?;

        self.shut_down()?;

        Ok(())
    }

    fn handle_keyboard(&mut self) -> AppResult<()> {
        let event = get_keyboard_event(GAME_LOOP_DELAY)?;

        if let Some(event) = event {
            if event == Event::Key(KeyCode::Esc.into()) {
                self.game_state.stop_game();

                return Ok(());
            }

            self.game_state.keyboard_event = Some(event);
        } else {
            self.game_state.keyboard_event = None;
        }

        Ok(())
    }

    fn start_and_run_game(&mut self) -> AppResult<()> {
        self.game_state.start_game();
        self.output.start()?;

        let mut app = App::new(self.dimensions, init_game_state).unwrap();

        while self.game_state.is_running() {
            let game_loop_start = get_now();

            self.handle_keyboard()?;

            let game_loop_duration = get_now() - game_loop_start;

            app.run_next_game_frame(&mut self.output, &mut self.game_state, game_loop_duration)?;
        }

        Ok(())
    }

    pub fn shut_down(&mut self) -> AppResult<()> {
        self.output.close()?;

        Ok(())
    }
}
