use crate::api::display::DisplayController;

use super::Position;

pub struct PositionController<'dimensions, 'display_controller> {
    /// Positions represent distinct characters, items, etc.
    positions: Vec<Position<'dimensions>>,
    /// This will interface with the display controller to simplify drawing
    display_controller: &'display_controller mut DisplayController<'dimensions>,
}

impl<'dimensions, 'display_controller> PositionController<'dimensions, 'display_controller> {
    pub fn new(
        positions: Vec<Position<'dimensions>>,
        display_controller: &'display_controller mut DisplayController<'dimensions>,
    ) -> Self {
        Self {
            positions,
            display_controller,
        }
    }

    pub fn add_position(&mut self, position: Position) -> &mut Self {
        // self.positions.push(position);

        self
    }

    pub fn update(&mut self) -> &mut Self {
        // Iterate over each entity and apply its velocity
        self
    }
}
