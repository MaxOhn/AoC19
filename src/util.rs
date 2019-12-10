
#![allow(unused)]
use std::{
    ops::{
        Add, Div, Mul, Sub,
        AddAssign, SubAssign,
        MulAssign, DivAssign,
    },
    fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn manhatten(&self) -> i32 {
        self.x + self.y
    }

    pub fn in_bounds(&self, low_x: i32, low_y: i32, high_x: i32, high_y: i32) -> bool {
        low_x <= self.x && self.x <= high_x && low_y <= self.y && self.y <= high_y 
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, other: i32) -> Point {
        Point { x: self.x * other, y: self.y * other }
    }
}

impl Div<i32> for Point {
    type Output = Point;
    fn div(self, other: i32) -> Point {
        Point { x: self.x / other, y: self.y / other }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other 
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = *self + other 
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other 
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, other: i32) {
        *self = *self / other 
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
