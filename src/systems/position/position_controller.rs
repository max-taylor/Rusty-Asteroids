use crate::api::DisplayController;

use super::Position;

pub struct PositionController<'dimensions> {
    /// Positions represent distinct characters, items, etc.
    positions: Vec<Position>,
    /// This will interface with the display controller to simplify drawing
    display_controller: DisplayController<'dimensions>,
}

impl<'dimensions> PositionController<'dimensions> {
    pub fn new(positions: Vec<Position>, display_controller: DisplayController) -> Self {
        Self {
            positions,
            display_controller,
        }
    }

    pub fn add_position(&mut self, position: Position) -> &mut Self {
        self.positions.push(position);

        self
    }

    pub fn update(&mut self) -> &mut Self {
        // Iterate over each entity and apply its velocity
        self
    }
}
