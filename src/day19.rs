use crate::{computer::Computer, Solution};

pub fn solve(input: String) -> Solution<usize, i64> {
    let program: Vec<i64> = input.split(',').map(|n| n.parse().unwrap()).collect();
    let mut p1 = 0;
    for x in 0..50 {
        for y in 0..50 {
            if gets_pulled(&program, x, y) {
                p1 += 1;
            }
        }
    }
    let mut x = 3;
    let mut y = 4;
    loop {
        while gets_pulled(&program, x, y) {
            x += 1;
        }
        if x > 99 && gets_pulled(&program, x - 100, y + 99) {
            return Solution::new(p1, (x - 100) * 10_000 + y);
        }
        y += 1;
    }
} // 584.81ms

fn gets_pulled(program: &[i64], x: i64, y: i64) -> bool {
    let mut drone = Computer::new(program.to_owned());
    drone.insert(x).insert(y).run().pop() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test19() {
        crate::util::tests::test_full_problem(19, solve, 189, 7621042);
    }
}
