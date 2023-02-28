# https://www.codewars.com/kata/54207f9677730acd490000d1

import hashlib


def pass_hash(pwd):
    return hashlib.md5(pwd.encode()).hexdigest()


def main():
    assert pass_hash("password") == "5f4dcc3b5aa765d61d8327deb882cf99"
    assert pass_hash("abc123") == "e99a18c428cb38d5f260853678922e03"


main()
