use crate::solution::Solution;

use std::collections::{HashMap, HashSet};

pub fn solve(input: String) -> Solution<u64, u64> {
    let reactions: HashMap<String, Reaction> = input
        .lines()
        .map(|line| {
            let r = Reaction::new(line);
            (r.output.name.clone(), r)
        })
        .collect();
    let p1 = produce_fuel(&reactions, 1);
    let mut bot = 0;
    let mut top = 1_000_000_000_000u64;
    let mut p2 = 0;
    loop {
        let fuel = bot + (top - bot) / 2;
        match produce_fuel(&reactions, fuel) {
            ore if ore <= 1_000_000_000_000 => {
                if ore == p2 {
                    return Solution::new(p1, fuel);
                }
                p2 = p2.max(ore);
                bot += (top - bot) / 2;
            }
            _ => top -= (top - bot) / 2,
        }
    }
}

fn produce_fuel(reactions: &HashMap<String, Reaction>, fuel: u64) -> u64 {
    let mut ore = 0;
    let mut extra = HashMap::new();
    let mut queue = Vec::new();
    queue.push(ReactPair::new(fuel, "FUEL".to_owned()));
    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        if curr.name == "ORE" {
            ore += curr.amount;
        } else {
            let reaction = reactions.get(&curr.name).unwrap();
            let used = curr
                .amount
                .min(*extra.entry(curr.name.clone()).or_insert(0));
            extra
                .get_mut(&curr.name)
                .map_or((), |e| *e = e.saturating_sub(used));
            let amount = curr.amount.saturating_sub(used);
            if amount > 0 {
                let mult = (amount as f64 / reaction.output.amount as f64).ceil() as u64;
                let preparing = reaction.output.amount * mult;
                if preparing > amount {
                    extra
                        .get_mut(&curr.name)
                        .map_or((), |e| *e += preparing - amount);
                }
                for input in &reaction.inputs {
                    queue.push(ReactPair::new(input.amount * mult, input.name.clone()));
                }
            }
        }
    }
    ore
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct ReactPair {
    amount: u64,
    name: String,
}

impl ReactPair {
    fn new(amount: u64, name: String) -> Self {
        Self { amount, name }
    }
}

struct Reaction {
    inputs: HashSet<ReactPair>,
    output: ReactPair,
}

impl Reaction {
    fn new(line: &str) -> Self {
        let mut inputs = HashSet::new();
        let mut split_arrow = line.split(" => ");
        split_arrow.next().unwrap().split(", ").for_each(|elem| {
            let mut elem = elem.split_whitespace();
            inputs.insert(ReactPair::new(
                elem.next().unwrap().trim().parse().unwrap(),
                elem.next().unwrap().trim().to_owned(),
            ));
        });
        let mut elem = split_arrow.next().unwrap().split_whitespace();
        let output = ReactPair::new(
            elem.next().unwrap().trim().parse().unwrap(),
            elem.next().unwrap().trim().to_owned(),
        );
        Reaction { inputs, output }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test14() {
        let input = String::from(
            "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        );
        assert_eq!(solve(input), Solution::new(13312, 82892753));
        let input = String::from(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF"
        );
        assert_eq!(solve(input), Solution::new(180697, 5586022));
        let input = String::from(
            "171 ORE => 8 CNZTR\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n5 BHXH, 4 VRPVC => 5 LTCX"
        );
        assert_eq!(solve(input), Solution::new(2210736, 460664));
        crate::util::tests::test_full_problem(14, solve, 598038, 2269325);
    }
}
