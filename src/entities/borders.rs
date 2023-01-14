use crossterm::style::Color;
use uuid::Uuid;

use crate::{
    api::display::{element::DEFAULT_BACKGROUND, DisplayControllerError, Element, Layout, Point},
    components::{Drawable, DrawableState, DrawableType},
};

#[derive(Debug)]
pub struct Borders {
    pub drawable: DrawableState,
}

impl Borders {
    pub fn new(dimensions: &Point<i64>) -> Result<Self, DisplayControllerError> {
        let mut drawable = DrawableState {
            layout: Layout::new(dimensions, None),
            location: Point::default(),
            velocity: Default::default(),
            drawable_type: DrawableType::Border,
            uuid: Uuid::new_v4(),
        };

        drawable.layout.draw_rect(
            &Default::default(),
            dimensions,
            Element::new(' ', Color::Red, Color::Red),
        )?;

        Ok(Self { drawable })
    }
}

impl Drawable for Borders {
    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }

    fn set_position(&mut self, updated_position: Point<i64>) -> &mut Self {
        self.drawable.location = updated_position;

        self
    }
}
