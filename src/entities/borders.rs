use crate::{
    api::display::{DisplayControllerError, Element, Map, Point},
    components::Drawable,
};

#[derive(Debug)]
pub struct Borders {
    pub drawable: Drawable,
}

impl Borders {
    pub fn new(dimensions: &Point) -> Result<Self, DisplayControllerError> {
        let mut drawable = Drawable {
            map: Map::new(dimensions, None),
            location: Point::default(),
        };

        drawable.map.draw_rect(
            &Default::default(),
            dimensions,
            Element::new_default_colors('x'),
        )?;

        Ok(Self { drawable })
    }
}
