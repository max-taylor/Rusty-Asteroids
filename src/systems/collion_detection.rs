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
    pub enemy_damage: u32,
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

/// UUID of affected drawable item -> UUID of causing drawable item -> collision results
/// This is stored in a complex manner to increase memory access speed when determining collision results
type CollisionResults = HashMap<Uuid, HashMap<Uuid, CollisionOutcome>>;

/// How much damage to apply to an enemy when it collides with a player, this should destroy the enemy if a collision occurs
pub const PLAYER_ENEMY_COLLISION_DAMAGE: u32 = 100;

pub fn run_collision_detection(
    drawable_items: Vec<&DrawableState>,
    dimensions: &Point<i64>,
) -> CollisionResults {
    let mut collision_outcomes: HashMap<Uuid, HashMap<Uuid, CollisionOutcome>> = HashMap::new();

    let positions_with_overlaps = get_positions_with_overlaps(drawable_items, dimensions);

    for elements_on_position in positions_with_overlaps {
        // Single out all the enemies, so that if there is a player or ammunition on the position we can apply affects
        let mut enemies: Vec<&MinimalDrawableDetails> = vec![];

        for element in &elements_on_position {
            match element.drawable_type {
                DrawableType::Enemy(..) => {
                    enemies.push(element);
                }
                _ => {}
            }
        }

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

                    for enemy in &enemies {
                        // Only create a new collision if one doesn't exist already for the same element/enemy combination
                        if !element_collisions.contains_key(&enemy.uuid) {
                            if let DrawableType::Enemy(enemy_damage) = enemy.drawable_type {
                                // If an enemy hit the player, apply the enemies damage to the player and destroy the enemy
                                if element.drawable_type == DrawableType::Player {
                                    element_collisions.insert(
                                        enemy.uuid,
                                        CollisionOutcome {
                                            affected_damage: enemy_damage,
                                            enemy_damage: PLAYER_ENEMY_COLLISION_DAMAGE,
                                            asteroid_uuid: enemy.uuid,
                                        },
                                    );
                                } else {
                                    // If the asteroid collided with ammunition, extract the ammunition damage
                                    if let DrawableType::Ammunition(ammunition_damage) =
                                        element.drawable_type
                                    {
                                        // Apply the enemies damage to the ammunition and the ammunition's damage to the enemy
                                        element_collisions.insert(
                                            enemy.uuid,
                                            CollisionOutcome {
                                                affected_damage: enemy_damage,
                                                enemy_damage: ammunition_damage,
                                                asteroid_uuid: enemy.uuid,
                                            },
                                        );
                                    }
                                }
                            } else {
                                panic!("Error: Not an enemy in the enemies vector");
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

pub struct Summary {
    pub uuid: Uuid,
    pub damage: u32,
}

pub type CollisionSummary = HashMap<Uuid, Summary>;

pub fn apply_damage_to_uuid(collision_summary: &mut CollisionSummary, uuid: Uuid, damage: u32) {
    match collision_summary.get_mut(&uuid) {
        Some(item) => item.damage += damage,
        None => {
            collision_summary.insert(uuid, Summary { uuid, damage });
        }
    };
}

/// Method reduces the collision results hashmap into a summary of damages for each element
pub fn get_collision_summary(collision_results: CollisionResults) -> CollisionSummary {
    let mut collision_summary: CollisionSummary = HashMap::new();

    for (uuid, element_collisions) in collision_results {
        for (_, asteroid_collision) in element_collisions {
            // Apply damage to the element
            apply_damage_to_uuid(
                &mut collision_summary,
                uuid,
                asteroid_collision.affected_damage,
            );
            // Also apply damage to the asteroid
            apply_damage_to_uuid(
                &mut collision_summary,
                asteroid_collision.asteroid_uuid,
                asteroid_collision.enemy_damage,
            );
        }
    }

    collision_summary
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        api::display::{Map, Point, TwoDVec},
        components::Drawable,
        entities::{player, Asteroid, Bullet, Player, ASTEROID_DAMAGE, BULLET_DAMAGE},
        systems::{CollisionResult, PLAYER_ENEMY_COLLISION_DAMAGE},
    };

    use super::{get_collision_summary, get_positions_with_overlaps, run_collision_detection};

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
        assert_eq!(
            player_asteroid_collision.enemy_damage,
            PLAYER_ENEMY_COLLISION_DAMAGE
        );
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
        assert_eq!(
            ammunition_asteroid_collision.affected_damage,
            ASTEROID_DAMAGE
        );

        // Expect that the asteroid took damage equal to the bullets damage
        assert_eq!(ammunition_asteroid_collision.enemy_damage, BULLET_DAMAGE);
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

    #[test]
    fn it_should_return_collisions_and_summarize_for_a_player_ammunition_and_multiple_asteroids() {
        let player = Player::new(Some(POSITION));
        let asteroid1 = get_asteroid_mock();
        let asteroid2 = get_asteroid_mock();
        let ammunition = Bullet::new(POSITION);

        let drawable_states = vec![
            player.get_drawable_state(),
            asteroid1.get_drawable_state(),
            asteroid2.get_drawable_state(),
            ammunition.get_drawable_state(),
        ];

        let collisions = run_collision_detection(
            drawable_states,
            &Point {
                width: 30,
                height: 30,
            },
        );

        let collision_summary = get_collision_summary(collisions);

        assert_eq!(collision_summary.len(), 4);

        for (_, collision) in collision_summary {
            if collision.uuid == player.drawable.uuid || collision.uuid == ammunition.drawable.uuid
            {
                assert_eq!(collision.damage, ASTEROID_DAMAGE * 2);
            } else if collision.uuid == asteroid1.drawable.uuid
                || collision.uuid == asteroid2.drawable.uuid
            {
                assert_eq!(
                    collision.damage,
                    BULLET_DAMAGE + PLAYER_ENEMY_COLLISION_DAMAGE
                );
            } else {
                // Ensure all items are tested
                panic!("No test case for element in collision summary")
            }
        }
    }
}
