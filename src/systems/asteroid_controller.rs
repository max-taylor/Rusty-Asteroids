use crate::{
    api::display::Point,
    components::{Drawable, DrawableState, Health},
    entities::Asteroid,
    helpers::get_now,
};
use rand::Rng;
use uuid::Uuid;

pub struct AsteroidController {
    pub asteroids: Vec<Asteroid>,
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
            self.asteroids.push(Asteroid::new(
                get_asteroid_spawn_location(&self.dimensions),
                get_asteroid_velocity(),
            ));
        }

        self
    }

    pub fn get_all_drawable_states(&self) -> Vec<&DrawableState> {
        self.asteroids
            .iter()
            .map(|asteroid| asteroid.get_drawable_state())
            .collect()
    }

    pub fn apply_asteroid_damage(&mut self, uuid: Uuid, damage: u32) -> &mut Self {
        let asteroid_index = self
            .asteroids
            .iter_mut()
            .position(|asteroid| asteroid.drawable.uuid == uuid);

        if asteroid_index.is_none() {
            return self;
        }

        // TODO: Clean this
        let asteroid_index_2 = asteroid_index.unwrap();

        // asteroid_index = asteroid_index.unwrap();

        self.asteroids[asteroid_index_2].apply_damage(damage);

        if self.asteroids[asteroid_index_2].health == 0 {
            self.asteroids.remove(asteroid_index_2);
        }

        self
    }
}
