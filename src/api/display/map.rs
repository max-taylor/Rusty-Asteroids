use std::vec;

use super::{element::Element, Point};

pub struct Map {
    /// A Map is a 2D vector, where the Vec<_> is the rows and Vec<Vec<_>> are the columns
    pub map: Vec<Vec<Option<Element>>>,
}

impl Map {
    /// Creates a new map with None values for initialization
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The x-y dimensions of the map
    pub fn new(dimensions: &Point) -> Self {
        Self {
            map: vec![vec![None; dimensions.x as usize]; dimensions.y as usize],
        }
    }
}
