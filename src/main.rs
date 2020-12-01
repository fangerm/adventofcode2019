use crate::a8::a82;
use crate::intcode::Program;

mod a3;
mod a4;
mod a6;
mod a7;
mod a8;
mod intcode;

fn main() {
    a82();
}

fn a5() {
    Program::new("input-5").exec_stdio();
}
