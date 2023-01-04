mod api;
mod app;
mod components;
mod entities;
mod systems;

use crate::api::display::Point;
use app::App;
use crossterm::terminal::size;

fn get_screen_size() -> Point<u32> {
    let (rows, columns) = size().unwrap();

    Point::new(rows as u32, columns as u32)
}

fn main() {
    // let dimensions: &Point<u32> = &Point {
    //     width: 60,
    //     height: 40,
    // };

    let mut app = App::new(&get_screen_size()).unwrap();

    let val = app.run();
}
