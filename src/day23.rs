use crate::{computer::Computer, Error, Solution};

use std::collections::VecDeque;

//use crossbeam::{channel, thread};
//use std::sync::Barrier;

struct Packet(i64, i64, Option<i64>);

impl Packet {
    fn dest(&self) -> i64 {
        self.0
    }

    fn x(&self) -> i64 {
        self.1
    }

    fn y(&self) -> Result<i64, Error> {
        match self.2 {
            Some(val) => Ok(val),
            None => bail!("Packet contains no y value"),
        }
    }
}

pub fn solve(input: String) -> Result<Solution<i64, i64>, Error> {
    /*
    let mut queue = VecDeque::new();
    let mut computers = Vec::with_capacity(50);
    for network_address in 0..50 {
        let mut computer = Computer::new(input.clone())?
            .insert(network_address);
        computers.push(computer);
    }
    let mut current = 0;
    loop {


    }
    */

    Ok(Solution::new(0, 0))

    /*
    let barrier = Barrier::new(5);
    let result = thread::scope(|s| {
        let (tx_out, rx_out) = channel::bounded(50);
        let mut handles = Vec::new();
        let mut senders = Vec::new();
        for i in 0..50 {
            let (tx_in, rx_in) = channel::bounded(50);
            senders.push(tx_in);
            let input = input.clone();
            let barrier = &barrier;
            let tx_out = tx_out.clone();
            let handle = s.spawn(move |_| {
                if let Ok((x, Some(y), in_channel, out_channel)) = rx_in.recv() {
                    barrier.wait();
                    let mut computer = Computer::new(input.clone())?;
                    computer
                        .set_input_channel(in_channel)
                        .set_output_channel(out_channel);
                    loop {
                        computer.run()?;
                        computer.insert(-1)?;
                    }
                    barrier.wait();
                    match computer.run()?.try_pop() {
                        Some(i) => {},
                        None => {},
                    }
                    if i == 4 {
                        let result = computer.pop();
                        tx_out.send((part, result)).unwrap();
                    }
                }
                Ok::<_, Error>(())
            });
            handles.push(handle);
        }

        let channels: Vec<_> = (0..50).map(|_| Channel::default()).collect();
        let mut outputs = (0..50).map(|i| channels[i].clone());
        let mut inputs = (0..50).map(|i| channels[(i + 49) % 50].clone());
        for i in 0..50 {
            let output = outputs.next().unwrap();
            let input = inputs.next().unwrap();
            //senders[i].send((1, phases[i], input, output)).unwrap();
        }

        drop(senders);
        drop(tx_out);

        let mut p1 = 0;
        let mut p2 = 0;

        let mut iter = rx_out.iter();
        while let Some((part, output)) = iter.next() {
            match part {
                1 => p1 = p1.max(output),
                2 => p2 = p2.max(output),
                _ => unreachable!(),
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Ok::<_, Error>(Solution::new(p1, p2))
    })
    .unwrap()?;
    Ok(result)
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test23() {
        crate::util::tests::test_full_problem(23, solve, 0, 0);
    }
}
