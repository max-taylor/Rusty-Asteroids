use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    api::display::Point, components::DrawableState, helpers::get_is_position_outside_dimensions,
};

pub enum Result {
    // If damage has been inflicted and how much
    Damage(u32),
    // Collisions have different affects depending on its entity type
    Collision,
}

pub struct CollisionOutcome {
    pub result: Result,
}

pub fn run_collision_detection(
    drawable_items: Vec<&DrawableState>,
    dimensions: &Point<i64>,
) -> HashMap<Uuid, Vec<CollisionOutcome>> {
    let mut collision_outcomes: HashMap<Uuid, Vec<CollisionOutcome>> = HashMap::new();

    // Iterating over each drawable item to handle it
    for drawable_state in drawable_items {
        let dimensions = drawable_state.layout.dimensions;

        // Each row in the drawable layout
        for (index, row) in drawable_state.layout.map.iter().enumerate() {
            let height = drawable_state.location.height + index as i64;

            if height < 0 || drawable_state.location.width + (row.len() as i64) < 0 {
                continue;
            }

            for (width, element) in row.iter().enumerate() {
                if let Some(element) = element {
                    let width = drawable_state.location.width + width as i64;

                    if get_is_position_outside_dimensions(&dimensions, &Point { width, height }) {
                        continue;
                    }

                    // match collision_outcomes.get(&drawable_state.uuid) {
                    //     Some(value) => {}
                    //     None => collision_outcomes.insert(&drawable_state.uuid, vec![]),
                    // }

                    // if let Some(value) = collision_outcomes.get(&drawable_state.uuid) {}

                    // match parsed_drawable_items[height as usize][width as usize] {
                    //     Some(items) => {
                    //         items.push(parsed_item);
                    //     }
                    //     None => {
                    //         parsed_drawable_items[height as usize][width as usize] =
                    //             Some(vec![parsed_item]);
                    //     }
                    // };
                }
            }
        }
    }

    collision_outcomes
}
