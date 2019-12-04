
pub fn solve(input: String) -> (usize, usize) {
    let intcodes = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut memory = intcodes.clone();
    let p1 = run(12, 2, &mut memory);
    for noun in 0..100 {
        for verb in 0..100 {
            memory = intcodes.clone();
            if run(noun, verb, &mut memory) == 19690720 {
                return (p1, 100 * noun + verb);
            }
        }
    }
    unreachable!();
} // 8.52ms

fn run(noun: usize, verb: usize, memory: &mut [usize]) -> usize {
    memory[1] = noun;
    memory[2] = verb;
    let mut i = 0;
    loop {
        let (a1, a2, s) = (memory[i + 1], memory[i + 2], memory[i + 3]);
        match memory[i] {
            99 => break memory[0],
            1 => memory[s] = memory[a1] + memory[a2],
            2 => memory[s] = memory[a1] * memory[a2],
            _ => panic!("Something went wrong :(")
        }
        i += 4;
    }
}

#[test]
fn example1() {
    let example_input = String::from("1,9,10,3,2,3,11,0,99,30,40,50").split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut memory = example_input.clone();
    assert_eq!(
        run(9, 10, &mut memory),
        3500
    )
}