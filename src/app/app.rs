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
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    dimensions: Point<u32>,
}

const SPAWN_GAME_LOOPS: u32 = 5;

impl App {
    pub fn new(dimensions: &Point<u32>) -> Result<App, DisplayControllerError> {
        // let dimensions = dimensions.unwrap_or_else(|| {
        //     let (rows, columns) = size().unwrap();

        //     &Point::new(rows as u32, columns as u32)
        // });

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
            bullets: Default::default(),
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

                    if spawn_in_loops == 0 {
                        self.asteroids.push(Asteroid::new(&self.dimensions));

                        spawn_in_loops = SPAWN_GAME_LOOPS;
                    }

                    // let test_items: Vec<Box<dyn Drawable>> = Vec::new():

                    // let entities_in_frame: Vec<dyn Drawable> = vec![self.player, self.borders];

                    self.display_controller.draw_drawable(&self.borders)?;

                    if let Some(event) = &self.game_state.keyboard_event {
                        if event == &create_event(KeyCode::Enter) {
                            // TODO: Refactor this
                            let spawn_position = self
                                .player
                                .drawable
                                .location
                                .add_width(self.player.drawable.layout.dimensions.width / 2)
                                .sub_height(1);

                            self.bullets.push(Bullet::new(spawn_position));
                        }

                        self.player.handle_event(&event);
                    }

                    update_positions(vec![&mut self.player]);
                    update_positions(self.bullets.iter_mut().collect());
                    update_positions(self.asteroids.iter_mut().collect());

                    self.display_controller
                        .draw_vec_drawable(self.bullets.iter().collect())?;

                    self.display_controller
                        .draw_vec_drawable(self.asteroids.iter().collect())?;

                    self.display_controller.draw_drawable(&self.player)?;

                    // let drawable_items: Vec<impl Drawable> = vec![self.borders];

                    self.output.print_display(&self.display_controller.layout)?;

                    spawn_in_loops -= 1;
                }

                Ok(())
            },
        ));

        //   if let Err(_) = result {
        //     DisplayController::close(&mut self.display_controller.target)?;
        // }

        self.shut_down()?;

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), DisplayControllerError> {
        self.game_state.start_game();
        self.output.start()?;

        Ok(())
    }

    pub fn shut_down(&mut self) -> Result<(), DisplayControllerError> {
        self.output.close();

        Ok(())
    }
}
