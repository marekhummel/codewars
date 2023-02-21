// https://www.codewars.com/kata/578553c3a1b8d5c40300037c

fn main() {
    assert_eq!(binary_slice_to_number(&vec![0, 0, 0, 1]), 1);
    assert_eq!(binary_slice_to_number(&vec![0, 0, 1, 0]), 2);
    assert_eq!(binary_slice_to_number(&vec![1, 1, 1, 1]), 15);
    assert_eq!(binary_slice_to_number(&vec![0, 1, 1, 0]), 6);
}

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    return slice.iter().fold(0, |acc, bit| acc * 2 + bit);
}
