use crate::{
    computer::{Channel, Computer},
    util::{GridMap, Point2i},
    Solution,
};
use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn solve(input: String) -> Solution<usize, i64> {
    let mut program: Vec<i64> = input.split(',').map(|n| n.parse().unwrap()).collect();
    let mut computer = Computer::new(program.clone());
    computer.set_output_channel(Channel::new(3000)).run();
    let mut grid = GridMap::new();
    while let Some(x) = computer.try_pop() {
        let y = computer.pop();
        let tile = computer.pop();
        grid.insert(Point2i::new(x as i32, y as i32), tile);
    }
    let p1 = grid.iter().filter(|(_, v)| **v == 2).count();
    let mut mapping = HashMap::new();
    mapping.insert(0, ' ');
    mapping.insert(1, '█');
    mapping.insert(2, 'X');
    mapping.insert(3, '-');
    mapping.insert(4, '•');
    program[0] = 2;
    let mut computer = Computer::new(program);
    let mut ready_to_play = false;
    let mut p2 = 0;
    let mut ball;
    let mut paddle = 0;
    const MANUAL: bool = false; // false -> let AI play; true -> play yourself
    computer
        .set_output_channel(Channel::new(3000))
        .insert(-1)
        .run();
    while let Some(x) = computer.try_pop() {
        let y = computer.pop();
        let tile = computer.pop();
        if x == -1 && y == 0 {
            ready_to_play = true;
            p2 = p2.max(tile);
        } else {
            grid.insert(Point2i::new(x as i32, y as i32), tile);
            if tile == 3 {
                paddle = x;
            } else if tile == 4 {
                ball = x;
                if ready_to_play {
                    if MANUAL {
                        computer
                            .insert(read_stdin(&grid.map_values(&mapping, Some(' '))))
                            .run();
                    } else {
                        computer.insert((ball - paddle).min(1).max(-1)).run();
                    }
                }
            }
        }
    }
    if MANUAL {
        println!("Score: {}", p2);
        println!("Game Over");
    }
    Solution::new(p1, p2)
} // 250.96ms

fn read_stdin(grid: &GridMap<char>) -> i64 {
    println!("{}", grid);
    println!("Next input:");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    // 1: Left, 2: Stay, 3: Right
    match line.trim().parse::<i64>() {
        Ok(val) if 0 < val && val < 4 => val - 2,
        _ => read_stdin(grid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test13() {
        crate::util::tests::test_full_problem(13, solve, 207, 10247);
    }
}
