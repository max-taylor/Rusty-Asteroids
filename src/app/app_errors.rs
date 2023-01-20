use crossterm::ErrorKind;

use crate::api::display::DisplayControllerError;

#[derive(Debug)]
pub enum AppError {
    DisplayControllerError(DisplayControllerError),
}

impl From<DisplayControllerError> for AppError {
    fn from(error: DisplayControllerError) -> Self {
        Self::DisplayControllerError(error)
    }
}

impl From<ErrorKind> for AppError {
    fn from(_: ErrorKind) -> Self {
        Self::DisplayControllerError(DisplayControllerError::CrossTermWriteError)
    }
}

pub type AppResult<T> = Result<T, AppError>;
