use crate::{
    api::display::{DisplayControllerError, Element, Map, Point},
    components::DrawableState,
};

#[derive(Debug)]
pub struct Borders {
    pub drawable: DrawableState,
}

impl Borders {
    pub fn new(dimensions: &Point) -> Result<Self, DisplayControllerError> {
        let mut drawable = DrawableState {
            map: Map::new(dimensions, None),
            location: Point::default(),
            velocity: 0,
        };

        drawable.map.draw_rect(
            &Default::default(),
            dimensions,
            Element::new_default_colors('x'),
        )?;

        Ok(Self { drawable })
    }
}
