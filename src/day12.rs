use crate::{
    util::{lcm, Point3i},
    Solution,
};

use num::Signed;

pub fn solve(input: String) -> Solution<i32, i64> {
    let mut moons = get_moons(input);
    let p1 = solve_part1(1000, &mut moons.iter().cloned().collect::<Vec<_>>());
    let p2 = solve_part2(&mut moons);
    Solution::new(p1, p2)
}

fn get_moons(input: String) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(", ")
                .map(|split| split.split("=").last().unwrap().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|vec| Moon::new(vec[0], vec[1], vec[2]))
        .collect::<Vec<_>>()
}

fn solve_part1(steps: usize, moons: &mut [Moon]) -> i32 {
    for _ in 0..steps {
        move_moons(moons);
    }
    moons.iter().map(|moon| moon.energy()).sum::<i32>()
}

fn solve_part2(moons: &mut [Moon]) -> i64 {
    let (mut steps, mut x_loop, mut y_loop, mut z_loop) = (0, 0, 0, 0);
    loop {
        steps += 1;
        move_moons(moons);
        if x_loop == 0 && moons.iter().all(|moon| moon.vel.x == 0) {
            x_loop = steps;
        }
        if y_loop == 0 && moons.iter().all(|moon| moon.vel.y == 0) {
            y_loop = steps;
        }
        if z_loop == 0 && moons.iter().all(|moon| moon.vel.z == 0) {
            z_loop = steps;
        }
        if x_loop > 0 && y_loop > 0 && z_loop > 0 {
            return 2 * lcm(lcm(x_loop, y_loop), z_loop);
        }
    }
}

fn move_moons(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            let diff = (moons[i].pos - moons[j].pos).restrict(-1, 1);
            moons[i].vel -= diff;
            moons[j].vel += diff;
        }
    }
    for moon in moons {
        moon.pos += moon.vel;
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Moon {
    pos: Point3i,
    vel: Point3i,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Moon {
            pos: Point3i::new(x, y, z),
            vel: Point3i::new(0, 0, 0),
        }
    }

    fn energy(&self) -> i32 {
        self.pos.abs().sum() * self.vel.abs().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test12() {
        let input = String::from(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
        );
        let mut moons = get_moons(input);
        assert_eq!(
            solve_part1(10, &mut moons.iter().cloned().collect::<Vec<_>>()),
            179
        );
        assert_eq!(solve_part2(&mut moons), 2772);
        let input = String::from(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
        );
        let mut moons = get_moons(input);
        assert_eq!(
            solve_part1(100, &mut moons.iter().cloned().collect::<Vec<_>>()),
            1940
        );
        assert_eq!(solve_part2(&mut moons), 4686774924i64);
        crate::util::tests::test_full_problem(12, solve, 9127, 353620566035124i64);
    }
}
