use std::{io::stdout, panic, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use crate::{
    api::display::{get_screen_size, DisplayController, Output, Point},
    components::{Drawable, DrawableState, Health},
    entities::{Borders, Controller, Player},
    helpers::{get_keyboard_event, get_now},
    systems::{get_collision_summary, run_collision_detection, AsteroidController},
    user_display::GAME_OVER_TEXT,
};

use super::{
    app_errors::AppError,
    game_state::{GameState, ASTEROID_DESTROYED_POINTS},
};

pub struct App {
    display_controller: DisplayController,
    output: Output,
    game_state: GameState,
    borders: Borders,
    player: Player,
    asteroid_controller: AsteroidController,
    dimensions: Point<i64>,
}

const HUD_HEIGHT: i64 = 10;

pub type AppResult<T> = Result<T, AppError>;

impl App {
    pub fn new() -> AppResult<App> {
        let mut output = Output::new(stdout());

        let screen_size = get_screen_size();

        let game_screen_size = screen_size.sub_height(HUD_HEIGHT);

        let game_display_controller =
            DisplayController::new(screen_size, Point::new(0, HUD_HEIGHT));

        if let Some(error) = game_display_controller.as_ref().err() {
            output.close()?;

            return Err(AppError::DisplayControllerError(error.clone()));
        }

        let game_display_controller = game_display_controller.unwrap();

        Ok(App {
            display_controller: game_display_controller,
            game_state: GameState::new(),
            borders: Borders::new(&game_screen_size)?,
            output,
            player: Player::new(None),
            asteroid_controller: AsteroidController::new(100, game_screen_size),
            dimensions: screen_size,
        })
    }

    /// Reset method to be called at the start of each loop
    fn reset(&mut self) {
        self.game_state.keyboard_event = None;

        self.display_controller.layout.reset();
    }

    /// Process the keyboard events, also returns true if the user closes the game with escape
    fn handle_keyboard(&mut self) -> AppResult<()> {
        let event = get_keyboard_event()?;

        if let Some(event) = event {
            if event == Event::Key(KeyCode::Esc.into()) {
                self.game_state.stop_game();

                return Ok(());
            }

            self.player.handle_event(&event);

            self.game_state.keyboard_event = Some(event);
        }

        Ok(())
    }

    fn run_game_loop(&mut self) -> AppResult<()> {
        while self.game_state.is_running() {
            let game_loop_start = get_now();
            self.reset();

            self.handle_keyboard()?;

            let game_loop_duration = get_now() - game_loop_start;

            self.asteroid_controller
                .handle_game_loop(game_loop_duration);

            self.update_positions(game_loop_duration);

            self.handle_collisions()?;

            self.draw_all_entities()?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> AppResult<()> {
        self.start()?;

        let result = self.run_game_loop();

        if let Err(err) = result.as_ref() {
            match err {
                AppError::OutOfLives => {
                    self.reset();
                    dbg!("HERE22");
                    let result = self.display_controller.layout.draw_str(
                        GAME_OVER_TEXT,
                        &Point::new(0, 0),
                        None,
                        None,
                    );

                    dbg!(result.err());

                    // while true {}
                }
                _ => {
                    dbg!("IN THIS ONE");
                }
            }
        }

        self.shut_down()?;

        result
    }

    fn handle_collisions(&mut self) -> AppResult<&mut Self> {
        let collision_results = get_collision_summary(run_collision_detection(
            self.get_all_drawable_states(),
            &self.dimensions,
        ));

        for (uuid, collision) in collision_results {
            // Asteroid collision
            if self
                .asteroid_controller
                .entity_controller
                .has_entity(collision.uuid)
            {
                let destroyed = self
                    .asteroid_controller
                    .entity_controller
                    .apply_entity_damage(uuid, collision.damage);

                if destroyed {
                    self.game_state.score += ASTEROID_DESTROYED_POINTS;
                }
            } else if self.player.bullet_entity_controller.has_entity(uuid) {
                // Bullet collision
                self.player
                    .bullet_entity_controller
                    .apply_entity_damage(uuid, collision.damage);
            } else if self.player.drawable.uuid == uuid {
                // Player collision
                self.player.apply_damage(collision.damage);

                if self.player.get_health() == 0 {
                    return Err(AppError::OutOfLives);
                }
            } else {
                // Use a lazy panic here because this shouldn't happen
                panic!("UUID missing from all arrays");
            }
        }

        Ok(self)
    }

    fn update_positions(&mut self, game_loop_duration: u128) -> &mut Self {
        self.player
            .update_position(Some(&self.dimensions), game_loop_duration);

        self.player
            .bullet_entity_controller
            .update_entity_positions(game_loop_duration);

        self.asteroid_controller
            .entity_controller
            .update_entity_positions(game_loop_duration);

        self
    }

    fn get_all_drawable_states(&self) -> Vec<&DrawableState> {
        let mut drawable_items: Vec<&DrawableState> = vec![self.player.get_drawable_state()];

        drawable_items.append(
            &mut self
                .asteroid_controller
                .entity_controller
                .get_all_drawable_states(),
        );
        drawable_items.append(
            &mut self
                .player
                .bullet_entity_controller
                .get_all_drawable_states(),
        );

        drawable_items
    }

    /// Method to handle drawing all the entities that will be rendered
    fn draw_all_entities(&mut self) -> AppResult<&mut Self> {
        self.display_controller
            .draw_drawable(&self.player.get_drawable_state())?;

        // Draw all the entities in the bullet and asteroid controller
        self.display_controller
            .draw_entity_controller_items(&mut self.player.bullet_entity_controller);

        self.display_controller
            .draw_entity_controller_items(&mut self.asteroid_controller.entity_controller);

        self.display_controller
            .draw_drawable(&self.borders.get_drawable_state())?;

        self.display_controller
            .draw_game_state(&self.game_state, self.player.get_health())?;

        self.output.print_display(&self.display_controller.layout)?;

        Ok(self)
    }

    pub fn start(&mut self) -> AppResult<()> {
        self.game_state.start_game();
        self.output.start()?;

        Ok(())
    }

    pub fn shut_down(&mut self) -> AppResult<()> {
        self.output.close()?;

        Ok(())
    }
}
