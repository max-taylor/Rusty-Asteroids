use crate::{
    api::display::{DisplayController, DisplayControllerError},
    components::{Drawable, DrawableState},
};

pub struct DrawableController<'drawable> {
    /// Positions represent distinct characters, items, etc.
    drawable_entities: Vec<&'drawable DrawableState>,
}

impl<'drawable> DrawableController<'drawable> {
    pub fn update(&mut self) -> &mut Self {
        // Iterate over each entity and apply its velocity
        self
    }

    pub fn add_drawable_entity(&mut self, drawable: &'drawable impl Drawable) -> &mut Self {
        self.drawable_entities.push(drawable.get_drawable_state());

        self
    }

    pub fn draw_entities(
        &mut self,
        display_controller: &mut DisplayController,
    ) -> Result<&mut Self, DisplayControllerError> {
        for entity in self.drawable_entities.iter() {
            display_controller.draw_drawable(entity)?;
        }

        // TODO: Collision detection goes here

        Ok(self)
    }
}

impl<'drawable> Default for DrawableController<'drawable> {
    fn default() -> Self {
        Self {
            drawable_entities: Default::default(),
        }
    }
}
