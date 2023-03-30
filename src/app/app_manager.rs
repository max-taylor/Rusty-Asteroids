use std::io::stdout;

use crossterm::event::{Event, KeyCode};

use crate::{
    api::display::{
        DisplayController, DisplayControllerError, Output, Point, GAME_DETAILS_BOX_WIDTH,
        MINIMUM_SCREEN_WIDTH,
    },
    helpers::{get_keyboard_event, get_now},
    user_display::DifficultyDisplay,
};

use super::{
    app::InitialGameState,
    app_errors::{AppError, AppResult},
    App, GameState,
};

pub struct AppManager {
    dimensions: Point<i64>,
    output: Output,
    app: Option<App>,
    game_state: GameState,
}

struct DifficultyOption<'name> {
    name: &'name str,
    level: u32,
}

const DIFFICULTIES: &'static [DifficultyOption] = &[
    DifficultyOption {
        name: "Easy",
        level: 1,
    },
    DifficultyOption {
        name: "Medium",
        level: 2,
    },
    DifficultyOption {
        name: "Hard",
        level: 3,
    },
];

const GAME_LOOP_DELAY: u64 = 75;

const INIT_GAME_STATE: InitialGameState = InitialGameState { player_health: 50 };

impl AppManager {
    pub fn new(dimensions: Point<i64>) -> AppResult<AppManager> {
        if dimensions.width < MINIMUM_SCREEN_WIDTH as i64 {
            return Err(AppError::ScreenWidthTooSmall(
                dimensions.width as u64,
                MINIMUM_SCREEN_WIDTH,
            ));
        }

        let output = Output::new(stdout());

        Ok(AppManager {
            dimensions,
            output,
            app: None,
            game_state: GameState::new(),
        })
    }

    pub fn run(&mut self) -> AppResult<()> {
        self.output.start()?;

        self.start_and_run_game()?;
        // self.run_difficulty_selection()?;

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

        let mut app = App::new(self.dimensions, INIT_GAME_STATE).unwrap();

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

    fn run_difficulty_selection(&mut self) -> AppResult<DifficultyOption> {
        let difficulty_selected: Option<DifficultyDisplay> = None;
        let mut display_controller = DisplayController::new(self.dimensions, Default::default())?;

        let start_position: Point<i64> = Default::default();
        let difficulty = DifficultyDisplay::new("Low", 1, Default::default()).unwrap();

        // let difficulty_displays: Vec<DifficultyDisplay> = DIFFICULTIES
        //     .into_iter()
        //     .map(|option| {
        //         DifficultyDisplay::new(option.name, option.level, Default::default()).unwrap()
        //     })
        //     .collect();

        let mut difficulty_selection_running = true;

        while difficulty_selection_running {
            display_controller.layout.reset();

            let event = get_keyboard_event(GAME_LOOP_DELAY)?;

            if let Some(event) = event {
                if event == Event::Key(KeyCode::Esc.into()) {
                    difficulty_selection_running = false;
                }
            }

            let updated_position = start_position + (10 as i64).into();

            display_controller.layout.draw_map(
                &difficulty.layout.map,
                start_position,
                &Default::default(),
            )?;
            // for (idx, difficulty) in difficulty_displays.iter().enumerate() {
            //     let updated_position = start_position + (10 as i64).into();

            //     display_controller.layout.draw_map(
            //         &difficulty.layout.map,
            //         start_position,
            //         &updated_position,
            //     )?;
            // }

            self.output.print_display(&display_controller.layout)?;

            // let game_loop_start = get_now();

            // self.handle_keyboard()?;

            // let game_loop_duration = get_now() - game_loop_start;

            // app.run_next_game_frame(&mut self.output, &mut self.game_state, game_loop_duration)?;
        }

        Ok(DifficultyOption {
            name: "Easy",
            level: 1,
        })
    }
}
