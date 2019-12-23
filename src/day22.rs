use crate::{Error, Solution};

pub fn solve(input: String) -> Result<Solution<usize, i32>, Error> {
    /*
    let __input =
        "deal into new stack\ncut -2\ndeal with increment 7\ncut 8\ncut -4\ndeal with increment 7\ncut 3\ndeal with increment 9\ndeal with increment 3\ncut -1"
        .to_owned();
    let input = __input;
    */
    let shuffle = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let first_word = split
                .next()
                .ok_or_else(|| error!("Can not read empty line"))?;
            let second_word = split
                .next()
                .ok_or_else(|| error!("Could not read second word of line '{}'", line))?;
            if first_word == "deal" {
                if second_word == "with" {
                    let incr: usize = split
                        .nth(1)
                        .ok_or_else(|| error!("Could not read increment value of line '{}'", line))?
                        .parse()?;
                    Ok(Step::Increment(incr))
                } else {
                    Ok(Step::Reverse)
                }
            } else {
                let cut: i32 = second_word.parse()?;
                Ok(Step::Cut(cut))
            }
        })
        .collect::<Result<Vec<Step>, Error>>()?;
    let mut p1 = 2019;
    for step in shuffle.iter() {
        p1 = step.predict_next(p1, 10_007);
    }
    const _LEN: i64 = 119_315_717_514_047;
    const _TIMES: i64 = 101_741_582_076_661;

    Ok(Solution::new(p1, 0))
}

#[derive(Debug)]
enum Step {
    Reverse,
    Increment(usize),
    Cut(i32),
}

impl Step {
    fn predict_next(&self, pos: usize, len: usize) -> usize {
        match self {
            Step::Reverse => len - pos - 1,
            Step::Increment(i) => (pos * i) % len,
            Step::Cut(c) => ((pos as i32 - c + len as i32) as usize) % len,
        }
    }

    #[allow(unused)]
    fn apply(&self, deck: &mut [usize]) {
        match self {
            Step::Reverse => deck.reverse(),
            Step::Increment(i) => {
                let size = deck.len();
                let mut new_deck = vec![0; size];
                let mut idx_n = 0;
                for card in deck.iter() {
                    new_deck[idx_n] = *card;
                    idx_n = (idx_n + i) % size;
                }
                deck.clone_from_slice(&new_deck);
            }
            Step::Cut(c) => {
                let (front, back) = if *c > 0 {
                    deck.split_at(*c as usize)
                } else {
                    deck.split_at(deck.len() - -c as usize)
                };
                #[allow(mutable_borrow_reservation_conflict)]
                deck.clone_from_slice(&back.iter().chain(front).cloned().collect::<Vec<_>>()[..]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test22() {
        crate::util::tests::test_full_problem(22, solve, 4284, 0);
    }
}
