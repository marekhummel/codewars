// https://www.codewars.com/kata/59568be9cc15b57637000054

use std::cmp::min;

fn main() {
    assert_eq!(elder_age(8, 5, 1, 100), 5);
    assert_eq!(elder_age(8, 8, 0, 100007), 224);
    assert_eq!(elder_age(25, 31, 0, 100007), 11925);
    assert_eq!(elder_age(5, 45, 3, 1000007), 4323);
    assert_eq!(elder_age(31, 39, 7, 2345), 1586);
    assert_eq!(elder_age(545, 435, 342, 1000007), 808451);
    assert_eq!(elder_age(65, 310, 16, 4359), 78);
    assert_eq!(elder_age(56511, 156362, 8876, 111150), 28895);
    assert_eq!(elder_age(353279, 397163, 267, 100554), 27281);

    // You need to run this test very quickly before attempting the actual tests :)
    assert_eq!(elder_age(28827050410, 35165045587, 7109602, 13719506), 5456283);
}

fn elder_age(m: u64, n: u64, l: u64, t: u64) -> u64 {
    elder_age_recursive(m, n, 0, 0, l, t)
    // elder_age_naive(m, n, l, t)
}

fn elder_age_recursive(m: u64, n: u64, ox: u64, oy: u64, l: u64, t: u64) -> u64 {
    // Abort if negative or zero side length
    if m < 1 || n < 1 {
        return 0;
    }

    // Let width be the bigger side
    let (w, h) = if m >= n { (m, n) } else { (n, m) };

    // Find biggest power of two as box, because one line contains set range of integers
    let biggest_pow_of_two = (w as f32).log2() as u64;
    let box_width = 1 << biggest_pow_of_two;
    let box_height = min(h, box_width);

    // Compute the sum of one line in the box (use u128 cause we cant mod before div by 2)
    let xor_value = ox ^ oy;
    let box_min = (xor_value.clamp(l, u64::MAX) - l) as u128;
    let box_max = ((xor_value + box_width - 1).clamp(l, u64::MAX) - l) as u128;
    let box_donation_line = ((box_min + box_max) * (box_max - box_min + 1)) / 2;

    // Compute total donation in box (mod first before cast)
    let box_donation = ((box_donation_line % t as u128) as u64 * (box_height % t)) % t;

    // Parts outside the box are computed recursively
    let (diff_w, diff_h) = (w - box_width, h - box_height);
    let bottom = elder_age_recursive(box_width, diff_h, ox, oy + box_height, l, t);
    let right = elder_age_recursive(diff_w, box_height, ox + box_width, oy, l, t);
    let remainder = elder_age_recursive(diff_w, diff_h, ox + box_width, oy + box_height, l, t);

    return (box_donation + bottom + right + remainder) % t;
}

#[allow(dead_code)]
fn elder_age_naive(m: u64, n: u64, l: u64, t: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..m {
        for j in 0..n {
            let potential = i ^ j;
            let donation = if potential > l { potential - l } else { 0 };
            sum = (sum + donation) % t;
            // print!("{:02} ", donation);
        }
        // println!();
    }

    sum
}
