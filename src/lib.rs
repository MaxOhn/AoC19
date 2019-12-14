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
mod util;

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
