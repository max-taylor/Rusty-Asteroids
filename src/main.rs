mod app;
mod display;

use app::App;

use std::io::stdout;

fn main() {
    let mut stdout = stdout();

    App::new(&mut stdout).unwrap()
}
