use std::{io::stdout, panic, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use crate::{
    api::display::{DisplayController, DisplayControllerError, Output, Point},
    components::Drawable,
    entities::{Asteroid, Borders, Bullet, Controller, Player},
};

use super::game_state::GameState;

pub struct App {
    display_controller: DisplayController,
    output: Output,
    game_state: GameState,
    borders: Borders,
    player: Player,
    asteroids: Vec<Asteroid>,
    dimensions: Point<i64>,
}

const SPAWN_GAME_LOOPS: i64 = 5;

impl App {
    pub fn new(dimensions: Option<&Point<i64>>) -> Result<App, DisplayControllerError> {
        dbg!(dimensions);
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
            asteroids: Default::default(),
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

        let mut spawn_in_loops = SPAWN_GAME_LOOPS;

        let result = panic::catch_unwind(panic::AssertUnwindSafe(
            || -> Result<(), DisplayControllerError> {
                while self.game_state.is_running() {
                    self.reset()?;

                    if poll(Duration::from_millis(100))? {
                        let event = read()?;

                        if event == Event::Key(KeyCode::Esc.into()) {
                            self.output.close()?;

                            break;
                        }

                        self.game_state.keyboard_event = Some(event);
                    }

                    if spawn_in_loops == 0 {
                        self.asteroids.push(Asteroid::new(&self.dimensions));

                        spawn_in_loops = SPAWN_GAME_LOOPS;
                    }

                    if let Some(event) = &self.game_state.keyboard_event {
                        self.player.handle_event(&event);
                    }

                    self.update_positions().draw_all_entities()?;

                    self.output.print_display(&self.display_controller.layout)?;

                    spawn_in_loops -= 1;
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

        self.asteroids.iter_mut().for_each(|asteroid| {
            asteroid.update_position(None);
        });

        self
    }

    pub fn draw_vec<'a>(
        display_controller: &'a mut DisplayController,
        vec_array: &'a mut Vec<impl Drawable>,
    ) -> &'a mut Vec<impl Drawable> {
        vec_array.retain(|drawable| {
            let result = display_controller.draw_drawable(drawable);

            let (_, did_draw) = result.unwrap();

            did_draw
        });

        vec_array
    }

    /// Method to handle drawing all the entities that will be rendered
    fn draw_all_entities(&mut self) -> Result<&mut Self, DisplayControllerError> {
        // Ignore the return type for the borders and player, because we don't delete these at any point
        self.display_controller.draw_drawable(&self.borders)?;
        self.display_controller.draw_drawable(&self.player)?;

        App::draw_vec(
            &mut self.display_controller,
            &mut self.player.bullets.entities,
        );

        App::draw_vec(&mut self.display_controller, &mut self.asteroids);

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
