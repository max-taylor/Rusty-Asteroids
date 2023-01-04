use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub width: T,
    pub height: T,
}

// impl<T> From<&Point<T>> for MoveTo
// where
//     u16: From<T>,
// {
//     fn from(point: &Point<T>) -> Self {
//         Self(
//             point.width.try_into().unwrap(),
//             point.height.try_into().unwrap(),
//         )
//     }
// }

impl Into<Point<i64>> for Point<u32> {
    fn into(self) -> Point<i64> {
        Point {
            height: self.height.into(),
            width: self.width.into(),
        }
    }
}

impl<T> Default for Point<T>
where
    T: AddAssign + SubAssign + Default,
{
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl<T> Point<T>
where
    T: AddAssign + SubAssign + Default,
{
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn add_width(mut self, width: T) -> Self {
        self.width += width;

        self
    }

    pub fn sub_width(mut self, width: T) -> Self {
        self.width -= width;

        self
    }

    pub fn add_height(mut self, height: T) -> Self {
        self.height += height;

        self
    }

    pub fn sub_height(mut self, height: T) -> Self {
        self.height -= height;

        self
    }

    pub fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Self::Output {
        Point {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Self::Output {
        Point {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T> Mul for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Point<T>;

    fn mul(self, other: Point<T>) -> Self::Output {
        Point {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl<T> Div for Point<T>
where
    T: Div<Output = T>,
{
    type Output = Point<T>;

    fn div(self, other: Point<T>) -> Self::Output {
        Point {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}
