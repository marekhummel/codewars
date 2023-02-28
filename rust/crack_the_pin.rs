// https://www.codewars.com/kata/5efae11e2d12df00331f91a6

use lazy_static::lazy_static;
use md5;

use std::collections::HashMap;

fn main() {
    assert_eq!(crack("827ccb0eea8a706c4c34a16891f84e7b".to_string()), Ok(12345));
    assert_eq!(crack("86aa400b65433b608a9db30070ec60cd".to_string()), Ok(00078));
}

fn crack(hash: String) -> Result<i32, ()> {
    lazy_static! {
        static ref LOOKUP: HashMap<String, String> = {
            (0..=99999)
                .map(|n| format!("{n:05}"))
                .map(|pin| (format!("{:x}", md5::compute(pin.as_bytes())), pin))
                .collect::<HashMap<_, _>>()
        };
    }

    match LOOKUP.get(&hash) {
        Some(pin) => Ok(pin.parse::<i32>().unwrap()),
        None => Err(()),
    }
}
