use crate::computer::{Channel, Computer};
use crate::solution::Solution;
use itertools::Itertools;
use std::io::{self, BufRead};

pub fn solve(input: String) -> Solution<usize, i64> {
    let mut program: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let mut computer = Computer::new(program.clone());
    computer.set_output_channel(Channel::new(3000)).run();
    let mut grid = vec![vec![0; 42]; 20];
    loop {
        if let Some(x) = computer.try_pop() {
            let y = computer.pop();
            let tile = computer.pop();
            grid[y as usize][x as usize] = tile;
        } else {
            break;
        }
    }
    let p1: usize = grid
        .iter()
        .map(|row| {
            row.iter()
                .filter(|elem| **elem == 2)
                .collect::<Vec<_>>()
                .len()
        })
        .sum();
    program[0] = 2;
    let mut computer = Computer::new(program);
    let mut ready_to_play = false;
    let mut p2 = 0;
    let mut ball;
    let mut paddle = 0;
    let manual = false;
    computer
        .set_output_channel(Channel::new(3000))
        .insert(-1)
        .run();
    loop {
        if let Some(x) = computer.try_pop() {
            let y = computer.pop();
            let tile = computer.pop();
            if x == -1 && y == 0 {
                ready_to_play = true;
                p2 = tile;
            } else {
                grid[y as usize][x as usize] = tile;
                if tile == 3 {
                    paddle = x;
                } else if tile == 4 {
                    ball = x;
                    if ready_to_play {
                        if manual {
                            computer.insert(read_stdin(&grid)).run();
                        } else {
                            computer.insert((ball - paddle).min(1).max(-1)).run();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    if manual {
        println!("Game Over");
    }
    Solution::new(p1, p2)
}

fn read_stdin(grid: &[Vec<i64>]) -> i64 {
    println!(
        "{}",
        grid.iter()
            .map(|row| row
                .iter()
                .map(|elem| match elem {
                    0 => ' ',
                    1 => '█',
                    2 => 'X',
                    3 => '-',
                    4 => '•',
                    _ => unreachable!(),
                })
                .join(""))
            .join("\n")
    );
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
