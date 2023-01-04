use crossterm::style::Color;

use super::{element::parse_str_to_element_array, Element, Point, TwoDVec};

pub type Map = TwoDVec<Option<Element>>;

pub fn create_map<T: Clone>(dimensions: &Point<i64>, default_item: T) -> Vec<Vec<T>> {
    vec![vec![default_item; dimensions.width as usize]; dimensions.height as usize]
}

pub fn map_from_str(str: &str, color: Color) -> Map {
    let rows = str.split("\n");

    let width = rows.into_iter().max_by_key(|row| row.len()).unwrap().len();

    let mut map: Vec<Vec<Option<Element>>> =
        vec![vec![None; width as usize]; str.split("\n").count()];

    for (index, row) in str.split("\n").enumerate() {
        map[index] = parse_str_to_element_array(row, None, Some(color));
    }

    map
}
