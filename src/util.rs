#![allow(unused)]
use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(test)]
pub(crate) mod tests {
    use crate::solution::Solution;
    use std::fmt::Debug;
    use std::fs;

    pub(crate) fn test_full_problem<F, U, V>(day: usize, solve_function: F, part1: U, part2: V)
    where
        F: Fn(String) -> Solution<U, V>,
        U: Eq + Debug,
        V: Eq + Debug,
    {
        let input = fs::read_to_string(format!("inputs/day{:02}.txt", day)).unwrap();
        let solution = solve_function(input);
        assert_eq!(solution.part1, part1);
        assert_eq!(solution.part2, part2);
    }
}

pub(crate) fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Add<Output = T> + Ord + PartialOrd + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn sum(&self) -> T {
        self.x + self.y
    }

    pub fn in_bounds(&self, low_x: T, low_y: T, high_x: T, high_y: T) -> bool {
        low_x <= self.x && self.x <= high_x && low_y <= self.y && self.y <= high_y
    }
}

impl<T> Ord for Point<T>
where
    T: Ord + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y != other.y {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl<T> PartialOrd for Point<T>
where
    T: Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Point<T>;
    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T, U: Copy> Mul<U> for Point<T>
where
    T: Mul<U, Output = T>,
{
    type Output = Point<T>;
    fn mul(self, other: U) -> Point<T> {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T, U: Copy> Div<U> for Point<T>
where
    T: Div<U, Output = T>,
{
    type Output = Point<T>;
    fn div(self, other: U) -> Point<T> {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> AddAssign for Point<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl<T> SubAssign for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl<T, U> MulAssign<U> for Point<T>
where
    T: Mul<U, Output = T> + Copy,
    U: Copy,
{
    fn mul_assign(&mut self, other: U) {
        *self = *self * other
    }
}

impl<T, U> DivAssign<U> for Point<T>
where
    T: Div<U, Output = T> + Copy,
    U: Copy,
{
    fn div_assign(&mut self, other: U) {
        *self = *self / other
    }
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
