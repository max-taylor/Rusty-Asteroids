use std::{
    error::Error,
    io::{self, stdout},
    thread::{self, Scope},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};

use crate::display::DisplayController;

pub struct App<'target> {
    display_controller: DisplayController,
    target: &'target mut io::Stdout,
}

impl<'target> App<'target> {
    pub fn new(target: &'target mut io::Stdout) -> Result<()> {
        enable_raw_mode()?;

        let display_controller = DisplayController::new(50, 50);

        let mut app = App {
            target,
            display_controller,
        };

        let size = size();
        dbg!(size);

        app.setup()?;
        app.setup_listeners();

        Ok(())
    }

    fn setup_listeners(&mut self) {
        loop {
            let event = read().unwrap();

            if event == Event::Key(KeyCode::Esc.into()) {
                self.close();

                break;
            }
        }
    }

    fn setup(&mut self) -> Result<()> {
        execute!(
            self.target,
            EnterAlternateScreen,
            Hide,
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::Red),
        )?;

        Ok(())
    }

    fn close(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.target, LeaveAlternateScreen, Show).unwrap();
    }
}
