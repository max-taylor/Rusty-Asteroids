mod api;
mod app;
mod components;
mod entities;
mod helpers;
mod systems;

use app::App;

// Run tests with logging: cargo test -- --nocapture
fn main() {
    // let dimensions: &Point<i64> = &Point {
    //     width: 60,
    //     height: 40,
    // };

    let mut app = App::new().unwrap();

    let val = app.run();
    dbg!(val);
}
