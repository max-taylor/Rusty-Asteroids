use crossterm::{
    event::{read, Event, KeyCode},
    terminal::enable_raw_mode,
    Result,
};

use crate::{
    api::{DisplayController, Point},
    entities::{Controller, Player},
    systems::position::{Position, PositionController},
};

pub struct App<'dimensions> {
    display_controller: DisplayController<'dimensions>,
    player: Player,
}

impl<'dimensions> App<'dimensions> {
    pub fn new(dimensions: Point) -> Result<()> {
        enable_raw_mode()?;

        let mut display_controller = DisplayController::new(&dimensions);
        let position_controller = PositionController::new(vec![], &mut display_controller);

        let mut app = App {
            player: Player::new(),
            display_controller,
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
