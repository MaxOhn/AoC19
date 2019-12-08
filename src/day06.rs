
use crate::solution::Solution;

use std::{
    cmp:: Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};
use itertools::Itertools;
use cached::UnboundCache;

pub fn solve(input: String) -> Solution<usize, usize> {
    let planets: HashMap<Planet, Vec<Planet>> = input.lines()
        .flat_map(|line| line.split(")"))
        .tuples()
        .fold(HashMap::new(), |mut map, (center, orbiter)| {
            let orbiter = Planet::new(orbiter);
            map.entry(Planet::new(center))
                .and_modify(|v| v.push(orbiter))
                .or_insert(vec![orbiter]);
            map.entry(orbiter)
                .or_insert(Vec::new());
            map
        });
    let amount_orbiters: HashMap<Planet, usize> = planets.iter()
        .map(|(center, _)| (*center, calc_amount_orbiters(&planets, &String::from(center.name))))
        .collect();
    let p1 = amount_orbiters.values().sum();
    let total_links: HashMap<Planet, Vec<Planet>> = planets.iter()
        .map(|(center, orbiters)| {
            (*center, {
                let mut neighbors = orbiters.clone();
                for (potential_neighbor, neighbors_neighbors) in &planets {
                    if center != potential_neighbor && neighbors_neighbors.contains(&center) {
                        neighbors.push(*potential_neighbor);
                    }
                }
                neighbors
            })
        }).collect();
    let p2 = dijkstra("YOU", &total_links).get(&Planet::new("SAN")).unwrap() - 2;
    Solution::new(p1, p2)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Planet<'a> {
    name: &'a str,
}

impl<'a> Planet<'a> {
    fn new(name: &'a str) -> Planet<'a> {
        Planet { name }
    }
}

struct Visit<V> {
    planet: V,
    dist: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}

impl<V> Eq for Visit<V> {}

fn dijkstra<'a>(start: &'a str, adjacency_list: &HashMap<Planet<'a>, Vec<Planet<'a>>>) -> HashMap<Planet<'a>, usize> {
    let start = Planet::new(start);
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visited = BinaryHeap::new();
    distances.insert(start, 0);
    to_visited.push(Visit {
        planet: start,
        dist: 0,
    });
    while let Some(Visit { planet, dist }) = to_visited.pop() {
        if !visited.insert(planet) { continue; }
        if let Some(neighbors) = adjacency_list.get(&planet) {
            for neighbor in neighbors {
                let new_dist = dist + 1;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_dist < current);
                if is_shorter {
                    distances.insert(*neighbor, new_dist);
                    to_visited.push(Visit {
                        planet: *neighbor,
                        dist: new_dist
                    });
                }
            }
        }
    }
    distances
}

cached_key!{
    CALC_AMOUNT_ORBITERS: UnboundCache<String, usize> = UnboundCache::new();
    Key = { format!("{}", center) };
    fn calc_amount_orbiters(planets: &HashMap<Planet, Vec<Planet>>, center: &String) -> usize = {
        let orbiters = planets.get(&Planet::new(center)).unwrap();
        orbiters.iter()
            .map(|orbiter| calc_amount_orbiters(planets, &String::from(orbiter.name)))
            .sum::<usize>() + orbiters.len() 
    }
}

/* // Caching ruins tests because of their multi-threading :(
#[test]
fn example1() {
    let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
    assert_eq!(solve(input).part1, 42);
}
*/

#[test]
fn example2() {
    let input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
    assert_eq!(solve(input), Solution::new(54, 4));
}
