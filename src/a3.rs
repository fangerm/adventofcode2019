use std::cmp::min;
use std::collections::HashMap;
use std::fs::read_to_string;

fn a3() {
    let raw_data = read_to_string("input-3")
        .expect("Failed to read program")
        .lines()
        .map(|line| line.split(',').map(String::from).collect::<Vec<String>>())
        .collect::<Vec<_>>();

    let mut first_map: HashMap<(i32, i32), i32> = HashMap::with_capacity(1000);
    do_map(&raw_data[0], |x, y, steps_total| {
        first_map.insert((x, y), steps_total);
    });

    let mut collide_map: HashMap<(i32, i32), (i32, i32)> = HashMap::with_capacity(1000);
    do_map(&raw_data[1], |x, y, steps_total| {
        if let Some(first) = first_map.get(&(x, y)) {
            collide_map.insert((x, y), (*first, steps_total));
        }
    });

    let mut smallest_steps = 99999999;
    let mut smallest_dist = 99999999;
    for ((x, y), (first, second)) in collide_map {
        smallest_steps = min(smallest_steps, first + second);
        smallest_dist = min(smallest_dist, x.abs() + y.abs());
    }
    println!("smallest steps: {}", smallest_steps);
    println!("smallest dist: {}", smallest_dist);
}

fn do_map<T: FnMut(i32, i32, i32)>(steps: &[String], mut action: T) {
    let mut x = 0;
    let mut y = 0;
    let mut steps_total = 0;
    for step in steps {
        let code = step.chars().next().unwrap();
        let steps = step[1..].parse::<usize>().unwrap();

        let (xv, yv): (i32, i32) = match code {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("oof"),
        };

        for _ in 0..steps {
            x += xv;
            y += yv;
            steps_total += 1;
            action(x, y, steps_total);
        }
    }
}
