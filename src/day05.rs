use crate::{computer::Computer, Solution};

pub fn solve(input: String) -> Solution<i64, i64> {
    let intcodes: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let mut computer = Computer::new(intcodes.clone());
    let p1 = computer.insert(1).run().output_iter().last().unwrap();
    let mut computer = Computer::new(intcodes.clone());
    let p2 = computer.insert(5).run().pop();
    Solution::new(p1, p2)
} // 0.77ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test05() {
        let intcodes = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut computer = Computer::new(intcodes.clone());
        assert_eq!(computer.insert(7).run().pop(), 1);
        let mut computer = Computer::new(intcodes.clone());
        assert_eq!(computer.insert(9).run().pop(), 0);
        let intcodes = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut computer = Computer::new(intcodes.clone());
        assert_eq!(computer.insert(7).run().pop(), 999);
        let mut computer = Computer::new(intcodes.clone());
        assert_eq!(computer.insert(8).run().pop(), 1000);
        let mut computer = Computer::new(intcodes.clone());
        assert_eq!(computer.insert(9).run().pop(), 1001);
        crate::util::tests::test_full_problem(5, solve, 7265618, 7731427);
    }
}
