fn a41() {
    run(|(digits, i, elem)| digits[i + 1] == elem);
}

fn a42() {
    run(|(digits, i, elem)| {
        digits[i + 1] == elem
            && *digits.get(i.overflowing_sub(1).0).unwrap_or(&10) != elem
            && *digits.get(i + 2).unwrap_or(&10) != elem
    })
}

fn run<T: Fn((&[u32], usize, u32)) -> bool>(cond: T) {
        let mut count = 0;
    for num in 245182usize..790572usize {
        let digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        if digits
            .iter()
            .try_fold(0, |acc, d| if *d < acc { Err(()) } else { Ok(*d) })
            .is_err()
        {
            continue;
        }

        count += digits.iter().enumerate().take(5).any(|(a, b)| cond((&digits, a, *b))) as i32;
    }
    println!("{}", count);
}