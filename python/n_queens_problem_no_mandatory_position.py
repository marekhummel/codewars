# https://www.codewars.com/kata/52cdc1b015db27c484000031

from n_queens_general import n_queens_general


def main():
    # print(nQueen(150))
    print(nQueen_clever(150))


def nQueen(n):
    queens = n_queens_general(n)
    return [x for x, _ in sorted(queens, key=lambda tpl: tpl[1])]


def nQueen_clever(n):
    # https://en.wikipedia.org/wiki/Eight_queens_puzzle#Existence_of_solutions
    if n in [2, 3]:
        return []
    r, odds, evens = n % 6, list(range(1, n, 2)), list(range(0, n, 2))
    if r == 2:
        evens[:2] = evens[:2][::-1]
        evens.append(evens.pop(2))
    if r == 3:
        odds.append(odds.pop(0))
        evens.extend(evens[:2])
        del evens[:2]
    return odds + evens


main()
