use std::{io::stdout, panic, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::enable_raw_mode,
};

use crate::{
    api::display::{DisplayController, DisplayControllerError, Map, Point},
    game_state::GameState,
};

pub struct App {
    display_controller: DisplayController,
    game_state: GameState,
}

impl App {
    pub fn new(dimensions: Point) -> Result<App, DisplayControllerError> {
        enable_raw_mode().map_err(DisplayControllerError::from_crossterm_error)?;

        let display_controller = DisplayController::new(&dimensions);

        if let Some(error) = display_controller.as_ref().err() {
            DisplayController::close(&mut stdout())?;

            return Err(error.clone());
        }

        Ok(App {
            display_controller: display_controller.unwrap(),
            game_state: GameState::new(),
        })
    }

    pub fn reset(&mut self) -> Result<(), DisplayControllerError> {
        self.game_state.keyboard_event = None;

        self.display_controller.display.reset();

        Ok(())
    }

    pub fn run<F>(&mut self, mut frame_action: F) -> Result<(), DisplayControllerError>
    where
        F: FnMut(&mut GameState, &mut Map),
    {
        self.game_state.start_game();
        self.display_controller.start();

        let result = panic::catch_unwind(panic::AssertUnwindSafe(
            || -> Result<(), DisplayControllerError> {
                while self.game_state.is_running() {
                    self.reset()?;

                    if poll(Duration::from_millis(100))? {
                        let event = read()?;

                        if event == Event::Key(KeyCode::Esc.into()) {
                            DisplayController::close(&mut self.display_controller.target)?;

                            break;
                        }

                        self.game_state.keyboard_event = Some(event);
                    }

                    frame_action(&mut self.game_state, &mut self.display_controller.display);

                    self.display_controller.print_display()?;

                    // TODO: Handle collison detections with the updated map that is returned

                    // frame_action();
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

    pub fn shut_down(&mut self) -> Result<(), DisplayControllerError> {
        DisplayController::close(&mut self.display_controller.target)?;

        Ok(())
    }
}
