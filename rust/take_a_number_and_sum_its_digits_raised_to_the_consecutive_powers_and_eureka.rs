// https://www.codewars.com/kata/5626b561280a42ecc50000d1

fn main() {
    assert_eq!(sum_dig_pow(1, 10), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(sum_dig_pow(1, 100), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 89]);
    assert_eq!(sum_dig_pow(10, 89), &[89]);
    assert_eq!(sum_dig_pow(10, 100), &[89]);
    assert_eq!(sum_dig_pow(90, 100), &[]);
    assert_eq!(sum_dig_pow(89, 135), &[89, 135]);
}

fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b)
        .into_iter()
        .filter(|n| {
            n.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .enumerate()
                .map(|(i, d)| d.pow(i as u32 + 1))
                .sum::<u64>()
                == *n
        })
        .collect()
}
