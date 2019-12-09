
use crate::solution::Solution;
use crate::computer::Computer;

#[allow(unused)]
use itertools::Itertools;

pub fn solve(input: String) -> Solution<String, String> {
    let program: Vec<i64> = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let p1 = solve_with_input(&program, Some(1));
    let p2 = solve_with_input(&program, Some(2));
    Solution::new(p1, p2)
}

fn solve_with_input(program: &Vec<i64>, computer_input: Option<i64>) -> String {
    let mut computer = Computer::new(program.clone());
    computer.run();
    if let Some(input) = computer_input {
        computer.run_on_input(input);
    }
    let mut result = String::new();
    if let Some(output) = computer.get_output() {
        result.push_str(&format!("{}", output));
        while let Some(output) = computer.get_output() {
            result.push_str(&format!(", {}", output))
        }
    }
    result
} // 114.91ms

#[test]
fn example1() {
    let mut program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let answer = solve_with_input(&program, None);
    program.reverse();
    assert_eq!(program.iter().join(", "), answer);
}

#[test]
fn example2() {
    let program = vec![1102,34915192,34915192,7,4,7,99,0];
    let answer = solve_with_input(&program, None).parse::<i64>().unwrap();
    assert!(1000000000000000 <= answer && answer < 10000000000000000);
}

#[test]
fn example3() {
    let program = vec![104,1125899906842624,99];
    let answer = solve_with_input(&program, None).parse::<i64>().unwrap();
    assert_eq!(1125899906842624, answer);
}

