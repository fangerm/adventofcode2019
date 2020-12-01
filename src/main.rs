use crate::a8::a82;
use crate::intcode::Program;

mod a3;
mod a4;
mod a6;
mod a7;
mod a8;
mod intcode;

fn main() {
    a9();
}

fn a5() {
    Program::new("inputs/input-5").exec_stdio();
}

fn a9() {
    Program::new("inputs/input-9").execute(|| 2, |val| println!("{}", val));
}
