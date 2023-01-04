mod api;
mod app;
mod components;
mod entities;
mod game_state;
mod systems;

use crate::api::display::Point;
use app::App;

fn main() {
    let mut app = App::new(Point::new(60, 40)).unwrap();

    app.run(|loop_state, display| {
        // Game logic
    })
    .unwrap()
}
