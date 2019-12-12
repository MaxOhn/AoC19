use crate::computer::Computer;
use crate::solution::Solution;
use crate::util::Point2;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: String) -> Solution<usize, String> {
    let program: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();
    // Directional maps setup
    let mut turn_left = HashMap::new();
    turn_left.insert(Direction::Up, Direction::Left);
    turn_left.insert(Direction::Left, Direction::Down);
    turn_left.insert(Direction::Down, Direction::Right);
    turn_left.insert(Direction::Right, Direction::Up);
    let mut turn_right = HashMap::new();
    turn_right.insert(Direction::Up, Direction::Right);
    turn_right.insert(Direction::Left, Direction::Up);
    turn_right.insert(Direction::Down, Direction::Left);
    turn_right.insert(Direction::Right, Direction::Down);
    const SIZE: usize = 150; // 150x150 appears to be sufficiently large
    let mut grid: Vec<Vec<i64>> = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        grid.push(vec![0; SIZE]);
    }
    let mut p1_grid = grid.iter().cloned().collect();
    let p1 = run(program.clone(), &mut p1_grid, &turn_left, &turn_right);
    grid[SIZE / 2][SIZE / 2] = 1;
    run(program, &mut grid, &turn_left, &turn_right);
    let mut first = usize::max_value();
    let mut last = 0;
    for row in &grid {
        let mut curr_first = usize::max_value();
        let mut curr_last = 0;
        for i in 0..row.len() {
            if row[i] == 1 {
                curr_first = curr_first.min(i);
                curr_last = curr_last.max(i);
            }
        }
        first = first.min(curr_first);
        last = last.max(curr_last);
    }
    let p2 = grid
        .iter()
        .filter(|row| row.iter().any(|&num| num != 0))
        .map(|row| {
            row[first..=last]
                .to_vec()
                .iter()
                .map(|num| match num {
                    0 => ' ',
                    1 => '\u{2588}',
                    _ => unreachable!(),
                })
                .join("")
        })
        .join("\n");
    Solution::new(p1, p2)
}

/* // Solve by storing the grid in a HashMap instead of a matrix
pub fn solve(input: String) -> Solution<usize, String> {
    let program: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();
    // Directional maps setup
    let mut turn_left = HashMap::new();
    turn_left.insert(Direction::Up, Direction::Left);
    turn_left.insert(Direction::Left, Direction::Down);
    turn_left.insert(Direction::Down, Direction::Right);
    turn_left.insert(Direction::Right, Direction::Up);
    let mut turn_right = HashMap::new();
    turn_right.insert(Direction::Up, Direction::Right);
    turn_right.insert(Direction::Left, Direction::Up);
    turn_right.insert(Direction::Down, Direction::Left);
    turn_right.insert(Direction::Right, Direction::Down);
    let p1 = run(0, program.clone(), &turn_left, &turn_right).len();
    let p2_grid = run(1, program, &turn_left, &turn_right);
    let msg_pixels = p2_grid
        .iter()
        .filter(|(_, &val)| val != 0)
        .map(|(&pos, _)| pos)
        .collect::<Vec<_>>();
    let max_x = msg_pixels.iter().map(|pos| pos.x).max().unwrap();
    let max_y = msg_pixels.iter().last().unwrap().y;
    let mut msg_grid = Vec::new();
    for _ in 0..=max_y {
        msg_grid.push(vec!['\u{2591}'; max_x as usize + 1]);
    }
    for Point { x, y } in msg_pixels {
        msg_grid[y as usize][x as usize] = '\u{2593}';
    }
    let p2 = msg_grid.iter().map(|row| row.iter().join("")).join("\n");
    Solution::new(p1, p2)
}
*/

fn run(
    program: Vec<i64>,
    grid: &mut Vec<Vec<i64>>,
    turn_left: &HashMap<Direction, Direction>,
    turn_right: &HashMap<Direction, Direction>,
) -> usize {
    let mut brain = Computer::new(program);
    let mut pos = Point2::new(grid.len() / 2, grid.len() / 2);
    let mut dir = &Direction::Up;
    let mut visited = HashSet::new();
    loop {
        visited.insert(pos);
        brain.insert(grid[pos.y][pos.x]).run();
        match brain.try_pop() {
            Some(output) => grid[pos.y][pos.x] = output,
            None => return visited.len(),
        };
        dir = match brain.pop() {
            0 => turn_left.get(dir).unwrap(),
            1 => turn_right.get(dir).unwrap(),
            _ => panic!("Unexpected output for direction"),
        };
        match dir {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        crate::util::tests::test_full_problem(11, solve, 2319, String::from("█  █ ████ ███  ███  ███  ████  ██    ██\n█  █ █    █  █ █  █ █  █ █    █  █    █\n█  █ ███  █  █ █  █ █  █ ███  █       █\n█  █ █    ███  ███  ███  █    █ ██    █\n█  █ █    █ █  █    █ █  █    █  █ █  █\n ██  ████ █  █ █    █  █ █     ███  ██ "));
    }
}
