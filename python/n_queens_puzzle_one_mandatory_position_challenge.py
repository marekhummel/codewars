from n_queens_general import n_queens_general


def main():
    print(solve_n_queens(8, (2, 2)))


def solve_n_queens(n, fixed_queen):
    queens = n_queens_general(n, [fixed_queen[::-1]])
    if not queens:
        return None

    sorted_queens = [f for f, _ in sorted(queens, key=lambda tpl: tpl[1])]
    return "\n".join("." * f + "Q" + "." * (n - f - 1) for f in sorted_queens) + "\n"


main()
