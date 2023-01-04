use crate::{
    api::display::{Element, Map, Point},
    components::Drawable,
};

use super::Controller;

pub struct Player {
    pub drawable: Drawable,
    pub move_speed: u32,
}

const SPACE_SHIP: &str = "
       _________
      (=========)
      |=========|
      |====_====|
      |== / \\ ==|
      |= / _ \\/ =|
   _  |=| ( ) |=|
  /=\\ |=|     |=| /=\\
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
 /===\\           /===\\
|||||||         |||||||
-------         -------
 (~~~)           (~~~)
";

impl Player {
    pub fn new(dimensions: &Point) -> Self {
        let location = Point {
            width: 5,
            height: 5,
        };

        let map = Map::from_ascii(SPACE_SHIP);

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
