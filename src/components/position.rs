use crate::systems::display::Point;

pub struct Position2<T> {
    field: T,
}

pub trait Position {
    fn new(position: Point, value: char) -> Self;

    fn update_position(&mut self, new_position: Point) -> &mut Self;
}

struct GenericType<T> {
    field: T,
}

impl<T> GenericType<T> {
    fn new(value: T) -> Self {
        Self { field: value }
    }
}

pub fn test_func() {
    let test_val = GenericType::new(false);
}

trait GenericTrait<T> {
    fn new(value: T) -> Self;
}
