use std::vec;

use super::{element::Element, Point};

pub struct Map<'dimensions> {
    pub dimensions: &'dimensions Point,
    pub map: Vec<Vec<Option<Element>>>,
}

impl<'dimensions> Map<'dimensions> {
    /// Creates a new map with None values for initialization
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The x-y dimensions of the map
    pub fn new(dimensions: &'dimensions Point) -> Self {
        Self {
            dimensions,
            map: vec![vec![None; dimensions.x as usize]; dimensions.y as usize],
        }
    }
}
