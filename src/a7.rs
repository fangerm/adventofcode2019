use crate::intcode::Program;
use std::iter;
use std::cmp::max;
use std::cell::Cell;

pub fn a7() {
    let mut max_amp = 0;
    // Thanks, I hate it!
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        if a == b
                            || a == c
                            || a == d
                            || a == e
                            || b == c
                            || b == d
                            || b == e
                            || c == d
                            || c == e
                            || d == e
                        {
                            continue;
                        } else {
                            max_amp = max(_a7(&[a, b, c, d, e]), max_amp);
                        }
                    }
                }
            }
        }
    }
    println!("{}", max_amp);
}

fn _a7(values: &[isize]) -> isize {
    let amps = iter::repeat(Program::new("input-7")).take(5).collect::<Vec<_>>();
    let last_out = Cell::new(0);
    for (mut amp, value) in amps.into_iter().zip(values) {
        let mut second_in = false;
        amp.execute(
            || if second_in { last_out.get() } else {
                second_in = true;
                *value
            },
            |value| last_out.set(value)
        );
    }
    last_out.get()
}
