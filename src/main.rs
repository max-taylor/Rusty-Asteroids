mod api;
mod app;
mod components;
mod entities;
mod systems;

use crate::api::display::Point;
use app::App;
use crossterm::terminal::size;

fn get_screen_size() -> Point<i64> {
    let (rows, columns) = size().unwrap();

    Point::new(rows as i64, columns as i64)
}

fn main() {
    // let dimensions: &Point<i64> = &Point {
    //     width: 60,
    //     height: 40,
    // };

    let mut app = App::new(&get_screen_size()).unwrap();

    let val = app.run();
}
