extern crate crossbeam;

mod computer;
pub mod day01;
pub mod day01_2018;
pub mod day02;
pub mod day02_2018;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

pub use self::solution::Solution;

pub mod solution {
    use std::fmt;

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Solution<U, V> {
        pub part1: U,
        pub part2: V,
    }

    impl<U, V> Solution<U, V> {
        pub fn new(part1: U, part2: V) -> Self {
            Solution { part1, part2 }
        }
    }

    impl<U, V> fmt::Display for Solution<U, V>
    where
        U: fmt::Display,
        V: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Part 1:\n{}\nPart 2:\n{}", self.part1, self.part2)
        }
    }
}

pub mod util {
    #![allow(unused)]

    use num::traits::{
        identities::{One, Zero},
        sign::{Signed, Unsigned},
        Num,
    };
    use std::{
        cmp::Ordering,
        collections::{BTreeMap, HashMap, HashSet},
        fmt,
        iter::FromIterator,
        ops::{
            Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign,
            Sub, SubAssign,
        },
        str::FromStr,
    };

    #[cfg(test)]
    pub mod tests {
        use crate::solution::Solution;
        use std::fmt::Debug;
        use std::fs;

        /// Check whether the solution of a day still gives the correct answer
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

    /// Calculates the greatest common denominator of 2 numbers
    /// # Examples
    /// ```
    /// use aoc19::util::gcd;
    ///
    /// let n = gcd(6, 15);
    /// assert_eq!(n, 3);
    /// ```
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

    /// Calculates the least common multiple of 2 numbers
    /// # Examples
    /// ```
    /// use aoc19::util::lcm;
    ///
    /// let n = lcm(3, 4);
    /// assert_eq!(n, 12);
    /// ```
    pub fn lcm<T>(a: T, b: T) -> T
    where
        T: Num + Copy,
    {
        a * b / gcd(a, b)
    }

    /// Store a grid in a `HashMap<Point2i, T>`.
    /// This way negative coordinates are more easily handled than with a grid of `Vec<Vec<T>>`
    pub struct GridMap<T>(HashMap<Point2i, T>);

    impl<T> Deref for GridMap<T> {
        type Target = HashMap<Point2i, T>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> GridMap<T> {
        pub fn new() -> Self {
            GridMap(HashMap::new())
        }
    }

    impl<T> DerefMut for GridMap<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T> FromIterator<(Point2i, T)> for GridMap<T> {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = (Point2i, T)>,
        {
            GridMap(HashMap::from_iter(iter))
        }
    }

    /// Simple struct containing two i32 values, generally used as coordinate.
    pub type Point2i = Point2<i32>;
    /// Simple struct containing two usize values, generally used as coordinate.
    pub type Point2us = Point2<usize>;

    /// Simple struct containing two values of the same type, generally used as coordinate.
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

        /// Sum the x and y fields
        /// # Examples
        /// ```
        /// use aoc19::util::Point2;
        ///
        /// let p = Point2::new(-3, 4);
        /// assert_eq!(p.sum(), 1);
        /// ```
        pub fn sum(&self) -> T {
            self.x + self.y
        }

        /// Check whether the point is whithin the given bounds
        /// # Examples
        /// ```
        /// use aoc19::util::Point2;
        ///
        /// let p = Point2::new(4, 5);
        /// assert!(p.in_bounds(0, 0, 5, 5));
        /// assert!(!p.in_bounds(0, 0, 2, 8));
        /// ```
        pub fn in_bounds(&self, low_x: T, low_y: T, high_x: T, high_y: T) -> bool {
            low_x <= self.x && self.x <= high_x && low_y <= self.y && self.y <= high_y
        }

        /// Clamp x and y value inbetween a min and max
        /// # Examples
        /// ```
        /// use aoc19::util::Point2;
        ///
        /// let p = Point2::new(-2, 7).restrict(-4, 4);
        /// assert_eq!(p.x, -2);
        /// assert_eq!(p.y, 4);
        /// ```
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

    /// Simple struct containing three i32 values, generally used as coordinate.
    pub type Point3i = Point3<i32>;
    /// Simple struct containing three usize values, generally used as coordinate.
    pub type Point3us = Point3<usize>;

    /// Simple struct containing three values of the same type, generally used as coordinate.
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

        /// Sum the x, y, and z fields
        /// # Examples
        /// ```
        /// use aoc19::util::Point3;
        ///
        /// let p = Point3::new(-3, 4, 1);
        /// assert_eq!(p.sum(), 2);
        /// ```
        pub fn sum(&self) -> T {
            self.x + self.y + self.z
        }

        /// Check whether the point is whithin the given bounds
        /// # Examples
        /// ```
        /// use aoc19::util::Point3;
        ///
        /// let p = Point3::new(4, 5, -2);
        /// assert!(p.in_bounds(0, 0, -2, 5, 5, 3));
        /// assert!(!p.in_bounds(0, 0, 0, 2, 8, 8));
        /// ```
        pub fn in_bounds(
            &self,
            low_x: T,
            low_y: T,
            low_z: T,
            high_x: T,
            high_y: T,
            high_z: T,
        ) -> bool {
            low_x <= self.x
                && self.x <= high_x
                && low_y <= self.y
                && self.y <= high_y
                && low_z <= self.z
                && self.z <= high_z
        }

        /// Clamp x, y, and z values inbetween a min and max
        /// # Examples
        /// ```
        /// use aoc19::util::Point3;
        ///
        /// let p = Point3::new(-2, 7, 3).restrict(-4, 4);
        /// assert_eq!(p.x, -2);
        /// assert_eq!(p.y, 4);
        /// assert_eq!(p.z, 3);
        /// ```
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

    /// North, South, West, East enumeration
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
    pub enum Direction {
        N,
        S,
        W,
        E,
    }

    impl Direction {
        /// Going north decrements the y value, south increments y value, west decrements x and east increments x of a Point2i
        /// # Examples
        /// ```
        /// use aoc19::util::{Direction, Point2i};
        ///
        /// let d = Direction::W;
        /// let p = Point2i::new(-2, 3) + d.shift();
        /// assert_eq!(p.x, -3);
        /// assert_eq!(p.y, 3);
        /// ```
        pub fn shift(&self) -> Point2i {
            match self {
                Direction::N => Point2i::new(0, -1),
                Direction::W => Point2i::new(-1, 0),
                Direction::S => Point2i::new(0, 1),
                Direction::E => Point2i::new(1, 0),
            }
        }

        /// Iterate over all directions
        /// # Examples
        /// ```
        /// use aoc19::util::Direction;
        ///
        /// for dir in Direction::iter() {
        ///     // ...
        /// }
        /// ```
        pub fn iter() -> std::slice::Iter<'static, Self> {
            static VALS: [Direction; 4] = [Direction::N, Direction::W, Direction::S, Direction::E];
            VALS.into_iter()
        }
    }
}
