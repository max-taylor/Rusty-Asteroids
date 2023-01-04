use crate::systems::position::Position;

// #[derive(Sized)]
pub trait Entity {
    fn get_entity_position(&self) -> &Position;
}

struct Manager {
    // entities: Vec<dyn Entity>,
}

impl Manager {
    pub fn add_entity(&mut self, entity: &dyn Entity) -> &mut Self {
        self
    }
}
