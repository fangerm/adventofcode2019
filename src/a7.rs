use crate::intcode::{Program, Status};
use std::cell::Cell;
use std::cmp::{max, min};
use std::{cmp, iter};

pub fn a71() {
    a7(0, 5)
}

pub fn a72() {
    a7(5, 10)
}

pub fn a7(min: isize, max: isize) {
    let mut max_amp = 0;
    // Thanks, I hate it!
    for a in min..max {
        for b in min..max {
            for c in min..max {
                for d in min..max {
                    for e in min..max {
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
                            max_amp = cmp::max(a7_exec(&[a, b, c, d, e]), max_amp);
                        }
                    }
                }
            }
        }
    }
    println!("{}", max_amp);
}

fn a7_exec(values: &[isize]) -> isize {
    let mut amps = iter::repeat(Program::new("input-7"))
        .take(5)
        .collect::<Vec<_>>();
    let last_out = Cell::new(0);

    for (mut amp, value) in amps.iter_mut().zip(values) {
        amp.execute_halt(); // Execute up to needing phase
        amp.resume_input(*value); // Give it the phase, now suspended on thrust
    }

    let mut index = 0;
    loop {
        let mut amp = &mut amps[index];
        if amp.status == Status::Halted {
            break;
        }

        let out = amp.resume_input(last_out.get());
        match out {
            Status::Output(out) => last_out.set(out),
            _ => (),
        }
        amp.resume_output();

        index = if index == values.len() - 1 {
            0
        } else {
            index + 1
        };
    }

    last_out.get()
}
