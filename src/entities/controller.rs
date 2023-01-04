use crossterm::event::Event;

pub trait Controller {
    fn up(&mut self) -> &mut Self;
    fn down(&mut self) -> &mut Self;
    fn left(&mut self) -> &mut Self;
    fn right(&mut self) -> &mut Self;

    fn handle_event(&mut self, event: Event) -> &mut Self;
}
