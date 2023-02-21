// https://www.codewars.com/kata/54b72c16cd7f5154e9000457

use std::collections::{HashMap, HashSet};

fn main() {
    assert_eq!(decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"), ".... . -.--   .--- ..- -.. .");
    assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    assert_eq!(decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}

fn decode_bits(encoded: &str) -> String {
    // Group
    let groups = encoded
        .trim_matches('0')
        .chars()
        .fold::<Vec<(char, u8)>, _>(Vec::new(), |mut acc, elem| {
            if !acc.is_empty() && acc.last().unwrap().0 == elem {
                acc.last_mut().unwrap().1 += 1;
            } else {
                acc.push((elem, 1));
            }
            acc
        });

    let base_time = groups
        .iter()
        .fold(HashSet::new(), |mut set, tuple| {
            set.insert(tuple.1);
            set
        })
        .into_iter()
        .min()
        .unwrap();

    return groups.iter().fold(String::new(), |mut morse, tpl| {
        match (tpl.0, tpl.1 / base_time) {
            ('0', 1) => morse.push_str(""),
            ('0', 3) => morse.push_str(" "),
            ('0', 7) => morse.push_str("   "),
            ('1', 1) => morse.push_str("."),
            ('1', 3) => morse.push_str("-"),
            _ => panic!(),
        }
        morse
    });
}

fn decode_morse(encoded: &str) -> String {
    #[allow(non_snake_case)]
    let MORSE_CODE = local_morse_code();

    return encoded
        .trim()
        .split("   ")
        .map(|w| {
            w.split(" ")
                .filter(|&c| !c.is_empty())
                .map(|c| MORSE_CODE[c].clone())
                .collect::<String>()
        })
        .into_iter()
        .collect::<Vec<_>>()
        .join(" ");
}

fn local_morse_code() -> HashMap<String, String> {
    return HashMap::from([
        ("..--.-".to_owned(), "_".to_owned()),
        ("-....-".to_owned(), "-".to_owned()),
        ("---...".to_owned(), ",".to_owned()),
        ("--..--".to_owned(), ",".to_owned()),
        ("-.-.-.".to_owned(), ";".to_owned()),
        ("-.-.--".to_owned(), "!".to_owned()),
        ("..--..".to_owned(), "?".to_owned()),
        (".-.-.-".to_owned(), ".".to_owned()),
        (".----.".to_owned(), "'".to_owned()),
        (".-..-.".to_owned(), "\"".to_owned()),
        ("-.--.".to_owned(), "(".to_owned()),
        ("-.--.-".to_owned(), ")".to_owned()),
        (".--.-.".to_owned(), "@".to_owned()),
        ("-..-.".to_owned(), "/".to_owned()),
        (".-...".to_owned(), "&".to_owned()),
        (".-.-.".to_owned(), "+".to_owned()),
        ("-...-".to_owned(), "=".to_owned()),
        ("...-..-".to_owned(), "$".to_owned()),
        ("-----".to_owned(), "0".to_owned()),
        (".----".to_owned(), "1".to_owned()),
        ("..---".to_owned(), "2".to_owned()),
        ("...--".to_owned(), "3".to_owned()),
        ("....-".to_owned(), "4".to_owned()),
        (".....".to_owned(), "5".to_owned()),
        ("-....".to_owned(), "6".to_owned()),
        ("--...".to_owned(), "7".to_owned()),
        ("---..".to_owned(), "8".to_owned()),
        ("----.".to_owned(), "9".to_owned()),
        (".-".to_owned(), "A".to_owned()),
        ("-...".to_owned(), "B".to_owned()),
        ("-.-.".to_owned(), "C".to_owned()),
        ("-..".to_owned(), "D".to_owned()),
        (".".to_owned(), "E".to_owned()),
        ("..-.".to_owned(), "F".to_owned()),
        ("--.".to_owned(), "G".to_owned()),
        ("....".to_owned(), "H".to_owned()),
        ("..".to_owned(), "I".to_owned()),
        (".---".to_owned(), "J".to_owned()),
        ("-.-".to_owned(), "K".to_owned()),
        (".-..".to_owned(), "L".to_owned()),
        ("--".to_owned(), "M".to_owned()),
        ("-.".to_owned(), "N".to_owned()),
        ("---".to_owned(), "O".to_owned()),
        (".--.".to_owned(), "P".to_owned()),
        ("--.-".to_owned(), "Q".to_owned()),
        (".-.".to_owned(), "R".to_owned()),
        ("...".to_owned(), "S".to_owned()),
        ("...---...".to_owned(), "SOS".to_owned()),
        ("-".to_owned(), "T".to_owned()),
        ("..-".to_owned(), "U".to_owned()),
        ("...-".to_owned(), "V".to_owned()),
        (".--".to_owned(), "W".to_owned()),
        ("-..-".to_owned(), "X".to_owned()),
        ("-.--".to_owned(), "Y".to_owned()),
        ("--..".to_owned(), "Z".to_owned()),
    ]);
}
