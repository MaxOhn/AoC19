use crate::solution::Solution;
use crate::util::{gcd, Point};

pub fn solve(input: String) -> Solution<usize, i32> {
    let asteroids: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let (p1, station) = solve_part1(&asteroids);
    let p2 = solve_part2(&asteroids, station, 200);
    Solution::new(p1, p2)
}

fn solve_part1(asteroids: &Vec<Vec<bool>>) -> (usize, Point<usize>) {
    let w = asteroids[0].len();
    let h = asteroids.len();
    let mut station = Point::default();
    let mut result = 0;
    for y in 0..h {
        for x in 0..w {
            if asteroids[y][x] {
                let mut sights = 0;
                for cy in 0..h {
                    for cx in 0..w {
                        let delta = Point::new(cx as i32 - x as i32, cy as i32 - y as i32);
                        if gcd(delta.x.abs(), delta.y.abs()) == 1 {
                            let mut curr = Point::new(cx as i32, cy as i32);
                            while curr.in_bounds(0, 0, w as i32 - 1, h as i32 - 1) {
                                if asteroids[curr.y as usize][curr.x as usize] {
                                    sights += 1;
                                    break;
                                }
                                curr += delta;
                            }
                        }
                    }
                }
                if sights > result {
                    result = sights;
                    station = Point::new(x, y);
                }
            }
        }
    }
    (result, station)
}

fn solve_part2(asteroids: &Vec<Vec<bool>>, station: Point<usize>, destroy_num: usize) -> i32 {
    let w = asteroids[0].len();
    let h = asteroids.len();
    assert_eq!(w, h);
    // fractions (coordinates) from 0 to 1 with max denumerator max(station.x, station.y)
    let mut coords = farey(station.x.max(station.y) as i32);
    // extend by reciprocals (bottom right corner)
    coords.extend(
        coords
            .clone()
            .into_iter()
            .map(|p| Point::new(p.y, p.x))
            .rev()
            .skip(1),
    );
    // extent by negated denumerators (bottom left corner)
    coords.extend(
        coords
            .clone()
            .into_iter()
            .map(|p| Point::new(p.x, -p.y))
            .rev()
            .skip(1),
    );
    // extent by negated numerators (top left corner)
    coords.extend(
        coords
            .clone()
            .into_iter()
            .map(|p| Point::new(-p.x, p.y))
            .rev()
            .skip(1),
    );
    coords.pop();
    // matrix-coordinates have flipped y values
    coords = coords.into_iter().map(|p| Point::new(p.x, -p.y)).collect();
    let station = Point::new(station.x as i32, station.y as i32);
    let mut destroy_order: Vec<Vec<Point<i32>>> = Vec::with_capacity(w.max(h));
    for &delta in &coords {
        let mut curr = station + delta;
        let mut rotation = 0;
        while curr.in_bounds(0, 0, w as i32 - 1, h as i32 - 1) {
            if asteroids[curr.y as usize][curr.x as usize] {
                if destroy_order.len() <= rotation {
                    destroy_order.push(Vec::new());
                }
                destroy_order[rotation].push(curr);
                rotation += 1;
            }
            curr += delta;
        }
    }
    let result = *destroy_order
        .iter()
        .flatten()
        .take(destroy_num)
        .last()
        .unwrap();
    result.x * 100 + result.y
}

// generate fractions from 0 to 1 with max denumerator n
fn farey(n: i32) -> Vec<Point<i32>> {
    let mut ab = Point::new(0, 1);
    let mut cd = Point::new(1, n);
    let mut sequence = vec![ab];
    while cd.x < n {
        let k = (n + ab.y) / cd.y;
        let old_cd = cd;
        cd = Point::new(k * cd.x - ab.x, k * cd.y - ab.y);
        ab = old_cd;
        sequence.push(ab);
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test10() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let asteroids: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(solve_part1(&asteroids), (33, Point::new(5, 8)));
        let input = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let asteroids: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(solve_part1(&asteroids), (41, Point::new(6, 3)));
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        assert_eq!(solve(input.to_string()), Solution::new(210, 802));
        crate::util::tests::test_full_problem(10, solve, 344, 2732);
    }
}
