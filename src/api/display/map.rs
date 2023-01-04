use std::vec::{self, IntoIter};

use super::{element::Element, DisplayControllerError, Point};

pub struct Map {
    /// A Map is a 2D vector, where the Vec<_> is the rows and Vec<Vec<_>> are the columns
    pub map: Vec<Vec<Option<Element>>>,
}

type MapResult<T> = Result<T, DisplayControllerError>;

impl Map {
    /// Creates a new map with None values for initialization
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The x-y dimensions of the map
    pub fn new(dimensions: &Point, default_element: Option<Element>) -> Self {
        Self {
            map: { vec![vec![default_element; dimensions.x as usize]; dimensions.y as usize] },
        }
    }

    pub fn get_row(&self, row_number: u32) -> MapResult<&Vec<Option<Element>>> {
        let row = self
            .map
            .get(row_number as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(row)
    }

    pub fn get_row_mut(&mut self, row_number: u32) -> MapResult<&mut Vec<Option<Element>>> {
        let row = self
            .map
            .get_mut(row_number as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(row)
    }

    pub fn get_element_mut(&mut self, point: &Point) -> MapResult<&mut Option<Element>> {
        let row = self.get_row_mut(point.x)?;

        let element = row
            .get_mut(point.y as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(element)
    }
}

// impl IntoIterator for &Map {
//     type Item = Vec<Option<Element>>;

//     type IntoIter = IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         &self.map.into_iter()
//     }
// }
