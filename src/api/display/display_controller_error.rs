use crossterm::ErrorKind;

#[derive(Debug, Clone, Copy)]
pub enum DisplayControllerError {
    DisplayTooSmallForDimensions,
    PositionOutOfRange,
    CrossTermWriteError,
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
