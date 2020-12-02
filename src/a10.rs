use std::{collections::HashMap, cell::RefCell, fs::read_to_string};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize, y: usize
}

pub fn a10() {
    // TODO: This does not work.
    let asteroids = read_to_string("inputs/input-10")
        .expect("Failed to read map")
        .split('\n')
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<_>>>();

    let mut map = HashMap::<Point, RefCell<HashMap<i64, Point>>>::new();
    for (x, line) in asteroids.iter().enumerate() {
        for (y, _) in line.iter().filter(|e| **e).enumerate() {
            map.insert(Point { x, y }, Default::default());
        }
    }

    for (location, l_v) in map.iter() {
        for (other, o_v) in map.iter() {
            let angle = get_angle(location, other);
            let angle = (angle + 180.0) % 360.0;
            l_v.borrow_mut().insert(angle as i64, *other);
            o_v.borrow_mut().insert(angle as i64, *location);
        }
    }

    let max = map.iter().max_by_key(|(_, visible)| visible.borrow().len()).unwrap();
    println!("{:?}: {}", max.0, max.1.borrow().len());
}


fn get_angle(a: &Point, b: &Point) -> f64 {
    let angle = (b.y as f64 - a.y as f64).atan2((b.x as f64 - a.x as f64) as f64).to_degrees();
    if angle < 0.0 {
        angle + 360.0
    } else {
        angle
    }
}
