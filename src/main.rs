use crate::intcode::Program;

mod a3;
mod a4;
mod intcode;

fn main() {
    a5();
}

fn a5() {
    Program::new("input-5").execute();
}
