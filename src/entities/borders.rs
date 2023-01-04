use crate::{
    api::{Map, Point},
    systems::position::Position,
};

use super::Entity;

pub struct Borders<'dimensions> {
    dimensions: &'dimensions Point,
    position: Position,
}

impl<'dimensions> Borders<'dimensions> {
    pub fn new(dimensions: &'dimensions Point) -> Self {
        let map = Map::new(dimensions);
        // TODO modify map for actual borders

        let position = Position::new(map, Point::home_point(), 0);

        Self {
            dimensions,
            position,
        }
    }
}

impl<'dimensions> Entity for Borders<'dimensions> {
    fn get_entity_position(&self) -> Position {
        self.position
    }
}
