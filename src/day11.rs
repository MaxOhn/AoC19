use crate::{
    computer::Computer,
    util::{Direction, GridMap, Point2},
    Solution,
};
use std::collections::HashMap;

pub fn solve(input: String) -> Solution<usize, String> {
    let program: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let mut grid = GridMap::new();
    run(0, program.clone(), &mut grid);
    let p1 = grid.len();
    grid.clear();
    run(1, program, &mut grid);
    let mut mapping = HashMap::new();
    mapping.insert(0, ' ');
    mapping.insert(1, '█');
    let p2 = grid
        .iter()
        .filter(|(_, v)| **v != 0)
        .map(|(&p, &v)| (p, v))
        .collect::<GridMap<i64>>()
        .map_values(&mapping, Some(' '))
        .to_string();
    Solution::new(p1, p2)
}

fn run(start: i64, program: Vec<i64>, grid: &mut GridMap<i64>) {
    let mut brain = Computer::new(program);
    let mut pos = Point2::new(0, 0);
    grid.insert(pos, start);
    let mut dir = Direction::N;
    loop {
        brain.insert(*grid.entry(pos).or_insert(0)).run();
        match brain.try_pop() {
            Some(output) => grid.insert(pos, output),
            None => break,
        };
        dir = match brain.pop() {
            0 => dir.to_left(),
            1 => dir.to_right(),
            _ => unreachable!(),
        };
        pos += dir.shift();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        crate::util::tests::test_full_problem(11, solve, 2319, String::from(
            "█  █ ████ ███  ███  ███  ████  ██    ██\n█  █ █    █  █ █  █ █  █ █    █  █    █\n█  █ ███  █  █ █  █ █  █ ███  █       █\n█  █ █    ███  ███  ███  █    █ ██    █\n█  █ █    █ █  █    █ █  █    █  █ █  █\n ██  ████ █  █ █    █  █ █     ███  ██ "
        ));
    }
}
