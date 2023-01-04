use crossterm::event::{Event, KeyCode};

fn create_event(keycode: KeyCode) -> Event {
    Event::Key(keycode.into())
}

pub trait Controller {
    fn up(&mut self) -> &mut Self;
    fn down(&mut self) -> &mut Self;
    fn left(&mut self) -> &mut Self;
    fn right(&mut self) -> &mut Self;

    fn handle_event(&mut self, event: &Event) -> &mut Self {
        if event == &create_event(KeyCode::Up) {
            return self.up();
        } else if event == &create_event(KeyCode::Down) {
            return self.down();
        } else if event == &create_event(KeyCode::Left) {
            return self.left();
        } else if event == &create_event(KeyCode::Right) {
            return self.right();
        }

        self
    }
}
