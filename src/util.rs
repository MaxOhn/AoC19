#![allow(unused)]

use num::traits::{
    identities::{One, Zero},
    sign::{Signed, Unsigned},
    Num,
};
use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

#[cfg(test)]
pub mod tests {
    use crate::solution::Solution;
    use std::fmt::Debug;
    use std::fs;

    pub fn test_full_problem<F, U, V>(day: usize, solve_function: F, part1: U, part2: V)
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

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Num + Copy,
{
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Num + Copy,
{
    a * b / gcd(a, b)
}

pub type Point2i = Point2<i32>;
pub type Point2us = Point2<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T>
where
    T: Add<Output = T> + Ord + PartialOrd + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }

    pub fn sum(&self) -> T {
        self.x + self.y
    }

    pub fn in_bounds(&self, low_x: T, low_y: T, high_x: T, high_y: T) -> bool {
        low_x <= self.x && self.x <= high_x && low_y <= self.y && self.y <= high_y
    }

    pub fn restrict(mut self, min: T, max: T) -> Self {
        self.x = self.x.min(max).max(min);
        self.y = self.y.min(max).max(min);
        self
    }
}

impl<T> Ord for Point2<T>
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

impl<T> PartialOrd for Point2<T>
where
    T: Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Display for Point2<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

macro_rules! impl_op {
    ($struct:ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait for $struct<T>
        where
            T: $trait<Output = T>,
        {
            type Output = Self;
            fn $fn(self, other: Self) -> Self {
                $struct {
                    x: T::$fn(self.x, other.x),
                    y: T::$fn(self.y, other.y),
                }
            }
        }

        impl<T> $assign_trait for $struct<T>
        where
            T: $assign_trait,
        {
            fn $assign_fn(&mut self, other: Self) {
                T::$assign_fn(&mut self.x, other.x);
                T::$assign_fn(&mut self.y, other.y);
            }
        }
    };
}

macro_rules! impl_scalar {
    ($struct: ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait<T> for $struct<T>
        where
            T: $trait<Output = T> + Clone,
        {
            type Output = Self;
            fn $fn(self, other: T) -> Self {
                $struct {
                    x: T::$fn(self.x, other.clone()),
                    y: T::$fn(self.y, other),
                }
            }
        }

        impl<T> $assign_trait<T> for $struct<T>
        where
            T: $assign_trait + Clone,
        {
            fn $assign_fn(&mut self, other: T) {
                T::$assign_fn(&mut self.x, other.clone());
                T::$assign_fn(&mut self.y, other);
            }
        }
    };
}

impl_op!(Point2, Add, add, AddAssign, add_assign);
impl_op!(Point2, Sub, sub, SubAssign, sub_assign);
impl_op!(Point2, Mul, mul, MulAssign, mul_assign);
impl_op!(Point2, Div, div, DivAssign, div_assign);
impl_op!(Point2, Rem, rem, RemAssign, rem_assign);

impl_scalar!(Point2, Mul, mul, MulAssign, mul_assign);
impl_scalar!(Point2, Div, div, DivAssign, div_assign);
impl_scalar!(Point2, Rem, rem, RemAssign, rem_assign);

impl<T> Neg for Point2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Point2 {
            x: T::neg(self.x),
            y: T::neg(self.y),
        }
    }
}

impl<T> Zero for Point2<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Point2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        T::is_zero(&self.x) && T::is_zero(&self.y)
    }
}

impl<T> One for Point2<T>
where
    T: One,
{
    fn one() -> Self {
        Point2 {
            x: T::one(),
            y: T::one(),
        }
    }
}

impl<T> Num for Point2<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let mut iter = s.split(',');
        if let Some(value) = iter.next() {
            let x = T::from_str_radix(value.trim(), radix)?;
            if let Some(value) = iter.next() {
                let y = T::from_str_radix(value.trim(), radix)?;
                if let Some(value) = iter.next() {
                    let z = T::from_str_radix(value.trim(), radix)?;
                    if let None = iter.next() {
                        return Ok(Point2 { x, y });
                    }
                }
            }
        }

        if let Err(e) = T::from_str_radix("", radix) {
            return Err(e);
        }
        unreachable!()
    }
}

impl<T> Signed for Point2<T>
where
    T: Signed + Clone,
{
    fn abs(&self) -> Self {
        Point2 {
            x: T::abs(&self.x),
            y: T::abs(&self.y),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let delta = self.clone() - other.clone();
        <Self as Signed>::abs(&delta)
    }

    fn signum(&self) -> Self {
        Point2 {
            x: T::signum(&self.x),
            y: T::signum(&self.y),
        }
    }

    fn is_positive(&self) -> bool {
        T::is_positive(&self.x) && T::is_positive(&self.y)
    }

    fn is_negative(&self) -> bool {
        T::is_negative(&self.x) && T::is_negative(&self.y)
    }
}

impl<T> Unsigned for Point2<T> where T: Unsigned {}

impl<T> FromStr for Point2<T>
where
    T: Num,
{
    type Err = <Self as Num>::FromStrRadixErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Num>::from_str_radix(s, 10)
    }
}

pub type Point3i = Point3<i32>;
pub type Point3us = Point3<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T>
where
    T: Add<Output = T> + Ord + PartialOrd + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3 { x, y, z }
    }

    pub fn sum(&self) -> T {
        self.x + self.y + self.z
    }

    pub fn in_bounds(&self, low_x: T, low_y: T, low_z: T, high_x: T, high_y: T, high_z: T) -> bool {
        low_x <= self.x
            && self.x <= high_x
            && low_y <= self.y
            && self.y <= high_y
            && low_z <= self.z
            && self.z <= high_z
    }

    pub fn restrict(mut self, min: T, max: T) -> Self {
        self.x = self.x.min(max).max(min);
        self.y = self.y.min(max).max(min);
        self.z = self.z.min(max).max(min);
        self
    }
}

impl<T> Ord for Point3<T>
where
    T: Ord + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.z != other.z {
            self.z.cmp(&other.z)
        } else if self.y != other.y {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl<T> PartialOrd for Point3<T>
where
    T: Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Display for Point3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

macro_rules! impl_op {
    ($struct:ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait for $struct<T>
        where
            T: $trait<Output = T>,
        {
            type Output = Self;
            fn $fn(self, other: Self) -> Self {
                $struct {
                    x: T::$fn(self.x, other.x),
                    y: T::$fn(self.y, other.y),
                    z: T::$fn(self.z, other.z),
                }
            }
        }

        impl<T> $assign_trait for $struct<T>
        where
            T: $assign_trait,
        {
            fn $assign_fn(&mut self, other: Self) {
                T::$assign_fn(&mut self.x, other.x);
                T::$assign_fn(&mut self.y, other.y);
                T::$assign_fn(&mut self.z, other.z);
            }
        }
    };
}

macro_rules! impl_scalar {
    ($struct: ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait<T> for $struct<T>
        where
            T: $trait<Output = T> + Clone,
        {
            type Output = Self;
            fn $fn(self, other: T) -> Self {
                $struct {
                    x: T::$fn(self.x, other.clone()),
                    y: T::$fn(self.y, other.clone()),
                    z: T::$fn(self.z, other),
                }
            }
        }

        impl<T> $assign_trait<T> for $struct<T>
        where
            T: $assign_trait + Clone,
        {
            fn $assign_fn(&mut self, other: T) {
                T::$assign_fn(&mut self.x, other.clone());
                T::$assign_fn(&mut self.y, other.clone());
                T::$assign_fn(&mut self.z, other);
            }
        }
    };
}

impl_op!(Point3, Add, add, AddAssign, add_assign);
impl_op!(Point3, Sub, sub, SubAssign, sub_assign);
impl_op!(Point3, Mul, mul, MulAssign, mul_assign);
impl_op!(Point3, Div, div, DivAssign, div_assign);
impl_op!(Point3, Rem, rem, RemAssign, rem_assign);

impl_scalar!(Point3, Mul, mul, MulAssign, mul_assign);
impl_scalar!(Point3, Div, div, DivAssign, div_assign);
impl_scalar!(Point3, Rem, rem, RemAssign, rem_assign);

impl<T> Neg for Point3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Point3 {
            x: T::neg(self.x),
            y: T::neg(self.y),
            z: T::neg(self.z),
        }
    }
}

impl<T> Zero for Point3<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Point3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        T::is_zero(&self.x) && T::is_zero(&self.y) && T::is_zero(&self.z)
    }
}

impl<T> One for Point3<T>
where
    T: One,
{
    fn one() -> Self {
        Point3 {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }
}

impl<T> Num for Point3<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let mut iter = s.split(',');
        if let Some(value) = iter.next() {
            let x = T::from_str_radix(value.trim(), radix)?;
            if let Some(value) = iter.next() {
                let y = T::from_str_radix(value.trim(), radix)?;
                if let Some(value) = iter.next() {
                    let z = T::from_str_radix(value.trim(), radix)?;
                    if let None = iter.next() {
                        return Ok(Point3 { x, y, z });
                    }
                }
            }
        }

        if let Err(e) = T::from_str_radix("", radix) {
            return Err(e);
        }
        unreachable!()
    }
}

impl<T> Signed for Point3<T>
where
    T: Signed + Clone,
{
    fn abs(&self) -> Self {
        Point3 {
            x: T::abs(&self.x),
            y: T::abs(&self.y),
            z: T::abs(&self.z),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let delta = self.clone() - other.clone();
        <Self as Signed>::abs(&delta)
    }

    fn signum(&self) -> Self {
        Point3 {
            x: T::signum(&self.x),
            y: T::signum(&self.y),
            z: T::signum(&self.z),
        }
    }

    fn is_positive(&self) -> bool {
        T::is_positive(&self.x) && T::is_positive(&self.y) && T::is_positive(&self.z)
    }

    fn is_negative(&self) -> bool {
        T::is_negative(&self.x) && T::is_negative(&self.y) && T::is_negative(&self.z)
    }
}

impl<T> Unsigned for Point3<T> where T: Unsigned {}

impl<T> FromStr for Point3<T>
where
    T: Num,
{
    type Err = <Self as Num>::FromStrRadixErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Num>::from_str_radix(s, 10)
    }
}
