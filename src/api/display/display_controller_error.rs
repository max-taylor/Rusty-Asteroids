use std::fmt::Error;

use crossterm::ErrorKind;

#[derive(Debug, Clone, Copy)]
pub enum DisplayControllerError {
    DisplayTooSmallForDimensions,
    PositionOutOfRange,
    CrossTermWriteError,
    CasteError(Error),
    Shutdown,
}

impl DisplayControllerError {
    pub fn from_crossterm_error(error: ErrorKind) -> Self {
        Self::CrossTermWriteError
    }
}

impl From<ErrorKind> for DisplayControllerError {
    fn from(_: ErrorKind) -> Self {
        Self::CrossTermWriteError
    }
}

impl From<Error> for DisplayControllerError {
    fn from(error: Error) -> Self {
        Self::CasteError(error)
    }
}
