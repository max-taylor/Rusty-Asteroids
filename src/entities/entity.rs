use crate::systems::position::Position;

pub trait Entity {
    fn get_entity_position(&self) -> Position;
}
