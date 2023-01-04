use crate::{
    api::display::DisplayController,
    components::{Drawable, DrawableState},
};

pub struct DrawableController<'drawable> {
    /// Positions represent distinct characters, items, etc.
    drawable_entities: Vec<&'drawable DrawableState>,
}

impl<'drawable> DrawableController<'drawable> {
    pub fn new() -> Self {
        Self {
            drawable_entities: vec![],
        }
    }

    pub fn update(&mut self) -> &mut Self {
        // Iterate over each entity and apply its velocity
        self
    }

    pub fn add_drawable_entity(&mut self, drawable: &'drawable impl Drawable) -> &mut Self {
        self.drawable_entities.push(drawable.get_drawable_state());

        self
    }

    pub fn draw_entities(&mut self, display_controller: &mut DisplayController) -> &mut Self {
        for entity in self.drawable_entities.iter() {
            display_controller.draw_drawable(entity);
        }

        self
    }
}

impl<'drawable> Default for DrawableController<'drawable> {
    fn default() -> Self {
        Self {
            drawable_entities: Default::default(),
        }
    }
}
