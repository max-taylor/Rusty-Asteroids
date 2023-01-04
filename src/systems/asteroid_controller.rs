use crate::{api::display::Point, entities::Asteroid};
use rand::Rng;

use super::EntityController;

pub struct AsteroidController {
    pub entity_controller: EntityController<Asteroid>,
    pub spawn_rate: u128,
    // Storing this in the struct, so that the game_loop_duration can be provided each loop, this prevents fetching the system time each loop and we already have the game_loop_duration
    time_elapsed_since_spawn: u128,
    dimensions: Point<i64>,
}

fn get_random_in_range(start: i64, end: i64) -> i64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(start..end)
}

fn get_asteroid_spawn_location(dimensions: &Point<i64>) -> Point<i64> {
    Point {
        height: -3,
        width: get_random_in_range(0, dimensions.width),
    }
}

fn get_asteroid_velocity() -> Point<i64> {
    Point {
        width: get_random_in_range(-20, 20),
        height: get_random_in_range(1, 40),
    }
}

impl AsteroidController {
    /// Creates a new instance of the asteroid controller
    ///
    /// # Arguments
    ///
    /// * `spawn_rate` - The spawn rate of asteroids in milliseconds, essentially sets the difficulty
    pub fn new(spawn_rate: u128, dimensions: Point<i64>) -> Self {
        Self {
            spawn_rate,
            time_elapsed_since_spawn: 0,
            dimensions,
            entity_controller: EntityController::new(),
        }
    }

    pub fn handle_game_loop(&mut self, game_loop_duration: u128) -> &mut Self {
        self.time_elapsed_since_spawn += game_loop_duration;

        if self.time_elapsed_since_spawn > self.spawn_rate {
            self.time_elapsed_since_spawn = 0;
            self.entity_controller.spawn_entity(Asteroid::new(
                get_asteroid_spawn_location(&self.dimensions),
                get_asteroid_velocity(),
            ));
        }

        self
    }
}
