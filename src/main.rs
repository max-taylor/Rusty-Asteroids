mod app;
mod components;
mod display;
mod entities;

use app::App;
use display::Point;

fn main() {
    App::new(Point::new(30, 30)).unwrap()
}
