
use itertools::Itertools;
use std::fmt;
use std::collections::HashMap;

pub fn solve(input:String) -> (usize, String) {
    solve_with_dimensions(input, 25, 6)
} // 14.96ms

pub fn solve_with_dimensions(input: String, width: usize, height: usize) -> (usize, String) {
    let layers: Vec<Layer> = input
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as u8)
        .chunks(width * height)
        .into_iter()
        .map(|layer| Layer::new(layer.collect::<Vec<u8>>(), width))
        .collect();
    let counts = layers
        .iter()
        .map(|layer| {
            let mut num_map: HashMap<u8, usize> = HashMap::new();
            for &d in &layer.data {
                *num_map.entry(d).or_insert(0) += 1;
            }
            num_map
        })
        .min_by_key(|num_map| *num_map.get(&0).unwrap_or(&0))
        .unwrap();
    let p1 = counts.get(&1).unwrap_or(&0) * counts.get(&2).unwrap_or(&0);
    let p2 = layers
        .iter()
        .skip(1)
        .fold(layers[0].clone(), |stacked, layer| stacked.stack(layer))
        .to_string();
    (p1, p2)
}

#[derive(Clone)]
struct Layer {
    data: Vec<u8>,
    width: usize,
}

impl Layer {
    fn new(data: Vec<u8>, width: usize) -> Self {
        Layer { data, width }
    }

    fn stack(&self, other: &Layer) -> Self {
        let data: Vec<u8> = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| if a == 2 { b } else { a })
            .collect();
        Layer::new(data, self.width)
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::new();
        self.data
            .chunks(self.width)
            .for_each(|row| {
                result.push_str(
                    row.iter()
                        .map(|digit| format!("{}", digit))
                        .collect::<String>()
                        .as_str()
                );
                result.push_str("\n");
            });
        write!(f, "{}", result.trim())
    }
}

#[test]
fn example1() {
    let input = String::from("123456789012");
    assert_eq!(solve_with_dimensions(input, 3, 2).0, 1);
    let input = String::from("0222112222120000");
    assert_eq!(solve_with_dimensions(input, 2, 2).0, 4);
}