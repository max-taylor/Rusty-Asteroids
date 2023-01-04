use crossterm::{
    event::{read, Event, KeyCode},
    terminal::enable_raw_mode,
    Result,
};

use crate::{
    display::{DisplayController, Point},
    entities::{Controller, Player},
};

pub struct App {
    display_controller: DisplayController,
    player: Player,
}

impl App {
    pub fn new(dimensions: Point) -> Result<()> {
        enable_raw_mode()?;

        let mut app = App {
            display_controller: DisplayController::new(dimensions),
            player: Player::new(),
        };

        app.setup_listeners();

        Ok(())
    }

    fn setup_listeners(&mut self) {
        loop {
            let event = read().unwrap();

            if event == Event::Key(KeyCode::Esc.into()) {
                self.display_controller.close();

                break;
            }

            self.player.handle_event(event);

            // TODO: Could try and simulate a framerate, as in don't return responses immediately return them on an interval
        }
    }
}
