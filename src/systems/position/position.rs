use crate::api::display::{Map, Point};

pub struct Position<'dimensions> {
    /// 2D vector describing the given entities elements, is an Option<Element> to simplify drawing items that contain deadspace
    map: Map<'dimensions>,
    location: Point,
    velocity: u32,
}

impl<'dimensions> Position<'dimensions> {
    pub fn new(map: Map<'dimensions>, location: Point, velocity: u32) -> Self {
        Self {
            map,
            location,
            velocity,
        }
    }
}
