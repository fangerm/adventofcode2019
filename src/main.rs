#![allow(dead_code)]
#![feature(destructuring_assignment)]

use crate::intcode::Program;
use a12::*;
use a4::*;

mod a10;
mod a11;
mod a12;
mod a3;
mod a4;
mod a6;
mod a7;
mod a8;
mod intcode;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    a12_1();
    a12_2();
}

fn a5() {
    Program::new("inputs/input-5").exec_stdio();
}

fn a9() {
    Program::new("inputs/input-9").execute(|| 2, |val| println!("{}", val));
}
