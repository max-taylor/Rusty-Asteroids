use crossterm::event::KeyCode;

use crate::{
    api::display::{Layout, Point},
    components::{Drawable, DrawableState, DrawableType, Spawnable},
};

use super::{consts::SPACE_SHIP, controller::create_event, Bullet, Controller};

pub struct Player {
    pub drawable: DrawableState,
    pub bullets: Vec<Bullet>,
}

const WIDTH_MAX_VELOCITY: i64 = 33;
const HEIGHT_MAX_VELOCITY: i64 = 20;

trait CanSpawn {
    fn get_spawnable_entities<T>(&self) -> Spawnable<T>;
}

impl Player {
    pub fn new(mut default_position: Option<Point<i64>>) -> Self {
        if default_position.is_none() {
            default_position = Some(Point {
                width: 5,
                height: 5,
            });
        }

        let layout = Layout::from_ascii(SPACE_SHIP);

        Self {
            drawable: DrawableState::new(
                layout,
                default_position.unwrap(),
                DrawableType::Player,
                None,
            ),
            bullets: Default::default(),
        }
    }

    pub fn get_all_drawable_states(&self) -> Vec<&DrawableState> {
        self.bullets
            .iter()
            .map(|asteroid| asteroid.get_drawable_state())
            .collect()
    }

    // TODO: Implement different bullet types
    // pub fn get_spawnable_entities(&self) -> Spawnable<Bullet> {
    // self.bullets
    // }
}

impl Drawable for Player {
    fn get_drawable_state(&self) -> &DrawableState {
        &self.drawable
    }

    fn set_position(&mut self, updated_position: Point<i64>) -> &mut Self {
        self.drawable.location = updated_position;

        self
    }
}

impl Controller for Player {
    fn up(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(0, -HEIGHT_MAX_VELOCITY);

        self
    }

    fn down(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(0, HEIGHT_MAX_VELOCITY);

        self
    }

    fn left(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(-WIDTH_MAX_VELOCITY, 0);

        self
    }

    fn right(&mut self) -> &mut Self {
        self.drawable.velocity = Point::new(WIDTH_MAX_VELOCITY, 0);

        self
    }

    fn additional_event_logic(&mut self, event: &crossterm::event::Event) -> &mut Self {
        // self.drawable.velocity = Default::default();

        if event == &create_event(KeyCode::Enter) {
            let spawn_position = self
                .drawable
                .location
                .add_width(self.drawable.layout.dimensions.width / 2 - 1)
                .add_height(1);

            self.bullets.push(Bullet::new(spawn_position));
        }

        self
    }
}
