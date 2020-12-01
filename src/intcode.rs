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

const PARAMETER_MODE: usize = 0;
const IMMEDIATE_MODE: usize = 1;

pub struct Program {
    pub data: Vec<isize>,
    pub ip: usize,
    pub modes: Vec<usize>,
}

impl Program {
    pub fn new(location: &str) -> Program {
        Program {
            data: read_to_string(location)
                .expect("Failed to read program")
                .split(',')
                .map(|int| int.parse().expect("Invalid number"))
                .collect(),
            ip: 0,
            modes: Vec::with_capacity(5),
        }
    }

    pub fn execute(&mut self) {
        loop {
            let inst = self.decode();
            // println!("INST: {} ({}), IP: {}", inst, self.data[self.ip], self.ip);
            match inst {
                ADD => {
                    self.store(3, self.param(1) + self.param(2));
                    self.ip += 4;
                }

                MUL => {
                    self.store(3, self.param(1) * self.param(2));
                    self.ip += 4;
                }

                IN => {
                    let stdin = io::stdin();
                    let line = stdin
                        .lock()
                        .lines()
                        .next()
                        .expect("there was no next line")
                        .expect("the line could not be read");
                    self.store(1, line.parse().expect("User input NaN"));
                    self.ip += 2;
                }

                OUT => {
                    println!("{}", self.param(1));
                    self.ip += 2;
                }

                JUMP_IF_TRUE => self.jump(|v| v != 0),
                JUMP_IF_FALSE => self.jump(|v| v == 0),

                LESS_THAN => self.compare(|a, b| a < b),
                EQUALS => self.compare(|a, b| a == b),

                HALT => break,

                _ => panic!(
                    "unknown opcode {} at ip {}, memory {:?}",
                    inst, self.ip, self.data
                ),
            }
        }
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

    fn param(&self, offset: usize) -> isize {
        let mode = if self.modes.len() >= offset {
            self.modes[self.modes.len() - offset]
        } else {
            PARAMETER_MODE
        };

        match mode {
            PARAMETER_MODE => self.get(self.get(self.ip + offset) as usize),
            IMMEDIATE_MODE => self.get(self.ip + offset),
            _ => panic!("unknown parameter mode"),
        }
    }

    fn store(&mut self, offset: usize, val: isize) {
        let idx = self.get(self.ip + offset);
        self.data[idx as usize] = val;
    }

    pub fn get(&self, loc: usize) -> isize {
        self.data[loc]
    }

    pub fn set(&mut self, offset: usize, val: isize) {
        self.data[self.ip + offset] = val;
    }
}
