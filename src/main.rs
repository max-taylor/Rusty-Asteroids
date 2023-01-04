mod api;
mod app;
mod components;
mod entities;
mod systems;

use crate::api::display::Point;
use app::App;

use entities::{Controller, Player};

fn main() {
    let dimensions: &Point<u32> = &Point {
        width: 60,
        height: 40,
    };

    let mut player = Player::new();

    let mut app = App::new(dimensions).unwrap();

    app.run(|game_state, display_controller| {
        if let Some(event) = &game_state.keyboard_event {
            player.handle_event(&event);
        }

        // Game logic
        display_controller.draw_drawable(&mut player)?;

        Ok(())
    })
    .unwrap()
}
