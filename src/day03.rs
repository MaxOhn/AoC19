
use std::fs;
use std::collections::HashMap;
use std::cmp;

struct Element {
    dir: char,
    len: i32
}

struct Wire {
    elements: Vec<Element>
}

impl Wire {
    fn new(line: &str) -> Self {
        Wire {
            elements: {
                line.split(',')
                    .map(|elem| elem.split_at(1))
                    .map(|(a, b)| Element { 
                        dir: a.parse::<char>().unwrap(), 
                        len: b.parse::<i32>().unwrap() 
                    }).collect()
            }
        }
    }
}

pub fn solve() -> (i32, i32) {
    let wires = fs::read_to_string("src/inputs/day03.txt")
        .unwrap()
        .lines()
        .map(|line| Wire::new(line))
        .collect::<Vec<Wire>>();
    let mut seen_positions: HashMap<(i32, i32), i32> = HashMap::new();
    follow_wire(&wires[0], &mut seen_positions, false);
    follow_wire(&wires[1], &mut seen_positions, true).unwrap()
} // 396.97ms

fn follow_wire(wire: &Wire, seen_positions: &mut HashMap<(i32, i32), i32>, output: bool) -> Option<(i32, i32)> {
    let (mut x, mut y, mut path) = (0, 0, 0);
    let mut closest_cross = i32::max_value();
    let mut shortest_cross = i32::max_value();
    for elem in &wire.elements {
        for _ in 1..elem.len + 1 {
            match &elem.dir {
                'U' => x -= 1,
                'D' => x += 1,
                'L' => y -= 1,
                'R' => y += 1,
                _ => panic!("Found non-directional char")
            }
            path += 1;
            if output {
                if seen_positions.contains_key(&(x, y)) {
                    closest_cross = cmp::min(closest_cross, x.abs() + y.abs());
                    shortest_cross = cmp::min(shortest_cross, path + seen_positions[&(x, y)]);
                }
            } else {
                seen_positions.entry((x, y)).or_insert(path);
            }
        }
    }
    if output {
        Some((closest_cross, shortest_cross))
    } else { 
        None 
    }
}