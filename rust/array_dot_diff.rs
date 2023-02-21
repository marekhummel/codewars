// https://www.codewars.com/kata/523f5d21c841566fde000009

fn main() {
    assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
    assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    assert_eq!(array_diff(vec![1, 2, 3], vec![1, 2]), vec![3]);
}

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().fold(Vec::new(), |mut acc, elem| {
        if !b.contains(&elem) {
            acc.push(elem)
        }
        acc
    })
    // a.into_iter().filter(|x| !b.contains(x)).collect()
}
