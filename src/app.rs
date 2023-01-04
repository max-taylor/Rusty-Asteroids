use std::io::{stdout, Error, ErrorKind};

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::enable_raw_mode,
    ErrorKind as CrosstermError,
};

use crate::{
    api::{DisplayController, Point},
    entities::{Controller, Player},
    systems::position::{Position, PositionController},
};

pub struct App<'dimensions> {
    player: Player,
    display_controller: DisplayController<'dimensions>, // position_controller: PositionController<'dimensions, 'position_controller>,
}

impl<'dimensions> App<'dimensions> {
    pub fn new(dimensions: Point) -> Result<(), CrosstermError> {
        enable_raw_mode()?;

        let mut display_controller = DisplayController::new(&dimensions);

        if let Some(error) = display_controller.as_ref().err() {
            DisplayController::close(&mut stdout());

            let cloned_error = error.clone();

            return Err(Error::new(cloned_error.kind(), "Boot-up error"));
        }

        // let position_controller = PositionController::new(vec![], &mut display_controller);

        let mut app = App {
            player: Player::new(),
            display_controller: display_controller.unwrap(), // position_controller,
        };

        app.setup_listeners();

        Ok(())
    }

    fn setup_listeners(&mut self) {
        loop {
            let event = read().unwrap();

            if event == Event::Key(KeyCode::Esc.into()) {
                DisplayController::close(&mut self.display_controller.target);

                break;
            }

            if event == Event::Key(KeyCode::Left.into()) {}

            self.player.handle_event(event);

            // TODO: Could try and simulate a framerate, as in don't return responses immediately return them on an interval
        }
    }
}
