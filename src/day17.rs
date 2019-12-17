use crate::{
    computer::{Channel, Computer},
    Solution,
};
#[allow(unused_imports)]
use itertools::Itertools;

pub fn solve(input: String) -> Solution<usize, i64> {
    let mut program = input
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();
    program[0] = 2;
    let mut computer = Computer::new(program);
    computer
        .set_output_channel(Channel::new(5000)) // Sufficiently large
        .set_input_channel(Channel::new(100))
        .run();
    let mut grid = vec![vec![]];
    loop {
        match computer.try_pop() {
            Some(10) => grid.push(Vec::with_capacity(grid.last().unwrap().len())),
            Some(output) => grid.last_mut().unwrap().push(output),
            None => break,
        }
    }
    /* // Display grid
    println!(
        "{}",
        grid.iter()
            .map(|row| row
                .iter()
                .map(|e| match e {
                    35 => '#',
                    46 => '.',
                    _ => 'X',
                })
                .join(""))
            .join("\n")
    );
    */
    grid.pop(); // Popping output "\nMain:\n" in ASCII
    grid.pop();
    grid.pop();
    let mut p1 = 0;
    for j in 1..grid.len() - 1 {
        for i in 1..grid[0].len() - 1 {
            if grid[j][i] == 35
                && grid[j + 1][i] == 35
                && grid[j - 1][i] == 35
                && grid[j][i + 1] == 35
                && grid[j][i - 1] == 35
            {
                p1 += i * j;
            }
        }
    }

    // Reading the grid and manually creating the instructions
    // Main: 65,44,67,44,65,44,66,44,65,44,67,44,66,44,67,44,66,44,67
    // A:82,44,56,44,76,44,49,48,44,76,44,49,50,44,82,52
    // B:82,44,56,44,76,44,49,48,44,82,44,56
    // C:82,44,56,44,76,44,49,50,44,82,44,52,44,82,44,52

    // Main
    computer
        .insert(65)
        .insert(44) // A
        .insert(67)
        .insert(44) // C
        .insert(65)
        .insert(44) // A
        .insert(66)
        .insert(44) // B
        .insert(65)
        .insert(44) // A
        .insert(67)
        .insert(44) // C
        .insert(66)
        .insert(44) // B
        .insert(67)
        .insert(44) // C
        .insert(66)
        .insert(44) // B
        .insert(67)
        .insert(10) // C
        // Function A
        .insert(82)
        .insert(44)
        .insert(56)
        .insert(44) // R8
        .insert(76)
        .insert(44)
        .insert(49)
        .insert(48)
        .insert(44) // L10
        .insert(76)
        .insert(44)
        .insert(49)
        .insert(50)
        .insert(44) // L12
        .insert(82)
        .insert(44)
        .insert(52)
        .insert(10) // R4
        // Function B
        .insert(82)
        .insert(44)
        .insert(56)
        .insert(44) // R8
        .insert(76)
        .insert(44)
        .insert(49)
        .insert(48)
        .insert(44) // L10
        .insert(82)
        .insert(44)
        .insert(56)
        .insert(10) // R8
        // Function C
        .insert(82)
        .insert(44)
        .insert(56)
        .insert(44) // R8
        .insert(76)
        .insert(44)
        .insert(49)
        .insert(50)
        .insert(44) // L12
        .insert(82)
        .insert(44)
        .insert(52)
        .insert(44) // R4
        .insert(82)
        .insert(44)
        .insert(52)
        .insert(10) // R4
        // No video feed
        .insert(110)
        .insert(10)
        .run();
    let p2 = computer.output_iter().last().unwrap();
    Solution::new(p1, p2)
} // 33.58ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test17() {
        crate::util::tests::test_full_problem(17, solve, 4044, 893283);
    }
}
