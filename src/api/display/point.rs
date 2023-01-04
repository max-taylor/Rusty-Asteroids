use std::ops::{Add, Div, Mul, Sub};

use crossterm::cursor::MoveTo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub width: u32,
    pub height: u32,
}

impl From<&Point> for MoveTo {
    fn from(point: &Point) -> Self {
        Self(
            point.width.try_into().unwrap(),
            point.height.try_into().unwrap(),
        )
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl Point {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn add_width(mut self, width: u32) -> Self {
        self.width += width;

        self
    }

    pub fn sub_width(mut self, width: u32) -> Self {
        self.width -= width;

        self
    }

    pub fn add_height(mut self, height: u32) -> Self {
        self.height += height;

        self
    }

    pub fn sub_height(mut self, height: u32) -> Self {
        self.height -= height;

        self
    }

    pub const fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}
