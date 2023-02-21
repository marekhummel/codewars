# https://www.codewars.com/kata/59568be9cc15b57637000054

import math


def main():
    assert elder_age(8, 5, 1, 100) == 5
    assert elder_age(8, 8, 0, 100007) == 224
    assert elder_age(25, 31, 0, 100007) == 11925
    assert elder_age(5, 45, 3, 1000007) == 4323
    assert elder_age(31, 39, 7, 2345) == 1586
    assert elder_age(545, 435, 342, 1000007) == 808451

    # You need to run this test very quickly before attempting the actual tests :)
    assert elder_age(28827050410, 35165045587, 7109602, 13719506) == 5456283
    print("all good")


def elder_age(m, n, l, t):
    return elder_age_recursive(m, n, 0, 0, l, t)
    # elder_age_naive(m, n, l, t)


def elder_age_recursive(m, n, ox, oy, l, t):
    # Abort if negative or zero side length
    if m < 1 or n < 1:
        return 0

    # Let width be the bigger side
    (w, h) = (m, n) if m >= n else (n, m)

    # Find biggest power of two as box, because one line contains set range of integers
    biggest_pow_of_two = int(math.log2(w))
    box_width = 1 << biggest_pow_of_two
    box_height = min(h, box_width)

    # Compute the sum of one line in the box and then in total box
    xor_value = ox ^ oy
    box_min = max(xor_value - l, 0)
    box_max = max(xor_value + box_width - 1 - l, 0)
    box_donation_line = ((box_min + box_max) * (box_max - box_min + 1)) // 2
    box_donation = (box_donation_line % t) * (box_height % t) % t

    # Parts outside the box are computed recursively
    diff_w, diff_h = (w - box_width, h - box_height)
    bottom = elder_age_recursive(box_width, diff_h, ox, oy + box_height, l, t)
    right = elder_age_recursive(diff_w, box_height, ox + box_width, oy, l, t)
    remainder = elder_age_recursive(
        diff_w, diff_h, ox + box_width, oy + box_height, l, t
    )

    return (box_donation + bottom + right + remainder) % t


main()
