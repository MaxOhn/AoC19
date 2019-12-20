use crate::{Error, Solution};

pub fn solve(input: String) -> Result<Solution<i32, i32>, Error> {
    let (mut p1, mut p2) = (0, 0);
    for line in input.lines() {
        let mut num: i32 = line.parse()?;
        p1 += num;
        while num > 0 {
            num = num / 3 - 2;
            p2 += num;
        }
    }
    /*
    input
        .lines()
        .map(|line| line.parse::<i32>()?)
        .for_each(|num| {
            let mut n = num / 3 - 2;
            p1 += n;
            while n > 0 {
                p2 += n;
                n = n / 3 - 2;
            }
        });
    */
    Ok(Solution::new(p1, p2))
} // 4.44ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve(String::from("14")).unwrap(), Solution::new(2, 2));
        assert_eq!(
            solve(String::from("1969")).unwrap(),
            Solution::new(654, 966)
        );
        assert_eq!(
            solve(String::from("100756")).unwrap(),
            Solution::new(33583, 50346)
        );
        crate::util::tests::test_full_problem(1, solve, 3296560, 4941976);
    }
}
