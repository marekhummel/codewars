# https://www.codewars.com/kata/561bed6a31daa8df7400000e

from n_queens_general import n_queens_general


def main():
    print(queens("c3", 8))


def queens(position, size):
    file = ord(position[0]) - ord("a")
    rank = int(position[1].replace("0", "10")) - 1

    queens = n_queens_general(size, [(file, rank)])

    return ",".join(
        chr(x + ord("a")) + str(y + 1).replace("10", "0") for x, y in queens
    )


main()
