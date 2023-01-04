use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    api::display::{collapse_twoDVec, create_map, Point, TwoDVec},
    components::{DrawableState, DrawableType},
    helpers::get_is_position_outside_dimensions,
};

#[derive(PartialEq, Debug)]
pub enum CollisionResult {
    // If damage has been inflicted and how much
    Damage(u32),
}

// Enum stores the damage result of the player or ammunition and also the damage result of the asteroid
pub struct CollisionOutcome {
    pub affected_damage: u32,
    pub asteroid_damage: u32,
    pub asteroid_uuid: Uuid,
}

#[derive(Clone, Debug, Copy)]
struct MinimalDrawableDetails {
    pub uuid: Uuid,
    pub drawable_type: DrawableType,
}

/// This method calculates the positions where the provided drawable_items overlap. It returns an array where each item is a position on the grid with overlapping drawable_items, it returns their uuid and drawable_type.
///
/// # Arguments
///
/// * `drawable_items` - The drawable items to check for overlaps
/// * `dimensions` - The game dimensions, ignoring outside of dimensions
///
/// ```
fn get_positions_with_overlaps(
    drawable_items: Vec<&DrawableState>,
    dimensions: &Point<i64>,
) -> TwoDVec<MinimalDrawableDetails> {
    let mut parsed_map: TwoDVec<Option<Vec<MinimalDrawableDetails>>> = create_map(dimensions, None);

    // Iterating over each drawable item to handle it
    for drawable_state in drawable_items {
        // Each row in the drawable layout
        for (index, row) in drawable_state.layout.map.iter().enumerate() {
            let height = drawable_state.location.height + index as i64;

            if height < 0 || height >= dimensions.height {
                continue;
            }

            for (width, element) in row.iter().enumerate() {
                if element.is_none() {
                    continue;
                }

                let width = drawable_state.location.width + width as i64;

                if width < 0 || width >= dimensions.width {
                    continue;
                }

                let parsed_item = MinimalDrawableDetails {
                    uuid: drawable_state.uuid,
                    drawable_type: drawable_state.drawable_type,
                };

                match &mut parsed_map[height as usize][width as usize] {
                    Some(items) => {
                        items.push(parsed_item);
                    }
                    None => {
                        parsed_map[height as usize][width as usize] = Some(vec![parsed_item]);
                    }
                };
            }
        }
    }

    // Reduce the map into a single array of overlapping drawable_items
    let mut drawable_details: TwoDVec<MinimalDrawableDetails> = vec![];

    for item in collapse_twoDVec(parsed_map).iter_mut() {
        if let Some(some_item) = item {
            if some_item.len() > 1 {
                drawable_details.push(some_item.to_vec());
            }
        }
    }

    drawable_details
}

fn get_elements_of_type(
    elements: &Vec<MinimalDrawableDetails>,
    drawable_type: DrawableType,
) -> Vec<&MinimalDrawableDetails> {
    let mut elements_of_type: Vec<&MinimalDrawableDetails> = vec![];
    for element in elements {
        if element.drawable_type == drawable_type {
            elements_of_type.push(element);
        }
    }

    elements_of_type
}

/// UUID of affected drawable item -> UUID of causing drawable item -> collision results
/// This is stored in a complex manner to increase memory access speed when determining collision results
type CollisionResults = HashMap<Uuid, HashMap<Uuid, CollisionOutcome>>;

pub fn run_collision_detection(
    drawable_items: Vec<&DrawableState>,
    dimensions: &Point<i64>,
) -> CollisionResults {
    let mut collision_outcomes: HashMap<Uuid, HashMap<Uuid, CollisionOutcome>> = HashMap::new();

    let positions_with_overlaps = get_positions_with_overlaps(drawable_items, dimensions);

    for elements_on_position in positions_with_overlaps {
        // Single out all the asteroids, so that if there is a player or ammunition on the position we can apply affects
        let asteroids = get_elements_of_type(&elements_on_position, DrawableType::Enemy);

        for element in &elements_on_position {
            match element.drawable_type {
                DrawableType::Player | DrawableType::Ammunition(..) => {
                    // Get the hashmap for element collisions for the player
                    let element_collisions = match collision_outcomes.get_mut(&element.uuid) {
                        // Return the hashmap if it exists
                        Some(collisions) => collisions,
                        // Otherwise create it then return a mutable reference to it
                        None => {
                            collision_outcomes.insert(element.uuid, HashMap::new());

                            collision_outcomes.get_mut(&element.uuid).unwrap()
                        }
                    };

                    for asteroid in &asteroids {
                        // Check if the asteroid has already had collision handling applied
                        if !element_collisions.contains_key(&asteroid.uuid) {
                            // If the asteroid hit the player, do 1 damage to the player and destroy the asteroid with 100 damage
                            if element.drawable_type == DrawableType::Player {
                                element_collisions.insert(
                                    asteroid.uuid,
                                    CollisionOutcome {
                                        affected_damage: 1,
                                        asteroid_damage: 100,
                                        asteroid_uuid: asteroid.uuid,
                                    },
                                );
                            } else {
                                // If the asteroid collided with ammunition, extract the damage applied and destroy the ammunition
                                if let DrawableType::Ammunition(ammunition_damage) =
                                    element.drawable_type
                                {
                                    element_collisions.insert(
                                        asteroid.uuid,
                                        CollisionOutcome {
                                            affected_damage: 100,
                                            asteroid_damage: ammunition_damage,
                                            asteroid_uuid: asteroid.uuid,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    collision_outcomes
}

struct Summary {
    pub uuid: Uuid,
    pub damage: u32,
}

type CollisionSummary = HashMap<Uuid, Summary>;

pub fn get_collision_summary(collision_results: CollisionResults) {
    let mut collision_summary: CollisionSummary = HashMap::new();

    for (uuid, element_collisions) in collision_results {
        let element_summary = match (collision_summary.get_mut(&uuid)) {
            Some(item) => item,
            None => {
                collision_summary.insert(uuid, Summary { uuid, damage: 0 });

                collision_summary.get_mut(&uuid).unwrap()
            }
        };

        for (_, collision) in element_collisions {
            // let asteroid_summary = match (collision_summary.get_mut(&collision.asteroid_uuid)) {
            //     Some(item) => item,
            //     None => {
            //         collision_summary.insert(uuid, Summary { uuid, damage: 0 });

            //         collision_summary.get_mut(&uuid).unwrap()
            //     }
            // };

            element_summary.damage += collision.affected_damage;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::display::{Map, Point, TwoDVec},
        components::Drawable,
        entities::{Asteroid, Bullet, Player, BULLET_DAMAGE},
        systems::CollisionResult,
    };

    use super::{get_positions_with_overlaps, run_collision_detection};

    const POSITION: Point<i64> = Point {
        width: 5,
        height: 5,
    };

    fn get_asteroid_mock() -> Asteroid {
        Asteroid::new(
            POSITION,
            Point {
                width: 1,
                height: 1,
            },
        )
    }

    /**
     *
     */
    #[test]
    fn it_should_return_no_collisions_when_providing_multiple_asteroids() {
        let asteroid = get_asteroid_mock();
        let drawable_states = vec![asteroid.get_drawable_state(); 10];

        let collisions = run_collision_detection(
            drawable_states,
            &Point {
                width: 30,
                height: 30,
            },
        );

        assert_eq!(collisions.len(), 0);
    }

    /**
     *
     */
    #[test]
    fn it_should_return_no_collisions_when_providing_player_and_ammunition() {
        let player = Player::new(Some(POSITION));
        let ammunition = Bullet::new(POSITION);

        let drawable_states = vec![player.get_drawable_state(), ammunition.get_drawable_state()];

        let collisions = run_collision_detection(
            drawable_states,
            &Point {
                width: 30,
                height: 30,
            },
        );

        assert_eq!(collisions.len(), 0);
    }

    /**
     *
     */
    #[test]
    fn it_should_return_a_collision_for_a_player_and_asteroid() {
        let player = Player::new(Some(POSITION));

        let asteroid = get_asteroid_mock();

        let drawable_states = vec![asteroid.get_drawable_state(), player.get_drawable_state()];

        let collisions = run_collision_detection(
            drawable_states,
            &Point {
                width: 30,
                height: 30,
            },
        );

        assert_eq!(collisions.len(), 1);

        let player_collisions = collisions.get(&player.drawable.uuid).unwrap();

        assert_eq!(player_collisions.len(), 1);

        let player_asteroid_collision = player_collisions.get(&asteroid.drawable.uuid).unwrap();

        // Expect that the player took damage
        assert_eq!(player_asteroid_collision.affected_damage, 1);

        // Expect that the asteroid took damage
        assert_eq!(player_asteroid_collision.asteroid_damage, 100);
    }

    /**
     *
     */
    #[test]
    fn it_should_return_a_collision_for_a_asteroid_and_ammunition() {
        let ammunition = Bullet::new(POSITION);

        let asteroid = get_asteroid_mock();

        let drawable_states = vec![
            asteroid.get_drawable_state(),
            ammunition.get_drawable_state(),
        ];

        let collisions = run_collision_detection(
            drawable_states,
            &Point {
                width: 30,
                height: 30,
            },
        );

        assert_eq!(collisions.len(), 1);

        let ammunition_collisions = collisions.get(&ammunition.drawable.uuid).unwrap();

        assert_eq!(ammunition_collisions.len(), 1);

        let ammunition_asteroid_collision =
            ammunition_collisions.get(&asteroid.drawable.uuid).unwrap();

        // Expect that the bullet took damange
        assert_eq!(ammunition_asteroid_collision.affected_damage, 100);

        // Expect that the asteroid took damage equal to the bullets damage
        assert_eq!(ammunition_asteroid_collision.asteroid_damage, BULLET_DAMAGE);
    }

    /**
     *
     */
    #[test]
    fn it_should_return_a_single_collision_for_player_and_multiple_occurrences_of_asteroid() {
        let player = Player::new(Some(POSITION));

        let mut drawable_states = vec![player.get_drawable_state()];

        let asteroid = get_asteroid_mock();
        // Create multiple asteroids and append them to the drawable states vector
        let mut asteroids = vec![asteroid.get_drawable_state(); 10];
        drawable_states.append(&mut asteroids);

        let collisions = run_collision_detection(
            drawable_states,
            &Point {
                width: 30,
                height: 30,
            },
        );

        assert_eq!(collisions.len(), 1);

        let player_collisions = collisions.get(&player.drawable.uuid).unwrap();

        assert_eq!(player_collisions.len(), 1);
    }
}
