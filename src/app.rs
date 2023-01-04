use std::{io::stdout, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{enable_raw_mode, size},
};

use crate::{
    api::display::{DisplayController, DisplayControllerError, Map, Point},
    entities::{Borders, Controller, Player},
};

pub struct App {
    player: Player,
    display_controller: DisplayController,
    borders: Borders,
}

pub struct LoopState {
    keyboard_event: Option<Event>,
}

impl App {
    pub fn new(dimensions: Point) -> Result<(), DisplayControllerError> {
        enable_raw_mode().map_err(DisplayControllerError::from_crossterm_error)?;

        let display_controller = DisplayController::new(&dimensions);

        if let Some(error) = display_controller.as_ref().err() {
            DisplayController::close(&mut stdout())?;

            return Err(error.clone());
        }

        let mut app = App {
            player: Player::new(),
            display_controller: display_controller.unwrap(), // position_controller,
            borders: Borders::new(&dimensions)?,
        };

        if let Err(err) = app.setup_listeners() {
            DisplayController::close(&mut stdout())?;

            return Err(err);
        }

        Ok(())
    }

    // pub fn run<F>(&mut self, mut frame_action: F)
    // where
    //     F: FnMut(&mut LoopState, &mut Map),
    // {
    //     self.display_controller.display.reset();

    //     while

    //     frame_action();
    // }

    fn setup_listeners(&mut self) -> Result<(), DisplayControllerError> {
        loop {
            // TODO: A high-order function that acts a game loop and does the resetting and other house keeping would be ideal
            self.display_controller.display.reset();

            if poll(Duration::from_millis(100))? {
                let event = read()?;

                if event == Event::Key(KeyCode::Esc.into()) {
                    DisplayController::close(&mut self.display_controller.target)?;

                    break;
                }

                self.player.handle_event(&event);
            }

            self.display_controller
                .draw_drawable(&self.borders.drawable)?;

            self.display_controller
                .draw_drawable(&self.player.drawable)?;
            // TODO: Add collision detection after updates

            self.display_controller.print_display()?;

            // TODO: Could try and simulate a framerate, as in don't return responses immediately return them on an interval
        }

        Ok(())
    }
}
