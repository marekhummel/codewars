# https://www.codewars.com/kata/54da539698b8a2ad76000228


def is_valid_walk(walk: list[str]):
    if len(walk) != 10:
        return False

    dirs = {"n": 0, "s": 0, "e": 0, "w": 0}
    for step in walk:
        if step not in walk:
            raise ArgumentError()
        dirs[step] += 1

    return dirs["n"] == dirs["s"] and dirs["e"] == dirs["w"]
