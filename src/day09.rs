use crate::{computer::Computer, Solution};

use itertools::Itertools;

pub fn solve(input: String) -> Solution<String, String> {
    let program: Vec<i64> = input.split(',').map(|n| n.parse().unwrap()).collect();
    let p1 = solve_with_input(&program, Some(1));
    let p2 = solve_with_input(&program, Some(2));
    Solution::new(p1, p2)
}

fn solve_with_input(program: &[i64], computer_input: Option<i64>) -> String {
    let mut computer = Computer::new(program.to_owned());
    computer.run();
    if let Some(input) = computer_input {
        computer.insert(input);
        computer.run();
    }
    #[allow(clippy::let_and_return)]
    let result = computer
        .output_iter()
        .map(|output| output.to_string())
        .join(", ");
    result
} // 103.28ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test09() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let answer = solve_with_input(&program, None);
        assert_eq!(program.iter().join(", "), answer);
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let answer = solve_with_input(&program, None).parse::<i64>().unwrap();
        assert!(1000000000000000 <= answer && answer < 10000000000000000);
        let program = vec![104, 1125899906842624, 99];
        let answer = solve_with_input(&program, None).parse::<i64>().unwrap();
        assert_eq!(1125899906842624, answer);
        crate::util::tests::test_full_problem(
            9,
            solve,
            3345854957i64.to_string(),
            68938.to_string(),
        );
    }
}
