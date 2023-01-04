use crate::{
    api::display::{DisplayControllerError, Element, Layout, Point},
    components::{Drawable, DrawableState},
};

#[derive(Debug)]
pub struct Borders {
    pub drawable: DrawableState,
}

impl Borders {
    pub fn new(dimensions: &Point) -> Result<Self, DisplayControllerError> {
        let mut drawable = DrawableState {
            map: Layout::new(dimensions, None),
            location: Point::default(),
            velocity: Default::default(),
        };

        drawable.map.draw_rect(
            &Default::default(),
            dimensions,
            Element::new_default_colors('x'),
        )?;

        Ok(Self { drawable })
    }
}

impl Drawable for Borders {
    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }
}
