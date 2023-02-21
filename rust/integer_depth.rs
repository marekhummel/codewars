// https://www.codewars.com/kata/59b401e24f98a813f9000026

fn main() {
    assert_eq!(compute_depth(1), 10);
    assert_eq!(compute_depth(42), 9);
    assert_eq!(compute_depth(8), 12);
    assert_eq!(compute_depth(13), 8);
    assert_eq!(compute_depth(7), 10);
    assert_eq!(compute_depth(25), 36);
}

fn compute_depth(n: u16) -> u8 {
    let target: u32 = 0b11111_11111;

    let mut current: u32 = 0;

    for depth in 1.. {
        let multiple = n * (depth as u16);
        let mult_str = multiple.to_string();
        let digits = mult_str.chars().map(|d| d.to_digit(10).unwrap());

        current = digits.fold(current, |mask, digit| (mask | (1 << digit)));
        if current == target {
            return depth;
        }
    }

    panic!();
}
