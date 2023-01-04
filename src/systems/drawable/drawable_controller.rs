use crate::{api::display::DisplayController, components::Drawable};

pub struct DrawableController {
    /// Positions represent distinct characters, items, etc.
    positions: Vec<&Drawable>,
}

impl DrawableController {
    pub fn new(positions: Vec<&Drawable>) -> Self {
        Self { positions }
    }

    pub fn update(&mut self) -> &mut Self {
        // Iterate over each entity and apply its velocity
        self
    }

    pub fn add_drawable(&mut self, drawable: &Drawable) -> &mut Self {
        self.positions.push(drawable);

        self
    }
}
