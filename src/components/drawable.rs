use crate::api::display::{Layout, Point};

#[derive(Debug)]
pub struct Velocity {
    width: i32,
    height: i32,
}

impl Velocity {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct DrawableState {
    pub map: Layout,
    pub location: Point,
    /// velocity in x-y (width, height)
    pub velocity: Velocity,
}

impl DrawableState {
    pub fn new(map: Layout, location: Point) -> Self {
        Self {
            map,
            location,
            velocity: Default::default(),
        }
    }
}

pub trait Drawable {
    fn get_drawable_state(&self) -> &DrawableState;
}
