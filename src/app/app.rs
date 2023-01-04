use std::{io::stdout, panic, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use crate::{
    api::display::{DisplayController, DisplayControllerError, Output, Point},
    components::Drawable,
    entities::{Borders, Controller, Player},
    helpers::get_now,
    systems::AsteroidController,
};

use super::game_state::GameState;

pub struct App {
    display_controller: DisplayController,
    output: Output,
    game_state: GameState,
    borders: Borders,
    player: Player,
    asteroid_controller: AsteroidController,
    dimensions: Point<i64>,
}

const SPAWN_GAME_LOOPS: i64 = 5;

impl App {
    pub fn new(dimensions: Option<&Point<i64>>) -> Result<App, DisplayControllerError> {
        let mut output = Output::new(stdout());

        let display_controller = DisplayController::new(dimensions);

        if let Some(error) = display_controller.as_ref().err() {
            output.close()?;

            return Err(error.clone());
        }
        let display_controller = display_controller.unwrap();

        let dimensions = display_controller.layout.dimensions;

        Ok(App {
            display_controller: display_controller,
            game_state: GameState::new(),
            borders: Borders::new(&dimensions)?,
            output,
            player: Player::new(),
            asteroid_controller: AsteroidController::new(50, dimensions),
            dimensions,
        })
    }

    /// Reset method to be called at the start of each loop
    fn reset(&mut self) -> Result<(), DisplayControllerError> {
        self.game_state.keyboard_event = None;

        self.display_controller.layout.reset();

        Ok(())
    }

    // TODO trying to return an array of Vec<impl Drawables from the function that is executed

    pub fn run(&mut self) -> Result<(), DisplayControllerError> {
        self.start()?;

        let result = panic::catch_unwind(panic::AssertUnwindSafe(
            || -> Result<(), DisplayControllerError> {
                while self.game_state.is_running() {
                    let game_loop_start = get_now();
                    self.reset()?;

                    if poll(Duration::from_millis(100))? {
                        let event = read()?;

                        if event == Event::Key(KeyCode::Esc.into()) {
                            self.output.close()?;

                            break;
                        }

                        self.game_state.keyboard_event = Some(event);
                    }

                    if let Some(event) = &self.game_state.keyboard_event {
                        self.player.handle_event(&event);
                    }

                    let game_loop_duration = get_now() - game_loop_start;

                    self.asteroid_controller
                        .handle_game_loop(game_loop_duration);

                    self.update_positions().draw_all_entities()?;

                    self.output.print_display(&self.display_controller.layout)?;
                }
                Ok(())
            },
        ));

        self.shut_down()?;

        result.unwrap()
    }

    fn update_positions(&mut self) -> &mut Self {
        self.player.update_position(Some(&self.dimensions));

        self.player.bullets.entities.iter_mut().for_each(|bullet| {
            bullet.update_position(None);
        });

        self.asteroid_controller
            .asteroids
            .iter_mut()
            .for_each(|asteroid| {
                asteroid.update_position(None);
            });

        self
    }

    /// Method to handle drawing all the entities that will be rendered
    fn draw_all_entities(&mut self) -> Result<&mut Self, DisplayControllerError> {
        // Ignore the return type for the borders and player, because we don't delete these at any point
        self.display_controller.draw_drawable(&self.borders)?;
        self.display_controller.draw_drawable(&self.player)?;

        self.display_controller
            .draw_vec(&mut self.player.bullets.entities);

        self.display_controller
            .draw_vec(&mut self.asteroid_controller.asteroids);

        Ok(self)
    }

    pub fn start(&mut self) -> Result<(), DisplayControllerError> {
        self.game_state.start_game();
        self.output.start()?;

        Ok(())
    }

    pub fn shut_down(&mut self) -> Result<(), DisplayControllerError> {
        self.output.close()?;

        Ok(())
    }
}
