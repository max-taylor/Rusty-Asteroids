mod api;
mod app;
mod components;
mod entities;
mod helpers;
mod systems;
mod user_display;

use app::App;

// Run tests with logging: cargo test -- --nocapture
fn main() {
    let mut app = App::new().unwrap();

    let val = app.run();
    dbg!("HERE");
    dbg!(val);
}
