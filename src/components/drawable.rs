use crate::api::display::{Map, Point};

#[derive(Debug)]
pub struct DrawableState {
    pub map: Map,
    pub location: Point,
    pub velocity: u32,
}

pub trait Drawable {
    fn get_drawable_state(&self) -> &DrawableState;
}
