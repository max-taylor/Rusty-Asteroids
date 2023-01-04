mod api;
mod app;
mod components;
mod entities;
mod helpers;
mod systems;

use app::App;

fn main() {
    // let dimensions: &Point<i64> = &Point {
    //     width: 60,
    //     height: 40,
    // };

    let mut app = App::new(None).unwrap();

    let val = app.run();
    dbg!(val);
}
