
use crate::solution::Solution;

pub fn solve(input: String) -> Solution<i32, i32> {
    let (mut p1, mut p2) = (0, 0);
    input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .for_each(|num| {
            let mut n = num / 3 - 2;
            p1 += n;
            while n > 0 {
                p2 += n;
                n = n / 3 - 2;
            }
        });
    Solution::new(p1, p2)
} // 4.44ms

#[test]
fn example1() {
    assert_eq!(
        solve(String::from("14")),
        Solution::new(2, 2)
    );
}

#[test]
fn example2() {
    assert_eq!(
        solve(String::from("1969")),
        Solution::new(654, 966)
    );
}

#[test]
fn example3() {
    assert_eq!(
        solve(String::from("100756")),
        Solution::new(33583, 50346)
    );
}
