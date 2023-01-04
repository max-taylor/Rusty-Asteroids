use std::{io::stdout, panic, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use crate::{
    api::display::{DisplayController, DisplayControllerError, Output, Point},
    components::{update_position_for_drawable_vec, Drawable, DrawableState},
    entities::{Borders, Controller, Player},
    helpers::get_now,
    systems::{get_collision_summary, run_collision_detection, AsteroidController},
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
            player: Player::new(None),
            asteroid_controller: AsteroidController::new(100, dimensions),
            dimensions,
        })
    }

    /// Reset method to be called at the start of each loop
    fn reset(&mut self) -> Result<(), DisplayControllerError> {
        self.game_state.keyboard_event = None;

        self.display_controller.layout.reset();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), DisplayControllerError> {
        self.start()?;

        // Simple "try-catch" wrapper to catch panic's so we can safely shutdown the display
        let result = panic::catch_unwind(panic::AssertUnwindSafe(
            || -> Result<(), DisplayControllerError> {
                while self.game_state.is_running() {
                    let game_loop_start = get_now();
                    self.reset()?;

                    // Handle keyboard presses
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

                    self.update_positions(game_loop_duration);

                    self.handle_collisions();

                    self.draw_all_entities()?;
                }
                Ok(())
            },
        ));

        self.shut_down()?;

        result.unwrap()
    }

    fn handle_collisions(&mut self) -> &mut Self {
        let collision_results = get_collision_summary(run_collision_detection(
            self.get_all_drawable_states(),
            &self.dimensions,
        ));

        for (uuid, collision) in collision_results {
            self.asteroid_controller
                .apply_asteroid_damage(uuid, collision.damage);
        }

        self
    }

    fn update_positions(&mut self, game_loop_duration: u128) -> &mut Self {
        self.player
            .update_position(Some(&self.dimensions), game_loop_duration);

        update_position_for_drawable_vec(
            &mut self.player.bullet_entity_controller.entities,
            game_loop_duration,
        );

        update_position_for_drawable_vec(
            &mut self.asteroid_controller.asteroids,
            game_loop_duration,
        );

        self
    }

    fn get_all_drawable_states(&self) -> Vec<&DrawableState> {
        let mut drawable_items: Vec<&DrawableState> = vec![self.player.get_drawable_state()];

        drawable_items.append(&mut self.asteroid_controller.get_all_drawable_states());
        drawable_items.append(&mut self.player.get_bullet_drawable_states());

        drawable_items
    }

    /// Method to handle drawing all the entities that will be rendered
    fn draw_all_entities(&mut self) -> Result<&mut Self, DisplayControllerError> {
        self.display_controller
            .draw_drawable(&self.borders.get_drawable_state())?;

        self.display_controller
            .draw_drawable(&self.player.get_drawable_state())?;

        self.display_controller
            .draw_entity_controller_items(&mut self.player.bullet_entity_controller);

        // self.display_controller
        //     .draw_entity_controller_items(&mut self.player.bullet_entity_controller);

        self.display_controller
            .draw_vec(&mut self.asteroid_controller.asteroids);

        self.output.print_display(&self.display_controller.layout)?;

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
