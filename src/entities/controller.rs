use crossterm::event::{Event, KeyCode};

pub fn create_event(keycode: KeyCode) -> Event {
    Event::Key(keycode.into())
}

pub trait Controller {
    fn up(&mut self) -> &mut Self;
    fn down(&mut self) -> &mut Self;
    fn left(&mut self) -> &mut Self;
    fn right(&mut self) -> &mut Self;

    fn additional_event_logic(&mut self, event: &Event) -> &mut Self;

    fn handle_event(&mut self, event: &Event) {
        if event == &create_event(KeyCode::Up) {
            self.up();
        } else if event == &create_event(KeyCode::Down) {
            self.down();
        } else if event == &create_event(KeyCode::Left) {
            self.left();
        } else if event == &create_event(KeyCode::Right) {
            self.right();
        } else {
            self.additional_event_logic(event);
        }
    }
}
