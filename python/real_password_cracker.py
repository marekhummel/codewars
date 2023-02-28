# https://www.codewars.com/kata/59146f7b4670ba520900000a

import hashlib
import itertools as it


def password_cracker(hash_msg: str) -> str | None:
    for pwd_len in range(1, 6):
        for pwd_chars in it.product("abcdefghijklmnopqrstuvwxyz", repeat=pwd_len):
            pwd = "".join(pwd_chars)
            sha1_hash = hashlib.sha1(pwd.encode()).hexdigest()
            if sha1_hash == hash_msg:
                return pwd

    return None


def main():
    assert password_cracker("e6fb06210fafc02fd7479ddbed2d042cc3a5155e") == "code"
    assert password_cracker("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3") == "test"


main()
