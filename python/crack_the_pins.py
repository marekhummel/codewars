# https://www.codewars.com/kata/5efae11e2d12df00331f91a6

import hashlib

LOOKUP = {
    hashlib.md5(pin_str.encode("UTF-8")).hexdigest(): pin_str
    for pin_str in (f"{pin:05d}" for pin in range(100000))
}


def crack(msg: str) -> str | None:
    return LOOKUP.get(msg)


def main():
    assert crack("827ccb0eea8a706c4c34a16891f84e7b") == "12345"
    assert crack("86aa400b65433b608a9db30070ec60cd") == "00078"


main()
