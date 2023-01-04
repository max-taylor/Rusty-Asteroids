use std::ops::{Add, Div, Mul, Sub};

use crossterm::cursor::MoveTo;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl From<&Point> for MoveTo {
    fn from(point: &Point) -> Self {
        Self(point.x.try_into().unwrap(), point.y.try_into().unwrap())
    }
}

impl Point {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn home_point() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn addX(mut self, x: u32) -> Self {
        self.x += x;

        self
    }

    pub fn addY(mut self, y: u32) -> Self {
        self.y += y;

        self
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
