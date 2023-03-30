use crossterm::style::Color;

use crate::helpers::get_is_position_outside_dimensions_with_offset;

use super::{
    element::{parse_str_to_element_array, Element, DEFAULT_BACKGROUND},
    map::{create_map, map_from_str, Map},
    DisplayControllerError, Point,
};

pub type TwoDVec<T> = Vec<Vec<T>>;

pub fn collapse_twoDVec<T>(twoDVec: TwoDVec<T>) -> Vec<T> {
    let mut return_vec: Vec<T> = vec![];

    for row in twoDVec {
        for item in row {
            return_vec.push(item);
        }
    }

    return_vec
}

#[derive(Debug, PartialEq)]
pub struct Layout {
    /// A Map is a 2D vector, where the Vec<_> are rows and Vec<Vec<_>> are items in a row
    pub map: Map,
    pub dimensions: Point<i64>,
    pub default_element: Option<Element>,
}

type LayoutResult<T> = Result<T, DisplayControllerError>;

pub enum Direction {
    Vertical,
    Horizontal,
}

impl Layout {
    /// Creates a new map with None values for initialization
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The x-y dimensions of the map
    pub fn new(dimensions: &Point<i64>, default_element: Option<Element>) -> Self {
        Self {
            map: create_map(dimensions, default_element),
            dimensions: *dimensions,
            default_element,
        }
    }

    pub fn from_map(map: Vec<Vec<Option<Element>>>, default_element: Option<Element>) -> Self {
        let width = map.iter().max_by_key(|item| item.len()).unwrap().len();

        let height = map.len();

        Self {
            dimensions: Point {
                width: width as i64,
                height: height as i64,
            },
            map,
            default_element,
        }
    }

    pub fn from_ascii(ascii: &str, color: Color) -> Self {
        Layout::from_map(map_from_str(ascii, color), None)
    }

    pub fn from_file(path: &str) -> Self {
        let img = image::open(path).unwrap();
        let rgb_img = img.to_rgb8();

        let (width, height) = (img.width(), img.height());

        let raw_rgb = rgb_img.to_vec();

        dbg!(width);
        dbg!(height);

        // Use a 1D vector before converting to the 2D vector to simplify the logic
        let mut converted_items: Vec<Option<Element>> = vec![None; (width * height) as usize];

        for index in (0..converted_items.len()) {
            let base_index = index * 3;
            let (r, g, b) = (
                raw_rgb[base_index],
                raw_rgb[base_index + 1],
                raw_rgb[base_index + 2],
            );

            converted_items[index] = Some(Element::new(
                ' ',
                DEFAULT_BACKGROUND,
                Color::Rgb { r, g, b },
            ));
        }

        let mut map = vec![vec![None; width as usize]; height as usize];

        for index in 0..converted_items.len() as i64 {
            let height = index / width as i64;
            let width = index % width as i64;

            map[height as usize][width as usize] = converted_items[index as usize];
        }

        // for index in (0..raw_rgb.len()).step_by(3) {
        //

        //

        //     // map[]
        // }

        Layout::from_map(map, None)
    }

    /// This method allows drawing an additional map on top of the map contained within this layout. This is useful when drawing ascii art.
    /// # Arguments
    ///
    /// * `map` - The map to draw
    /// * `location` - Location to draw it
    /// * `drawable_offset` - This is used to determine if the map is within the drawable area. When drawing game components this offset will be as large as the user's UI so that it doesn't draw items in the user UI
    pub fn draw_map(
        &mut self,
        map: &Map,
        location: Point<i64>,
        drawable_offset: &Point<i64>,
    ) -> LayoutResult<bool> {
        let mut has_drawn_drawable = false;

        // Iterate over each row in the map
        for (num_row, drawable_row) in map.iter().enumerate() {
            // Then each column in the row
            for num_column in 0..drawable_row.len() {
                if let Some(has_element) = drawable_row[num_column] {
                    let updated_position = location
                        .add_width(num_column as i64)
                        .add_height(num_row as i64);

                    // Check if position is outside of dimension range
                    if get_is_position_outside_dimensions_with_offset(
                        &self.dimensions,
                        &updated_position,
                        drawable_offset,
                    ) {
                        continue;
                    }

                    has_drawn_drawable = true;

                    self.draw_item(has_element, &updated_position)?;
                }
            }
        }

        Ok(has_drawn_drawable)
    }

    pub fn reset(&mut self) -> &mut Self {
        self.map = Layout::new(&self.dimensions, self.default_element).map;

        self
    }

    pub fn draw_str(
        &mut self,
        str: &str,
        position: &Point<i64>,
        background: Option<Color>,
        foreground: Option<Color>,
    ) -> LayoutResult<&mut Self> {
        let element_array = parse_str_to_element_array(str, background, foreground);

        self.draw_element_array(element_array, position)?;

        Ok(self)
    }

    // pub fn get_vec_of_all_points_with_element(&self)

    pub fn get_row(&self, row_number: i64) -> LayoutResult<&Vec<Option<Element>>> {
        let row = self
            .map
            .get(row_number as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(row)
    }

    /// Returns the selected column_number. Has a differing type to get_row because we have to create an array of references and return it, whereas the get_row method returns a pointer to the row. A column doesn't exactly exist, it is just an element at the same row index for each row
    pub fn get_column(&self, column_number: i64) -> LayoutResult<Vec<&Option<Element>>> {
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

    pub fn get_row_mut(&mut self, row_number: i64) -> LayoutResult<&mut Vec<Option<Element>>> {
        let row = self
            .map
            .get_mut(row_number as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(row)
    }

    // pub fn get_column_mut(&mut self, column_number: i64) -> MapResult<Vec<Option<&mut Element>>> {
    //     let mut items: Vec<Option<&mut Element>> =
    //         Vec::with_capacity(self.dimensions.height as usize);

    //     for height in 0..self.dimensions.height as usize {
    //         let element = self.map[height][column_number as usize].as_mut();

    //         items[height] = element;
    //     }

    //     Ok(items)
    // }

    pub fn get_element_mut(&mut self, point: &Point<i64>) -> LayoutResult<&mut Option<Element>> {
        let row = self.get_row_mut(point.height)?;

        let element = row
            .get_mut(point.width as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(element)
    }

    pub fn get_element(&self, point: &Point<i64>) -> LayoutResult<&Option<Element>> {
        let row = self.get_row(point.height)?;

        let element = row
            .get(point.width as usize)
            .ok_or(DisplayControllerError::PositionOutOfRange)?;

        Ok(element)
    }

    pub fn draw_rect(
        &mut self,
        start_position: &Point<i64>,
        dimensions: &Point<i64>,
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
        len: i64,
        start_position: &Point<i64>,
        direction: Direction,
    ) -> Result<&mut Self, DisplayControllerError> {
        match direction {
            Direction::Horizontal => {
                for index in 0..len {
                    let existing_element =
                        self.get_element_mut(&start_position.add_width(index))?;

                    *existing_element = Some(element);
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

    pub fn draw_element_array(
        &mut self,
        elements: Vec<Option<Element>>,
        position: &Point<i64>,
    ) -> Result<&mut Self, DisplayControllerError> {
        for (index, element) in elements.iter().enumerate() {
            let element_position = &position.add_width(index as i64);

            if let Some(element) = element {
                self.draw_item(*element, element_position)?;
            }
        }

        Ok(self)
    }

    pub fn draw_item(
        &mut self,
        element: Element,
        position: &Point<i64>,
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
    use crate::api::display::{Element, Layout, Point};

    use super::Direction;

    const WIDTH: i64 = 30;
    const HEIGHT: i64 = 20;

    const DIMENSIONS: &Point<i64> = &Point::new(WIDTH, HEIGHT);

    const DEFAULT_ELEMENT: Element = Element::default();

    #[test]
    fn it_creates_correct_dimensions() {
        let map = Layout::new(DIMENSIONS, None);

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
        let mut map = Layout::new(DIMENSIONS, None);

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
        let mut map = Layout::new(DIMENSIONS, None);

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
        let mut map = Layout::new(&Point::new(WIDTH, HEIGHT), None);

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
