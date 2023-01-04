use crate::{api::display::Point, entities::Asteroid, helpers::get_now};
use rand::Rng;

pub struct AsteroidController {
    pub asteroids: Vec<Asteroid>,
    pub spawn_rate: u128,
    // Storing this in the struct, so that the game_loop_duration can be provided each loop, this prevents fetching the system time each loop and we already have the game_loop_duration
    time_elapsed_since_spawn: u128,
    dimensions: Point<i64>,
}

fn get_asteroid_spawn_location(dimensions: &Point<i64>) -> Point<i64> {
    let mut rng = rand::thread_rng();

    Point {
        height: 0,
        width: rng.gen_range(0..dimensions.width),
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
            asteroids: vec![],
            spawn_rate,
            time_elapsed_since_spawn: 0,
            dimensions,
        }
    }

    pub fn handle_game_loop(&mut self, game_loop_duration: u128) -> &mut Self {
        self.time_elapsed_since_spawn += game_loop_duration;

        if self.time_elapsed_since_spawn > self.spawn_rate {
            self.time_elapsed_since_spawn = 0;
            self.asteroids
                .push(Asteroid::new(get_asteroid_spawn_location(&self.dimensions)));
        }

        self
    }
}
