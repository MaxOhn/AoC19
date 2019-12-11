use crate::solution::Solution;

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

pub fn solve(input: String) -> Solution<usize, String> {
    let (p1, p2) = solve_with_dimensions(input, 25, 6);
    Solution::new(p1, p2)
} // 14.96ms

pub fn solve_with_dimensions(input: String, width: usize, height: usize) -> (usize, String) {
    let layers: Vec<Layer> = input
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as usize)
        .chunks(width * height)
        .into_iter()
        .map(|layer| Layer::new(layer.collect::<Vec<usize>>(), width))
        .collect();
    let p1 = layers
        .iter()
        .map(|layer| layer.digit_count())
        .min_by_key(|count| *count.get(&0).unwrap_or(&0))
        .unwrap()
        .iter()
        .filter(|(&digit, _)| digit != 0)
        .map(|(_, &count)| count)
        .product();
    let p2 = layers
        .iter()
        .skip(1)
        .fold(layers[0].clone(), |stacked, layer| stacked.stack(layer))
        .to_string();
    (p1, p2)
}

#[derive(Clone)]
struct Layer {
    data: Vec<usize>,
    width: usize,
}

impl Layer {
    fn new(data: Vec<usize>, width: usize) -> Self {
        Layer { data, width }
    }

    fn stack(&self, other: &Layer) -> Self {
        let data: Vec<usize> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| if a == 2 { b } else { a })
            .collect();
        Layer::new(data, self.width)
    }

    fn digit_count(&self) -> HashMap<usize, usize> {
        let mut count: HashMap<usize, usize> = HashMap::new();
        for &d in &self.data {
            *count.entry(d).or_insert(0) += 1;
        }
        count
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .chunks(self.width)
                .map(|row| row
                    .iter()
                    .map(|digit| match digit {
                        0 => '\u{2592}',
                        1 => '\u{2593}',
                        2 => '\u{2591}',
                        _ => 'X',
                    })
                    .collect::<String>())
                .join("\n")
        )
    }
}

#[test]
fn example1() {
    let input = String::from("123456789012");
    assert_eq!(solve_with_dimensions(input, 3, 2).0, 1);
}

#[test]
fn example2() {
    let input = String::from("0222112222120000");
    assert_eq!(
        solve_with_dimensions(input, 2, 2),
        (4, String::from("\u{2592}\u{2593}\n\u{2593}\u{2592}"))
    );
}
