use std::time::Duration;

use crossterm::{
    event::{poll, read, Event},
    Result,
};

pub fn get_keyboard_event() -> Result<Option<Event>> {
    // Handle keyboard presses
    if poll(Duration::from_millis(100))? {
        let event = read()?;

        return Ok(Some(event));
    }

    Ok(None)
}
