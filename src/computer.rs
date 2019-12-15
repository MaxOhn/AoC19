use crossbeam::channel::{self, Receiver, RecvTimeoutError, SendTimeoutError, Sender};
use std::time::Duration;

pub struct Computer {
    memory: Vec<i64>,
    pc: usize,
    rb: i32,
    input: Channel,
    output: Channel,
    state: State,
    multithread: bool,
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Computer {
            memory,
            pc: 0,
            rb: 0,
            input: Channel::default(),
            output: Channel::default(),
            state: State::Ready,
            multithread: false,
        }
    }

    pub fn run(&mut self) -> &mut Self {
        if self.state == State::Done {
            return self;
        } else if self.state == State::Wait {
            if self.input.is_empty() {
                panic!("Cannot run while waiting for input");
            }
            self.state = State::Ready;
        }
        if self.memory[self.pc] == 99 {
            self.state = State::Done;
            return self;
        }
        while let Some(mut op) = Operation::new(&mut self.memory, self.pc, self.rb) {
            match op.opcode {
                1 => self.memory[op.w] = op.v1 + op.v2,
                2 => self.memory[op.w] = op.v1 * op.v2,
                3 => {
                    if self.multithread || !self.input.is_empty() {
                        let input = self.input.pop();
                        self.memory[op.w] = input;
                    } else {
                        self.state = State::Wait;
                        return self;
                    }
                }
                4 => self.output.push(op.v1),
                5 => {
                    op.pc = if op.v1 != 0 {
                        op.v2 as usize
                    } else {
                        op.pc + 3
                    }
                }
                6 => {
                    op.pc = if op.v1 == 0 {
                        op.v2 as usize
                    } else {
                        op.pc + 3
                    }
                }
                7 => self.memory[op.w] = if op.v1 < op.v2 { 1 } else { 0 },
                8 => self.memory[op.w] = if op.v1 == op.v2 { 1 } else { 0 },
                9 => self.rb = op.rb,
                _ => unreachable!(),
            }
            self.pc = op.pc;
        }
        self
    }

    #[allow(unused)]
    pub fn set_input_channel(&mut self, input: Channel) -> &mut Self {
        self.input = input;
        self
    }

    #[allow(unused)]
    pub fn set_output_channel(&mut self, output: Channel) -> &mut Self {
        self.output = output;
        self
    }

    #[allow(unused)]
    pub fn set_multithread(&mut self) -> &mut Self {
        self.multithread = true;
        self
    }

    pub fn pop(&mut self) -> i64 {
        self.output.pop()
    }

    pub fn try_pop(&mut self) -> Option<i64> {
        self.output.try_pop()
    }

    pub fn insert(&mut self, input: i64) -> &mut Self {
        self.input.push(input);
        self
    }

    pub fn output_iter<'a>(&'a mut self) -> impl Iterator<Item = i64> + 'a {
        self.output.iter()
    }
}

#[derive(Eq, PartialEq)]
enum State {
    Ready,
    Done,
    Wait,
}

#[derive(Clone, Debug)]
pub struct Channel {
    sender: Sender<i64>,
    receiver: Receiver<i64>,
}

impl Default for Channel {
    fn default() -> Self {
        Channel::new(32)
    }
}

impl Channel {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = channel::bounded(size);
        Channel { sender, receiver }
    }

    fn push(&mut self, input: i64) {
        if let Err(e) = self.sender.send_timeout(input, Duration::from_secs(2)) {
            match e {
                SendTimeoutError::Timeout(_) => {
                    panic!("Error: Timed out while trying to push output to channel")
                }
                _ => unreachable!(),
            }
        }
    }

    fn try_pop(&mut self) -> Option<i64> {
        match self.receiver.try_recv() {
            Ok(output) => Some(output),
            Err(_) => None,
        }
    }

    fn pop(&mut self) -> i64 {
        match self.receiver.recv_timeout(Duration::from_secs(2)) {
            Ok(output) => output,
            Err(e) => match e {
                RecvTimeoutError::Timeout => panic!("Error: Timed out while waiting for a result"),
                _ => unreachable!(),
            },
        }
    }

    fn iter<'a>(&'a mut self) -> impl Iterator<Item = i64> + 'a {
        self.receiver.try_iter()
    }

    fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }
}

struct Operation {
    opcode: i64,
    v1: i64,
    v2: i64,
    w: usize,
    pc: usize,
    rb: i32,
}

impl Operation {
    fn new(mem: &mut Vec<i64>, pc: usize, rb: i32) -> Option<Self> {
        while mem.len() <= pc + 3 {
            mem.push(0);
        }
        let opcode = mem[pc] % 100;
        if opcode == 99 {
            return None;
        }
        if opcode == 3 {
            let w = match (mem[pc] / 100) % 10 {
                0 => mem[pc + 1] as usize,
                2 => (rb + mem[pc + 1] as i32) as usize,
                _ => unreachable!(),
            };
            if mem.len() <= w {
                mem.resize(w + 1, 0);
            }
            return Some(Operation {
                opcode,
                v1: 0,
                v2: 0,
                w,
                pc: pc + 2,
                rb,
            });
        }

        let v1 = match (mem[pc] / 100) % 10 {
            0 => {
                if mem.len() as i64 <= mem[pc + 1] {
                    mem.resize(mem[pc + 1] as usize + 1, 0);
                }
                mem[mem[pc + 1] as usize]
            }
            1 => mem[pc + 1],
            2 => {
                if mem.len() as i32 <= rb + mem[pc + 1] as i32 {
                    mem.resize((rb as i64 + mem[pc + 1]) as usize + 1, 0);
                }
                mem[(rb + mem[pc + 1] as i32) as usize]
            }
            _ => unreachable!(),
        };
        if opcode == 4 {
            return Some(Operation {
                opcode,
                v1,
                v2: 0,
                w: 0,
                pc: pc + 2,
                rb,
            });
        }
        if opcode == 9 {
            return Some(Operation {
                opcode,
                v1: 0,
                v2: 0,
                w: 0,
                pc: pc + 2,
                rb: rb + v1 as i32,
            });
        }
        let v2 = match (mem[pc] / 1000) % 10 {
            0 => {
                if mem.len() as i64 <= mem[pc + 2] {
                    mem.resize(mem[pc + 2] as usize + 1, 0);
                }
                mem[mem[pc + 2] as usize]
            }
            1 => mem[pc + 2],
            2 => {
                if mem.len() as i32 <= rb + mem[pc + 2] as i32 {
                    mem.resize((rb + mem[pc + 2] as i32) as usize + 1, 0);
                }
                mem[(rb + mem[pc + 2] as i32) as usize]
            }
            _ => unreachable!(),
        };
        match opcode {
            1 | 2 | 7 | 8 => {
                let w = match (mem[pc] / 10000) % 10 {
                    0 => mem[pc + 3] as usize,
                    2 => (rb + mem[pc + 3] as i32) as usize,
                    _ => unreachable!(),
                };
                if mem.len() <= w {
                    mem.resize(w + 1, 0);
                }
                Some(Operation {
                    opcode,
                    v1,
                    v2,
                    w,
                    pc: pc + 4,
                    rb,
                })
            }
            5 | 6 => Some(Operation {
                opcode,
                v1,
                v2,
                w: 0,
                pc,
                rb,
            }),
            _ => panic!("Error: opcode = {}", opcode),
        }
    }
}
