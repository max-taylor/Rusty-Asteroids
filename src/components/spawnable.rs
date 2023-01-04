use crate::api::display::Element;

trait Spawnable {
    fn spawn(&mut self, item: Element) -> &mut Self;
}
