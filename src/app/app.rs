use std::{io::stdout, panic, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use crate::{
    api::display::{DisplayController, DisplayControllerError, Output, Point},
    entities::Borders,
};

use super::game_state::GameState;

pub struct App {
    display_controller: DisplayController,
    output: Output,
    game_state: GameState,
    borders: Borders,
}

impl App {
    pub fn new(dimensions: &Point<u32>) -> Result<App, DisplayControllerError> {
        let mut output = Output::new(stdout());
        let display_controller = DisplayController::new(&dimensions, true);

        if let Some(error) = display_controller.as_ref().err() {
            output.close()?;

            return Err(error.clone());
        }

        Ok(App {
            display_controller: display_controller.unwrap(),
            game_state: GameState::new(),
            borders: Borders::new(dimensions)?,
            output,
        })
    }

    /// Reset method to be called at the start of each loop
    fn reset(&mut self) -> Result<(), DisplayControllerError> {
        self.game_state.keyboard_event = None;

        self.display_controller.layout.reset();

        Ok(())
    }

    pub fn run<F>(&mut self, mut frame_action: F) -> Result<(), DisplayControllerError>
    where
        F: FnMut(&mut GameState, &mut DisplayController) -> Result<(), DisplayControllerError>,
    {
        self.start()?;

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

                    self.display_controller.draw_drawable(&self.borders)?;

                    frame_action(&mut self.game_state, &mut self.display_controller)?;

                    self.output.print_display(&self.display_controller.layout)?;
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
