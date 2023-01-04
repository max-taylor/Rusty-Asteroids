use crate::api::display::{Map, Point};

#[derive(Debug)]
pub struct Drawable {
    pub map: Map,
    pub location: Point,
    pub velocity: u32,
}
