mod app;
mod components;
mod entities;
mod systems;

use app::App;
use systems::display::Point;

fn main() {
    App::new(Point::new(30, 30)).unwrap()
}
