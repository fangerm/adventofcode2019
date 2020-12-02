use crate::intcode::{Program, Status};
use crate::Point;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Rotation {
    Left,
    Right,
    Up,
    Down,
}

impl Rotation {
    pub fn rot(&self, dir: isize) -> Rotation {
        match (self, dir) {
            (Rotation::Left, DIR_CW) => Rotation::Up,
            (Rotation::Right, DIR_CW) => Rotation::Down,
            (Rotation::Up, DIR_CW) => Rotation::Right,
            (Rotation::Down, DIR_CW) => Rotation::Left,
            // Work smart, not hard lol
            _ => self.rot(DIR_CW).rot(DIR_CW).rot(DIR_CW),
        }
    }

    pub fn apply(&self, location: &mut Point) {
        match self {
            Rotation::Left => location.x += 1,
            Rotation::Right => location.x -= 1,
            Rotation::Up => location.y -= 1,
            Rotation::Down => location.y += 1,
        }
    }
}

const DIR_CCW: isize = 0;
const DIR_CW: isize = 1;
const BLACK: isize = 0;
const WHITE: isize = 1;

pub fn a11_1() {
    println!("{}", a11(BLACK).len())
}

pub fn a11_2() {
    let painted = a11(WHITE);
    let smallest_x = painted.keys().min_by_key(|p| p.x).unwrap().x;
    let smallest_y = painted.keys().min_by_key(|p| p.y).unwrap().y;
    let biggest_x = painted.keys().max_by_key(|p| p.x).unwrap().x;
    let biggest_y = painted.keys().max_by_key(|p| p.y).unwrap().y;

    for x in smallest_x..=biggest_x {
        for y in smallest_y..=biggest_y {
            print!("{}", painted.get(&Point { x, y }).unwrap_or(&0))
        }
        println!();
    }
}

fn a11(starting: isize) -> HashMap<Point, isize> {
    let mut program = Program::new("inputs/input-11");
    let mut painted = HashMap::<Point, isize>::new();
    let mut current = Point { x: 0, y: 0 };
    painted.insert(current, starting);
    let mut rot = Rotation::Right;

    program.execute_halt();
    while program.status != Status::Halted {
        let color = painted.get(&current).unwrap_or(&BLACK);
        let out = program.resume_input(*color);
        painted.insert(current, out.into_output());

        let dir = program.resume_output();
        rot = rot.rot(dir.into_output());
        rot.apply(&mut current);

        program.resume_output();
    }
    painted
}
