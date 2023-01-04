use crossterm::style::Color;

use super::{
    element::{parse_str_to_element_array, Element},
    DisplayControllerError, Point,
};

#[derive(Debug)]
pub struct Map {
    /// A Map is a 2D vector, where the Vec<_> are rows and Vec<Vec<_>> are columns
    pub map: Vec<Vec<Option<Element>>>,
    pub dimensions: Point,
    pub default_element: Option<Element>,
}

type MapResult<T> = Result<T, DisplayControllerError>;

pub enum Direction {
    Vertical,
    Horizontal,
}
impl Map {
    /// Creates a new map with None values for initialization
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The x-y dimensions of the map
    pub fn new(dimensions: &Point, default_element: Option<Element>) -> Self {
        Self {
            map: vec![vec![default_element; dimensions.width as usize]; dimensions.height as usize],
            dimensions: *dimensions,
            default_element,
        }
    }

    pub fn from_map(map: Vec<Vec<Option<Element>>>, default_element: Option<Element>) -> Self {
        let width = map.iter().max_by_key(|item| item.len()).unwrap().len();

        let height = map.len();

        Self {
            dimensions: Point {
                width: width as u32,
                height: height as u32,
            },
            map,
            default_element,
        }
    }

    pub fn from_ascii(ascii: &str) -> Self {
        let rows = ascii.split("\n");

        let width = rows.into_iter().max_by_key(|row| row.len()).unwrap().len();

        let audit_element = Element::new(' ', Color::Cyan, Color::Cyan);

        let mut map: Vec<Vec<Option<Element>>> =
            vec![vec![Some(audit_element); width as usize]; ascii.split("\n").count()];

        // for (index, row) in ascii.split("\n").enumerate() {
        //     map[index] = parse_str_to_element_array(row);
        // }

        Map::from_map(map, None)
    }

    pub fn reset(&mut self) -> &mut Self {
        self.map = Map::new(&self.dimensions, self.default_element).map;

        self
    }

    pub fn get_row(&self, row_number: u32) -> MapResult<&Vec<Option<Element>>> {
        let row = self
            .map
            .get(row_number as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(row)
    }

    /// Returns the selected column_number. Has a differing type to get_row because we have to create an array of references and return it, whereas the get_row method returns a pointer to the row. A column doesn't exactly exist, it is just an element at the same row index for each row
    pub fn get_column(&self, column_number: u32) -> MapResult<Vec<&Option<Element>>> {
        let mut items: Vec<&Option<Element>> = Vec::with_capacity(self.dimensions.height as usize);

        for height in 0..self.dimensions.height {
            let element = self.get_element(&Point {
                width: column_number,
                height,
            })?;

            items.push(element);
        }

        Ok(items)
    }

    pub fn get_row_mut(&mut self, row_number: u32) -> MapResult<&mut Vec<Option<Element>>> {
        let row = self
            .map
            .get_mut(row_number as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(row)
    }

    // pub fn get_column_mut(&mut self, column_number: u32) -> MapResult<Vec<&mut Option<Element>>> {
    // let mut items: Vec<&mut Option<Element>> =
    //     Vec::with_capacity(self.dimensions.height as usize);

    // for height in 0..self.dimensions.height {
    //     let element = self.get_element_mut(&Point {
    //         width: column_number,
    //         height,
    //     })?;

    //     items[height as usize] = element;
    // }

    //     Ok(items)
    // }

    pub fn get_element_mut(&mut self, point: &Point) -> MapResult<&mut Option<Element>> {
        let row = self.get_row_mut(point.height)?;

        let element = row
            .get_mut(point.width as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(element)
    }

    pub fn get_element(&self, point: &Point) -> MapResult<&Option<Element>> {
        let row = self.get_row(point.height)?;

        let element = row
            .get(point.width as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(element)
    }

    pub fn draw_rect(
        &mut self,
        start_position: &Point,
        dimensions: &Point,
        element: Element,
    ) -> Result<&mut Self, DisplayControllerError> {
        self.draw_line(
            element,
            dimensions.width,
            start_position,
            Direction::Horizontal,
        )?
        .draw_line(
            element,
            dimensions.width,
            &start_position.add_height(dimensions.height - 1),
            Direction::Horizontal,
        )?
        .draw_line(
            element,
            dimensions.height,
            start_position,
            Direction::Vertical,
        )?
        .draw_line(
            element,
            dimensions.height,
            &start_position.add_width(dimensions.width - 1),
            Direction::Vertical,
        )?;

        Ok(self)
    }

    // TODO: Add docs describing that the line draws from top->bottom
    pub fn draw_line(
        &mut self,
        element: Element,
        len: u32,
        start_position: &Point,
        direction: Direction,
    ) -> Result<&mut Self, DisplayControllerError> {
        match direction {
            Direction::Horizontal => {
                let row = self.get_row_mut(start_position.height)?;

                for index in 0..len {
                    row[index as usize] = Some(element);
                }
            }
            Direction::Vertical => {
                for index in 0..len {
                    let existing_element =
                        self.get_element_mut(&start_position.add_height(index))?;

                    *existing_element = Some(element);
                }
            }
        }

        Ok(self)
    }

    pub fn draw_item(
        &mut self,
        element: Element,
        position: &Point,
    ) -> Result<&mut Self, DisplayControllerError> {
        // Position is exclusive of the dimension borders
        if position.width >= self.dimensions.width || position.height >= self.dimensions.height {
            return Err(DisplayControllerError::PositionOutOfRange);
        }

        let existing_element = self.get_element_mut(&position)?;

        *existing_element = Some(element);

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::display::{
        element::{DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Map, Point,
    };

    use super::Direction;

    const WIDTH: u32 = 30;
    const HEIGHT: u32 = 20;

    const DIMENSIONS: &Point = &Point::new(WIDTH, HEIGHT);

    const DEFAULT_ELEMENT: Element = Element::default();

    #[test]
    fn it_creates_correct_dimensions() {
        let map = Map::new(DIMENSIONS, None);

        // Ensure the correct amount of rows
        assert_eq!(map.map.len(), HEIGHT as usize);

        // Ensure correct amount of items in each row
        for row in map.map.iter() {
            assert_eq!(row.len(), WIDTH as usize);
        }
    }

    const LINE_ELEMENT: Element = Element::new_default_colors('x');

    #[test]
    fn it_draws_a_horizontal_line() {
        let mut map = Map::new(DIMENSIONS, None);

        map.draw_line(
            LINE_ELEMENT,
            WIDTH,
            &Point::default(),
            Direction::Horizontal,
        )
        .unwrap();

        for existing_element in map.get_row(0).unwrap() {
            assert_eq!(existing_element, &Some(LINE_ELEMENT));
        }
    }

    #[test]
    fn it_draws_a_vertical_line() {
        let mut map = Map::new(DIMENSIONS, None);

        let column_number = 5;

        map.draw_line(
            LINE_ELEMENT,
            HEIGHT,
            &Point {
                width: column_number,
                height: 0,
            },
            Direction::Vertical,
        )
        .unwrap();

        for existing_element in map.get_column(column_number).unwrap() {
            assert_eq!(existing_element, &Some(LINE_ELEMENT));
        }
    }

    #[test]
    fn it_draws_a_rect() {
        let mut map = Map::new(&Point::new(WIDTH, HEIGHT), None);

        map.draw_rect(&Default::default(), DIMENSIONS, LINE_ELEMENT)
            .unwrap();

        let top_row = map.get_row(0).unwrap();

        assert_eq!(top_row, &vec![Some(LINE_ELEMENT); WIDTH as usize]);

        // Rows start at 0, so subtract 1
        let bottom_row = map.get_row(DIMENSIONS.height - 1).unwrap();

        assert_eq!(bottom_row, &vec![Some(LINE_ELEMENT); WIDTH as usize]);

        let left_column = map.get_column(0).unwrap();

        assert_eq!(left_column, vec![&Some(LINE_ELEMENT); HEIGHT as usize]);

        // Columns also start at 0, so subtract 1
        let right_column = map.get_column(WIDTH - 1).unwrap();

        assert_eq!(right_column, vec![&Some(LINE_ELEMENT); HEIGHT as usize]);
    }
}
