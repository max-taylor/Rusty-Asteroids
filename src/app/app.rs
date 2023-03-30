use std::panic;

use crossterm::event::{poll, read, Event, KeyCode};

use crate::{
    api::display::{get_screen_size, DisplayController, Output, Point},
    components::{Drawable, DrawableState, Health},
    entities::{Borders, Controller, Player},
    systems::{get_collision_summary, run_collision_detection, AsteroidController},
};

use super::{
    app_errors::{AppError, AppResult},
    game_state::{GameState, ASTEROID_DESTROYED_POINTS},
};

pub struct App {
    display_controller: DisplayController,
    borders: Borders,
    player: Player,
    asteroid_controller: AsteroidController,
    dimensions: Point<i64>,
}

pub struct InitialGameState {
    pub player_health: u32,
}

const HUD_HEIGHT: i64 = 10;

impl App {
    pub fn new(dimensions: Point<i64>, init_game_state: InitialGameState) -> AppResult<App> {
        let game_screen_size = dimensions.sub_height(HUD_HEIGHT);

        let game_display_controller = DisplayController::new(dimensions, Point::new(0, HUD_HEIGHT));

        if let Some(error) = game_display_controller.as_ref().err() {
            return Err(AppError::DisplayControllerError(error.clone()));
        }

        let game_display_controller = game_display_controller.unwrap();

        Ok(App {
            display_controller: game_display_controller,
            borders: Borders::new(&game_screen_size)?,
            player: Player::new(None, init_game_state.player_health),
            asteroid_controller: AsteroidController::new(100, game_screen_size),
            dimensions,
        })
    }

    /// Reset method to be called at the start of each loop
    fn reset(&mut self) {
        // self.game_state.keyboard_event = None;

        self.display_controller.layout.reset();
    }

    /// Process the keyboard events, also returns true if the user closes the game with escape
    fn handle_keyboard(&mut self, keyboard_input: Option<&Event>) -> AppResult<()> {
        if let Some(event) = keyboard_input {
            self.player.handle_event(&event);
        }

        Ok(())
    }

    pub fn run_next_game_frame(
        &mut self,
        output: &mut Output,
        game_state: &mut GameState,
        game_loop_duration: u128,
    ) -> AppResult<()> {
        self.reset();

        self.handle_keyboard(game_state.keyboard_event.as_ref())?;

        self.asteroid_controller
            .handle_game_loop(game_loop_duration);

        self.update_positions(game_loop_duration);

        self.handle_collisions(game_state)?;

        self.draw_all_entities(game_state, output)?;

        Ok(())
    }

    // fn run_game_loop(&mut self) -> AppResult<()> {
    //     while self.game_state.is_running() {
    //         let game_loop_start = get_now();
    //         self.reset();

    //         // self.handle_keyboard()?;

    //         let game_loop_duration = get_now() - game_loop_start;

    //         self.asteroid_controller
    //             .handle_game_loop(game_loop_duration);

    //         self.update_positions(game_loop_duration);

    //         self.handle_collisions()?;

    //         self.draw_all_entities()?;
    //     }

    //     Ok(())
    // }

    fn handle_collisions(&mut self, game_state: &mut GameState) -> AppResult<&mut Self> {
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
                    game_state.score += collision.points;
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
                    game_state.handle_game_over();
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
    fn draw_all_entities(
        &mut self,
        game_state: &mut GameState,
        output: &mut Output,
    ) -> AppResult<&mut Self> {
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
            .draw_game_state(game_state, self.player.get_health())?;

        output.print_display(&self.display_controller.layout)?;

        Ok(self)
    }
}
