use uuid::Uuid;

use crate::{
    api::display::{create_map, Element, Layout, Point},
    components::{Drawable, DrawableState, DrawableType},
    helpers::get_is_position_outside_dimensions,
};

enum Result {
    // If damage has been inflicted and how much
    Damage(u32),
    // Collisions have different affects depending on its entity type
    Collision,
}

struct CollisionResult {
    /// The id of the drawable item with the collision
    pub id: Uuid,
    pub result: Result,
}

#[derive(Clone)]
struct ParsedDrawableItems {
    pub id: Uuid,
    pub drawable_type: DrawableType,
}

pub fn run_collision_detection(drawable_items: Vec<&DrawableState>, dimensions: &Point<i64>) {
    let mut parsed_drawable_items: Vec<Vec<Vec<ParsedDrawableItems>>> =
        create_map(dimensions, vec![]);

    for drawable_state in drawable_items {
        let dimensions = drawable_state.layout.dimensions;

        for (index, row) in drawable_state.layout.map.iter().enumerate() {
            let height = drawable_state.location.height + index as i64;
            if height < 0 {
                continue;
            }

            for (width, element) in row.iter().enumerate() {
                if let Some(element) = element {
                    let width = drawable_state.location.width + width as i64;

                    if get_is_position_outside_dimensions(&dimensions, &Point { width, height }) {
                        continue;
                    }

                    parsed_drawable_items[height as usize][width as usize].push(
                        ParsedDrawableItems {
                            id: drawable_state.uuid,
                            drawable_type: drawable_state.drawable_type,
                        },
                    );
                }
            }
        }
    }
}
