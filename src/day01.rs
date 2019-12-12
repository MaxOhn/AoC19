use crate::solution::Solution;

pub fn solve(input: String) -> Solution<i32, i32> {
    let (mut p1, mut p2) = (0, 0);
    input
        .lines()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve(String::from("14")), Solution::new(2, 2));
        assert_eq!(solve(String::from("1969")), Solution::new(654, 966));
        assert_eq!(solve(String::from("100756")), Solution::new(33583, 50346));
        crate::util::tests::test_full_problem(1, solve, 3296560, 4941976);
    }
}
