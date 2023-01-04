mod api;
mod app;
mod components;
mod entities;
mod systems;

use crate::api::display::Point;
use app::App;

fn main() {
    App::new(Point::new(100, 60)).unwrap()
}
