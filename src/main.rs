use crate::a7::a72;
use crate::intcode::Program;

mod a3;
mod a4;
mod a6;
mod a7;
mod intcode;

fn main() {
    a72();
}

fn a5() {
    Program::new("input-5").exec_stdio();
}
