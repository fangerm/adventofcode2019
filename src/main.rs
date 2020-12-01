use crate::a7::a7;
use crate::intcode::Program;

mod a3;
mod a4;
mod a6;
mod a7;
mod intcode;

fn main() {
    a7();
}

fn a5() {
    Program::new("input-5").exec_stdio();
}
