// https://www.codewars.com/kata/5262119038c0985a5b00029f

fn main() {
    println!("{:?}", (0..=100i64).filter(|n| is_prime(*n)).collect::<Vec<_>>());
    println!("{:?}", is_prime(2i64.pow(29) - 3));
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }

    let root_n = (n as f64).sqrt() as i64;
    for p in 2..=root_n {
        if n % p == 0 {
            return false;
        }
    }

    return true;
}
