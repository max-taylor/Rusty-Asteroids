use std::io::stdout;

use crossterm::{
    event::{Event, KeyCode},
    style::Color,
};

use crate::{
    api::display::{
        element::DEFAULT_BACKGROUND, map_from_str, DisplayController, Output, Point,
        MINIMUM_SCREEN_WIDTH,
    },
    components::Drawable,
    entities::Borders,
    helpers::{get_keyboard_event, get_now},
    user_display::{DifficultyDisplay, GAME_OVER_TEXT},
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

const INIT_GAME_STATE: InitialGameState = InitialGameState { player_health: 3 };

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

        let mut is_running = true;

        while is_running {
            self.start_and_run_game()?;

            if self.game_state.game_over {
                let new_game = self.handle_game_over()?;
                if !new_game {
                    is_running = false;
                } else {
                    self.game_state.game_over = false;
                    self.game_state.score = 0;
                }
            } else {
                // If the user exited or an error was encountered
                is_running = false;
            }
        }

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

    fn handle_game_over(&mut self) -> AppResult<bool> {
        let mut display_controller = DisplayController::new(self.dimensions, Default::default())?;

        let mut while_running = true;
        let mut new_game = false;

        let border = Borders::new(&self.dimensions, Color::Cyan)?;

        let draw_start_height = self.dimensions.height / 2 - 10;

        while while_running {
            display_controller.layout.reset();

            let event = get_keyboard_event(GAME_LOOP_DELAY)?;

            if let Some(event) = event {
                if event == Event::Key(KeyCode::Esc.into()) {
                    while_running = false;
                }
                if event == Event::Key(KeyCode::Enter.into()) {
                    new_game = true;
                    while_running = false;
                }
            }

            display_controller.draw_drawable(&border.get_drawable_state())?;

            display_controller.layout.draw_map(
                &map_from_str(GAME_OVER_TEXT, Color::Green),
                Point {
                    height: draw_start_height,
                    width: self.dimensions.width / 2 - 47,
                },
                &Default::default(),
            )?;

            display_controller.draw_str(
                "Score:",
                DEFAULT_BACKGROUND,
                Color::Red,
                Point {
                    height: draw_start_height + 10,
                    width: self.dimensions.width / 2 - 5,
                },
            )?;

            let score_items = self.game_state.score.to_string().len();

            display_controller.draw_u32(
                self.game_state.score as u32,
                Point {
                    height: draw_start_height + 12,
                    width: self.dimensions.width / 2 - (2 + score_items as i64 * 3),
                },
                Color::Cyan,
            )?;

            display_controller.draw_str(
                "Press ENTER for new game",
                DEFAULT_BACKGROUND,
                Color::Green,
                Point {
                    height: self.dimensions.height - 4,
                    width: self.dimensions.width / 2 - 13,
                },
            )?;

            display_controller.draw_str(
                "or ESC to close",
                DEFAULT_BACKGROUND,
                Color::Red,
                Point {
                    height: self.dimensions.height - 3,
                    width: self.dimensions.width / 2 - 9,
                },
            )?;

            self.output.print_display(&display_controller.layout)?;
        }

        Ok(new_game)
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
