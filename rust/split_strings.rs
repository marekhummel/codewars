// https://www.codewars.com/kata/515de9ae9dcfc28eb6000001

fn main() {
    println!("{:?}", solution("abc"));
    println!("{:?}", solution("abcdef"));
}

fn solution(s: &str) -> Vec<String> {
    let mut ps = String::from(s.clone());

    if ps.len() & 1 == 1 {
        ps = format!("{}_", s);
    }

    return ps
        .chars()
        .zip(ps.chars().skip(1))
        .enumerate()
        .filter(|&(i, _)| i & 1 == 0)
        .map(|(_, x)| format!("{}{}", x.0, x.1))
        .collect();
}

// s.chars().chunks(2).into_iter().map(|c| c.pad_using(2, |_| '_').collect()).collect()
