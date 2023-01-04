use crate::api::{Element, Point};

type Map = Vec<Vec<Option<Element>>>;

pub struct Position {
    /// 2D vector describing the given entities elements, is an Option<Element> to simplify drawing items that contain deadspace
    map: Map,
    location: Point,
    velocity: u32,
}

impl Position {
    pub fn new(map: Map, location: Point, velocity: u32) -> Self {
        Self {
            map,
            location,
            velocity,
        }
    }
}
