use crate::computer::Computer;
use crate::solution::Solution;
use crate::util::{Direction, GridMap, Point2i};

use std::collections::HashSet;

pub fn solve(input: String) -> Solution<i32, i32> {
    let program: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let mut computer = Computer::new(program);
    let mut curr_pos = Point2i::new(0, 0);
    let mut curr_dir = Direction::N;
    // 0: Wall; 1: Path; 2: Blocked; 3: Oxygenated
    let mut grid = GridMap::new();
    grid.insert(curr_pos, 1u8);
    let mut oxy_pos = Point2i::default();
    let mut p1 = 0;
    let p2;
    loop {
        match curr_dir {
            Direction::N => computer.insert(1),
            Direction::S => computer.insert(2),
            Direction::W => computer.insert(3),
            Direction::E => computer.insert(4),
        };
        let res = computer.run().pop();
        match res {
            0 => {
                grid.insert(curr_pos + curr_dir.shift(), 0);
            }
            1 => {
                curr_pos += curr_dir.shift();
                grid.insert(curr_pos, 1);
            }
            2 => {
                curr_pos += curr_dir.shift();
                oxy_pos = curr_pos;
                grid.insert(curr_pos, 1);
            }
            _ => unreachable!(),
        }
        let mut found = false;
        // Find unvisited neighbor
        for &dir in Direction::iter() {
            let next_pos = curr_pos + dir.shift();
            if !grid.contains_key(&next_pos) {
                curr_dir = dir;
                found = true;
                break;
            }
        }
        // Find unblocked neighbor
        if !found {
            grid.insert(curr_pos, 2);
            for &dir in Direction::iter() {
                let next_pos = curr_pos + dir.shift();
                if *grid.get(&next_pos).unwrap() == 1 {
                    curr_dir = dir;
                    found = true;
                    break;
                }
            }
        }
        // Grid fully discovered, start flooding
        if !found {
            // Part 1
            let pos = Point2i::new(0, 0);
            let mut visited = HashSet::new();
            visited.insert(pos);
            let mut backtrack = vec![(pos, 0)];
            while !backtrack.is_empty() {
                let (pos, dist) = backtrack.pop().unwrap();
                for dir in Direction::iter() {
                    let next_pos = pos + dir.shift();
                    if *grid.get(&next_pos).unwrap() != 0 {
                        if !visited.contains(&next_pos) {
                            if next_pos == oxy_pos {
                                p1 = dist + 1;
                                backtrack.clear();
                                break;
                            } else {
                                visited.insert(next_pos);
                                backtrack.push((next_pos, dist + 1));
                            }
                        }
                    }
                }
            }
            // Part 2
            visited.clear();
            visited.insert(oxy_pos);
            let mut backtrack = vec![(oxy_pos, 0)];
            let mut minutes = 0;
            while !backtrack.is_empty() {
                let (pos, m) = backtrack.pop().unwrap();
                minutes = minutes.max(m);
                grid.insert(pos, 3);
                for dir in Direction::iter() {
                    let next_pos = pos + dir.shift();
                    if *grid.get(&next_pos).unwrap() != 0 {
                        if !visited.contains(&next_pos) {
                            visited.insert(next_pos);
                            backtrack.push((next_pos, m + 1));
                        }
                    }
                }
            }
            p2 = minutes;
            break;
        }
    }
    Solution::new(p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test15() {
        crate::util::tests::test_full_problem(15, solve, 272, 398);
    }
}
