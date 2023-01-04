use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{Drawable, DrawableState, Health};

pub struct EntityController<T: Drawable + Health> {
    entity_hashmap: HashMap<Uuid, T>,
}

impl<T: Drawable + Health> EntityController<T> {
    pub fn new() -> Self {
        EntityController {
            entity_hashmap: HashMap::new(),
        }
    }

    pub fn has_entity(&self, uuid: Uuid) -> bool {
        self.entity_hashmap.get(&uuid).is_some()
    }

    pub fn get_all_drawable_states(&self) -> Vec<&DrawableState> {
        self.entity_hashmap
            .iter()
            .map(|(_, entity)| entity.get_drawable_state())
            .collect()
    }

    pub fn spawn_entity(&mut self, entity: T) -> &mut Self {
        self.entity_hashmap
            .insert(entity.get_drawable_state().uuid, entity);

        self
    }

    pub fn delete_entity(&mut self, uuid: Uuid) -> &mut Self {
        self.entity_hashmap.remove(&uuid);

        self
    }

    /// Applies damage to a given entity associated with the uuid
    /// Returns true if the entity was destroyed
    pub fn apply_entity_damage(&mut self, uuid: Uuid, damage: u32) -> bool {
        let entity = self.entity_hashmap.get_mut(&uuid);

        let mut destroyed = false;

        if let Some(entity) = entity {
            entity.apply_damage(damage);

            if entity.get_health() == 0 {
                self.delete_entity(uuid);

                destroyed = true;
            }
        }

        destroyed
    }

    pub fn update_entity_positions(&mut self, game_loop_duration: u128) -> &mut Self {
        for (_, entity) in self.entity_hashmap.iter_mut() {
            entity.update_position(None, game_loop_duration);
        }

        self
    }
}
