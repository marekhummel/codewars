// https://www.codewars.com/kata/54b724efac3d5402db00065e

use std::collections::HashMap;

fn main() {
    assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
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
