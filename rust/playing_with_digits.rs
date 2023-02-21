// https://www.codewars.com/kata/5552101f47fc5178b1000050

fn main() {
    assert_eq!(dig_pow(89, 1), 1);
    assert_eq!(dig_pow(92, 1), -1);
    assert_eq!(dig_pow(695, 2), 2);
    assert_eq!(dig_pow(46288, 3), 51);
    assert_eq!(dig_pow(3456789, 5), -1);
}

fn dig_pow(n: i64, p: i32) -> i64 {
    // your code
    let digits = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().into())
        .collect::<Vec<i64>>();

    let total: i64 = digits
        .into_iter()
        .enumerate()
        .map(|(i, d)| d.pow((p + i as i32) as u32) as i64)
        .sum();
    return if total % n == 0 { total / n } else { -1 };
}
