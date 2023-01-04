use uuid::Uuid;

use crate::components::{Drawable, Health};

struct EntityController<T: Drawable + Health> {
    entities: Vec<T>,
}

impl<T: Drawable + Health> EntityController<T> {
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
