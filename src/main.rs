mod api;
mod app;
mod components;
mod entities;
mod systems;

use crate::api::display::Point;
use app::App;

use entities::{Controller, Player};

fn main() {
    let dimensions = &Point {
        width: 60,
        height: 40,
    };

    let mut player = Player::new();

    let mut app = App::new(dimensions).unwrap();

    app.run(move |game_state, display_controller, drawable_controller| {
        if let Some(event) = &game_state.keyboard_event {
            player.handle_event(&event);
        }

        // Game logic
        // drawable_controller.add_drawable_entity(&mut player);

        Ok(())
    })
    .unwrap()
}
