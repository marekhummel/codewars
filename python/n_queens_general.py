def n_queens_general(n, mandatory=None):
    """Used for all three n queens katas"""
    # Init board
    board = Board(n)
    mandatory = mandatory or []
    for q_file, q_rank in mandatory:
        board.set_queen(q_file, q_rank)

    # Start loop
    next_fr = None
    while not board.is_filled():
        # Check if previous rank is continued or a new one is started
        if next_fr:
            start_file, rank = next_fr
            file_range = [f for f in range(start_file, n) if board.valid_queen(f, rank)]
        else:
            # Find rank with lowest possible files for next check
            taken_ranks = {f for _, f in board.queens}
            possible_files = [
                (r, [f for f in range(n) if board.valid_queen(f, r)])
                for r in range(n)
                if r not in taken_ranks
            ]

            rank, file_range = sorted(possible_files, key=lambda tpl: len(tpl[1]))[0]

        # Check rank
        if file_range:
            # Found new queen
            f = file_range[0]
            board.set_queen(f, rank)
            next_fr = None
        else:
            # All options exhausted (mandatory queen cant be removed)
            if len(board.queens) <= len(mandatory):
                return []

            # No queen found
            f, rank = board.remove_queen()
            next_fr = f + 1, rank

    return board.queens


class Board:
    def __init__(self, n) -> None:
        self.n = n
        self.queens = []
        self._files = 0
        self._diags_ne = 0
        self._diags_nw = 0

    def is_filled(self):
        return len(self.queens) == self.n

    def set_queen(self, file, rank):
        self._files |= 1 << file
        self._diags_ne |= 1 << self._to_ne_diag(file, rank)
        self._diags_nw |= 1 << self._to_nw_diag(file, rank)
        self.queens.append((file, rank))

    def remove_queen(self):
        file, rank = self.queens.pop()
        self._files &= ~(1 << file)
        self._diags_ne &= ~(1 << self._to_ne_diag(file, rank))
        self._diags_nw &= ~(1 << self._to_nw_diag(file, rank))
        return file, rank

    def valid_queen(self, file, rank):
        return (
            self._files & (1 << file) == 0
            and self._diags_ne & (1 << self._to_ne_diag(file, rank)) == 0
            and self._diags_nw & (1 << self._to_nw_diag(file, rank)) == 0
        )

    def _to_ne_diag(self, file, rank):
        return file + rank

    def _to_nw_diag(self, file, rank):
        return file + (self.n - rank - 1)
