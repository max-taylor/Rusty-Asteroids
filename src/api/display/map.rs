use std::vec;

use super::{element::Element, Point};

pub struct Map<'dimensions> {
    pub dimensions: &'dimensions Point,
    /// A Map is a 2D vector, where the Vec<_> is the rows and Vec<Vec<_>> are the columns
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
