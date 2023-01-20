use std::time::Duration;

use crossterm::{
    event::{poll, read, Event},
    Result,
};

pub fn get_keyboard_event(delay: u64) -> Result<Option<Event>> {
    // Handle keyboard presses
    if poll(Duration::from_millis(delay))? {
        let event = read()?;

        return Ok(Some(event));
    }

    Ok(None)
}
