use rand::Rng;
use std::{io::stdout, panic, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::size,
};

use crate::{
    api::display::{DisplayController, DisplayControllerError, Output, Point},
    components::Drawable,
    entities::{controller::create_event, Asteroid, Borders, Bullet, Controller, Player},
    systems::update_positions,
};

use super::game_state::GameState;

pub struct App {
    display_controller: DisplayController,
    output: Output,
    game_state: GameState,
    borders: Borders,
    player: Player,
    // bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    dimensions: Point<i64>,
}

const SPAWN_GAME_LOOPS: i64 = 5;

impl App {
    pub fn new(dimensions: &Point<i64>) -> Result<App, DisplayControllerError> {
        dbg!(dimensions);
        let mut output = Output::new(stdout());

        let display_controller = DisplayController::new(dimensions);

        if let Some(error) = display_controller.as_ref().err() {
            output.close()?;

            return Err(error.clone());
        }

        Ok(App {
            display_controller: display_controller.unwrap(),
            game_state: GameState::new(),
            borders: Borders::new(dimensions)?,
            output,
            player: Player::new(),
            // bullets: Default::default(),
            asteroids: Default::default(),
            dimensions: *dimensions,
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

                    // if spawn_in_loops == 0 {
                    // self.asteroids.push(Asteroid::new(&self.dimensions));

                    // spawn_in_loops = SPAWN_GAME_LOOPS;
                    // }

                    // let test_items: Vec<Box<dyn Drawable>> = Vec::new():

                    // let entities_in_frame: Vec<dyn Drawable> = vec![self.player, self.borders];

                    if let Some(event) = &self.game_state.keyboard_event {
                        self.player.handle_event(&event);
                    }

                    self.update_positions().draw_all_entities()?;

                    self.output.print_display(&self.display_controller.layout)?;
                }
                dbg!("HMMM????");
                dbg!(self.game_state.is_running());
                Ok(())
            },
        ));
        dbg!("Done!");
        //   if let Err(_) = result {
        //     DisplayController::close(&mut self.display_controller.target)?;
        // }

        self.shut_down()?;

        Ok(())
    }

    fn update_positions(&mut self) -> &mut Self {
        self.player.update_position();

        self.player.bullets.entities.iter_mut().for_each(|bullet| {
            bullet.update_position();
        });

        self.asteroids.iter_mut().for_each(|asteroid| {
            asteroid.update_position();
        });

        self
    }

    /// Method to handle drawing all the entities that will be rendered
    fn draw_all_entities(&mut self) -> Result<&mut Self, DisplayControllerError> {
        self.display_controller
            .draw_drawable(&self.borders)?
            .draw_drawable(&self.player)?
            .draw_vec_drawable(self.player.bullets.entities.iter().collect())?
            .draw_vec_drawable(self.asteroids.iter().collect())?;

        Ok(self)
    }

    pub fn start(&mut self) -> Result<(), DisplayControllerError> {
        self.game_state.start_game();
        self.output.start()?;

        Ok(())
    }

    pub fn shut_down(&mut self) -> Result<(), DisplayControllerError> {
        dbg!("Shutting down");
        self.output.close();

        Ok(())
    }
}
