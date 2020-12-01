use crate::intcode::Program;
use crate::a6::a6;

mod a3;
mod a4;
mod a6;
mod intcode;

fn main() {
    a6();
}

fn a5() {
    Program::new("input-5").execute();
}
