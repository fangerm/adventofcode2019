use std::fs::read_to_string;
use std::io;
use std::io::BufRead;

const HALT: usize = 99;
const ADD: usize = 1;
const MUL: usize = 2;
const IN: usize = 3;
const OUT: usize = 4;
const JUMP_IF_TRUE: usize = 5;
const JUMP_IF_FALSE: usize = 6;
const LESS_THAN: usize = 7;
const EQUALS: usize = 8;
const ADD_TO_REL_BASE: usize = 9;

const POSITION_MODE: usize = 0;
const IMMEDIATE_MODE: usize = 1;
const RELATIVE_MODE: usize = 2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Ready,
    Output(isize),
    WaitOnInput,
    Halted,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub data: Vec<isize>,
    pub ip: usize,
    pub modes: Vec<usize>,
    pub status: Status,
    relative_base: isize
}

impl Program {
    pub fn new(location: &str) -> Program {
        Program {
            data: read_to_string(location)
                .expect("Failed to read program")
                .trim()
                .split(',')
                .map(|int| int.parse().expect("Invalid number"))
                .collect(),
            ip: 0,
            modes: Vec::with_capacity(5),
            status: Status::Ready,
            relative_base: 0
        }
    }

    pub fn exec_stdio(&mut self) {
        self.execute(
            || {
                let stdin = io::stdin();
                let line = stdin
                    .lock()
                    .lines()
                    .next()
                    .expect("there was no next line")
                    .expect("the line could not be read");
                line.parse().expect("User input NaN")
            },
            |num| println!("{}", num),
        );
    }

    pub fn execute<I, O>(&mut self, mut input: I, mut output: O)
    where
        I: FnMut() -> isize,
        O: FnMut(isize),
    {
        loop {
            let run = self.execute_halt();
            match run {
                Status::Output(out) => {
                    output(out);
                    self.resume_output();
                }
                Status::WaitOnInput => {
                    self.resume_input(input());
                }
                Status::Halted => return,
                _ => (),
            }
        }
    }

    pub fn execute_halt(&mut self) -> Status {
        loop {
            let inst = self.decode();
            match inst {
                ADD => {
                    let a = self.param(1);
                    let b = self.param(2);
                    self.store(3, a + b);
                    self.ip += 4;
                }

                MUL => {
                    let a = self.param(1);
                    let b = self.param(2);
                    self.store(3, a * b);
                    self.ip += 4;
                }

                IN => self.status = Status::WaitOnInput,
                OUT => self.status = Status::Output(self.param(1)),

                JUMP_IF_TRUE => self.jump(|v| v != 0),
                JUMP_IF_FALSE => self.jump(|v| v == 0),

                LESS_THAN => self.compare(|a, b| a < b),
                EQUALS => self.compare(|a, b| a == b),

                ADD_TO_REL_BASE => {
                    self.relative_base += self.param(1);
                    self.ip += 2;
                }

                HALT => self.status = Status::Halted,

                _ => panic!(
                    "unknown opcode {} at ip {}, memory {:?}",
                    inst, self.ip, self.data
                ),
            }

            if self.status != Status::Ready {
                return self.status;
            }
        }
    }

    pub fn resume_input(&mut self, input: isize) -> Status {
        assert!(matches!(self.status, Status::WaitOnInput));
        self.status = Status::Ready;
        self.store(1, input);
        self.ip += 2;
        self.execute_halt()
    }

    pub fn resume_output(&mut self) -> Status {
        assert!(matches!(self.status, Status::Output(_)));
        self.status = Status::Ready;
        self.ip += 2;
        self.execute_halt()
    }

    fn jump<T: Fn(isize) -> bool>(&mut self, condition: T) {
        if condition(self.param(1)) {
            self.ip = self.param(2) as usize;
        } else {
            self.ip += 3;
        }
    }

    fn compare<T: Fn(isize, isize) -> bool>(&mut self, condition: T) {
        if condition(self.param(1), self.param(2)) {
            self.store(3, 1)
        } else {
            self.store(3, 0)
        }
        self.ip += 4;
    }

    fn decode(&mut self) -> usize {
        self.modes.clear();
        let inst = self.data[self.ip];
        inst.to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .for_each(|d| self.modes.push(d));
        self.modes.pop().unwrap() + (self.modes.pop().unwrap_or(0) * 10)
    }

    fn param(&mut self, offset: usize) -> isize {
        let idx = self.idx_of(offset);
        self.get(idx)
    }

    fn store(&mut self, offset: usize, val: isize) {
        let idx = self.idx_of(offset);
        self.set(idx as usize, val);
    }

    fn idx_of(&mut self, offset: usize) -> usize {
        let mode = if self.modes.len() >= offset {
            self.modes[self.modes.len() - offset]
        } else {
            POSITION_MODE
        };

        match mode {
            POSITION_MODE => self.get(self.ip + offset) as usize,
            IMMEDIATE_MODE => self.ip + offset,
            RELATIVE_MODE => (self.relative_base + self.get(self.ip + offset)) as usize,
            _ => panic!("unknown parameter mode"),
        }
    }

    pub fn get(&mut self, loc: usize) -> isize {
        self.maybe_resize(loc);
        self.data[loc]
    }

    pub fn set(&mut self, loc: usize, val: isize) {
        self.maybe_resize(loc);
        self.data[loc] = val;
    }

    fn maybe_resize(&mut self, loc: usize) {
        if loc >= self.data.len() {
            self.data.resize(loc + 10, 0);
        }
    }
}
