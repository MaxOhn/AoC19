use crate::solution::Solution;

use std::collections::{HashMap, HashSet, VecDeque};

type Relations = HashMap<usize, Vec<usize>>;

pub fn solve(input: String) -> Solution<usize, usize> {
    let (ids, directed, undirected) = prepare_maps(input);
    let p1 = solve_part1(&directed, *ids.get("COM").unwrap());
    let p2 = solve_part2(
        &undirected,
        *ids.get("YOU").unwrap(),
        *ids.get("SAN").unwrap(),
    )
    .unwrap();
    Solution::new(p1, p2)
}

fn prepare_maps(input: String) -> (HashMap<String, usize>, Relations, Relations) {
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut directed: Relations = HashMap::new();
    let mut undirected: Relations = HashMap::new();
    let mut id = 0;
    for line in input.lines() {
        let mut line_iter = line.split(")");
        let center = *ids
            .entry(String::from(line_iter.next().unwrap()))
            .or_insert_with(|| {
                id += 1;
                id
            });
        let orbiter = *ids
            .entry(String::from(line_iter.next().unwrap()))
            .or_insert_with(|| {
                id += 1;
                id
            });
        directed
            .entry(center)
            .or_insert_with(|| Vec::new())
            .push(orbiter);
        undirected
            .entry(center)
            .or_insert_with(|| Vec::new())
            .push(orbiter);
        undirected
            .entry(orbiter)
            .or_insert_with(|| Vec::new())
            .push(center);
    }
    (ids, directed, undirected)
}

fn solve_part1(map: &Relations, start: usize) -> usize {
    let mut sum = 0;
    let mut depths = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    depths.insert(start, 0);
    queue.push_back(start);
    visited.insert(start);

    while let Some(ref center) = queue.pop_front() {
        if let Some(orbiters) = map.get(center) {
            for &orbiter in orbiters {
                if !visited.contains(&orbiter) {
                    let depth = depths.get(center).unwrap() + 1;
                    depths.insert(orbiter, depth);
                    visited.insert(orbiter);
                    sum += depth;
                    queue.push_back(orbiter);
                }
            }
        }
    }
    sum
}

fn solve_part2(map: &Relations, start: usize, end: usize) -> Option<usize> {
    let mut depths = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    depths.insert(start, 0usize);
    queue.push_back(start);
    visited.insert(start);

    while let Some(ref center) = queue.pop_front() {
        if let Some(orbiters) = map.get(center) {
            for &orbiter in orbiters {
                if !visited.contains(&orbiter) {
                    let depth = depths.get(center).unwrap() + 1;
                    if orbiter == end {
                        return Some(depth - 2);
                    }
                    visited.insert(orbiter);
                    depths.insert(orbiter, depth);
                    queue.push_back(orbiter);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test06() {
        let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let (ids, map, _) = prepare_maps(input);
        assert_eq!(solve_part1(&map, *ids.get("COM").unwrap()), 42);
        let input =
            String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
        assert_eq!(solve(input), Solution::new(54, 4));
        crate::util::tests::test_full_problem(6, solve, 453028, 562);
    }
}
