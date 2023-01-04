use uuid::Uuid;

use crate::components::{Drawable, DrawableState, Health};

pub struct EntityController<T: Drawable + Health> {
    pub entities: Vec<T>,
}

// pub trait EntityControllerTrait<T: Drawable + Health> {
//     fn new() -> Self;

//     fn get_mut_drawable_array(&self) -> &mut Vec<T>;

//     fn get_all_drawable_states(&self) -> Vec<&DrawableState>;

//     fn spawn_entity(&mut self, entity: T) -> &mut Self;

//     fn apply_entity_damage(&mut self, uuid: Uuid, damage: u32) -> &mut Self;
// }

impl<T: Drawable + Health> EntityController<T> {
    pub fn new() -> Self {
        EntityController { entities: vec![] }
    }

    pub fn get_mut_drawable_array(&mut self) -> &mut Vec<T> {
        &mut self.entities
    }

    pub fn get_all_drawable_states(&self) -> Vec<&DrawableState> {
        self.entities
            .iter()
            .map(|entity| entity.get_drawable_state())
            .collect()
    }

    pub fn spawn_entity(&mut self, entity: T) -> &mut Self {
        self.entities.push(entity);

        self
    }

    pub fn apply_entity_damage(&mut self, uuid: Uuid, damage: u32) -> &mut Self {
        let entity_index = self
            .entities
            .iter_mut()
            .position(|entity| entity.get_drawable_state().uuid == uuid);

        if entity_index.is_none() {
            return self;
        }

        let asteroid_index_2 = entity_index.unwrap();

        // asteroid_index = asteroid_index.unwrap();

        self.entities[asteroid_index_2].apply_damage(damage);

        if self.entities[asteroid_index_2].get_health() == 0 {
            self.entities.remove(asteroid_index_2);
        }

        self
    }
}
