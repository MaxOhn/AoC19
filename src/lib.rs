
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
        where U: fmt::Display, V: fmt::Display
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Part 1:\n{}\nPart 2:\n{}", self.part1, self.part2)
        }
    }
}
