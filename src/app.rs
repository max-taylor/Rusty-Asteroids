use crossterm::{
    event::{read, Event, KeyCode},
    terminal::enable_raw_mode,
    Result,
};

use crate::{
    entities::{Controller, Player},
    systems::display::{DisplayController, Point},
};

pub struct App<'dimensions> {
    display_controller: DisplayController<'dimensions>,
    player: Player,
}

impl<'dimensions> App<'dimensions> {
    pub fn new(dimensions: Point) -> Result<()> {
        enable_raw_mode()?;

        let mut app = App {
            display_controller: DisplayController::new(&dimensions),
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

            if event == Event::Key(KeyCode::Left.into()) {}

            self.player.handle_event(event);

            // TODO: Could try and simulate a framerate, as in don't return responses immediately return them on an interval
        }
    }
}
