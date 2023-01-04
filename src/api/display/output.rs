use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ErrorKind as CrosstermError,
};
use std::io;
use std::io::Write;

use crate::api::display::element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND};

use super::{DisplayControllerError, Element, Layout, Point};

pub struct Output {
    target: io::Stdout,
}

impl Output {
    pub fn new(target: io::Stdout) -> Self {
        Output { target }
    }

    pub fn start(&mut self) -> Result<(), DisplayControllerError> {
        enable_raw_mode().map_err(DisplayControllerError::from_crossterm_error)?;

        queue!(self.target, EnterAlternateScreen, Hide).unwrap();

        Ok(())
    }

    pub fn close(&mut self) -> Result<(), DisplayControllerError> {
        disable_raw_mode().map_err(DisplayControllerError::from_crossterm_error)?;
        execute!(self.target, LeaveAlternateScreen, Show)
            .map_err(DisplayControllerError::from_crossterm_error)?;

        Ok(())
    }

    /// Flushing the target publishes all queued writes
    fn flush(&mut self) -> &mut Self {
        self.target.flush().unwrap();

        self
    }

    pub fn reset_cursor(&mut self) -> Result<(), DisplayControllerError> {
        queue!(
            self.target,
            SetForegroundColor(DEFAULT_FOREGROUND),
            SetBackgroundColor(DEFAULT_BACKGROUND),
            MoveTo(0, 0)
        )
        .unwrap();

        Ok(())
    }

    pub fn print_element(
        &mut self,
        element: &Element,
        move_to: Option<&Point<i64>>,
    ) -> Result<(), DisplayControllerError> {
        if let Some(move_to_destination) = move_to {
            queue!(
                self.target,
                MoveTo(
                    move_to_destination.width as u16,
                    move_to_destination.height as u16
                )
            )
            .map_err(DisplayControllerError::from_crossterm_error)?;
        };

        queue!(
            self.target,
            SetForegroundColor(element.foreground),
            SetBackgroundColor(element.background),
            Print(element.value)
        )
        .map_err(DisplayControllerError::from_crossterm_error)?;

        Ok(())
    }

    pub fn print_display(&mut self, map: &Layout) -> Result<(), DisplayControllerError> {
        self.reset_cursor()?;

        for row in map.map.iter() {
            for element in row.iter() {
                match element {
                    Some(element) => {
                        self.print_element(element, None)?;
                    }
                    None => {
                        self.print_element(&Default::default(), None)?;
                    }
                }
            }
        }

        Ok(())
    }
}
