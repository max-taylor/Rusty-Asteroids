use std::sync::Arc;

use crate::{
    api::display::{
        element::{parse_str_to_element_array, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND},
        Element, Map, Point,
    },
    components::Drawable,
};

use super::Controller;

pub struct Player {
    pub drawable: Drawable,
    pub move_speed: u32,
}

const ARROW_ELEMENT: Element = Element::new('^', DEFAULT_BACKGROUND, DEFAULT_FOREGROUND);

const SPACE_SHIP: &str = "       _________
      (=========)
      |=========|
      |====_====|
      |== / \\ ==|
      |= / _ \\/ =|
   _  |=| ( ) |=|
  /=\\ |=|     |=| /=\
  |=| |=| USA |=| |=|
  |=| |=|  _  |=| |=|
  |=| |=| | | |=| |=|
  |=| |=| | | |=| |=|
  |=| |=| | | |=| |=|
  |=| |/  | |  \\| |=|
  |=|/    | |    \\|=|
  |=/NASA |_| NASA\\=|
  |(_______________)|
  |=| |_|__|__|_| |=|
  |=|   ( ) ( )   |=|
 /===\\           /===\
|||||||         |||||||
-------         -------
 (~~~)           (~~~)
";

fn parse_ascii_to_map(ascii: &str) -> Map {
    let rows = ascii.split("\n");

    let width = rows.into_iter().max_by_key(|row| row.len()).unwrap().len();

    let mut map: Vec<Vec<Option<Element>>> =
        vec![vec![None; ascii.split("\n").count() as usize]; width];

    for (index, row) in ascii.split("\n").enumerate() {
        map[index] = parse_str_to_element_array(row);
    }

    Map::from_map(map, None)

    // let vec = Map::new(
    //     &Point {
    //         width: width as u32,
    //         height: rows. as u32,
    //     },
    //     None,
    // );

    // for row in rows {

    // }
}

impl Player {
    pub const ascii: &str = "
       ^ 
      ^^^
    ";

    pub fn new(dimensions: &Point) -> Self {
        let location = *dimensions
            / Point {
                width: 2,
                height: 2,
            };

        let map = parse_ascii_to_map(SPACE_SHIP);

        // let spaceship: Vec<Vec<Option<Element>>> = [
        //     [None, Some(ARROW_ELEMENT), None],
        //     [
        //         Some(ARROW_ELEMENT),
        //         Some(ARROW_ELEMENT),
        //         Some(ARROW_ELEMENT),
        //     ],
        // ]
        // .map(|row| row.to_vec())
        // .to_vec();

        // let map = Map::from_map(spaceship, None);

        Self {
            drawable: Drawable { map, location },
            move_speed: 1,
        }
    }
}

impl Controller for Player {
    fn up(&mut self) -> &mut Self {
        self.drawable.location.height -= self.move_speed;

        self
    }

    fn down(&mut self) -> &mut Self {
        self.drawable.location.height += self.move_speed;

        self
    }

    fn left(&mut self) -> &mut Self {
        self.drawable.location.width -= self.move_speed;

        self
    }

    fn right(&mut self) -> &mut Self {
        self.drawable.location.width += self.move_speed;

        self
    }
}
