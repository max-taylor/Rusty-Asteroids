mod api;
mod app;
mod components;
mod entities;
mod helpers;
mod systems;
mod user_display;

use std::io::stdout;

use app::{App, AppManager};

use crate::api::display::{get_screen_size, Output};

// Run tests with logging: cargo test -- --nocapture
fn main() {
    let screen_size = get_screen_size();

    let mut app_manager = AppManager::new(screen_size).unwrap();

    app_manager.run();
}
