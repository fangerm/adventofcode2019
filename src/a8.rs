use std::fs::read_to_string;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_COUNT: usize = WIDTH * HEIGHT;

pub fn a81() {
    let str = read_to_string("inputs/input-8").expect("Failed to read image");
    let mut digits = str.chars().map(|c| c.to_digit(10).expect("NaN"));

    let mut smallest_zero_count = 9999999;
    let mut smallest_output_value = 0;

    while digits.size_hint().0 != 0 {
        let mut total_zero = 0;
        let mut total_one = 0;
        let mut total_two = 0;
        for _ in 0..LAYER_COUNT {
            let digit = digits.next().unwrap();
            match digit {
                0 => total_zero += 1,
                1 => total_one += 1,
                2 => total_two += 1,
                _ => (),
            }
        }
        if total_zero < smallest_zero_count {
            smallest_zero_count = total_zero;
            smallest_output_value = total_one * total_two;
        }
    }

    println!("{}", smallest_output_value);
}

pub fn a82() {
    let mut pixels = [[2; WIDTH]; HEIGHT];
    let str = read_to_string("inputs/input-8").expect("Failed to read image");
    let mut digits = str.chars().map(|c| c.to_digit(10).expect("NaN"));

    while digits.size_hint().0 != 0 {
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                let digit = digits.next().unwrap();
                if pixels[h][w] == 2 {
                    pixels[h][w] = digit
                }
            }
        }
    }

    for row in &pixels {
        for pixel in row {
            print!("{}", pixel);
        }
        println!();
    }
}
