# https://www.codewars.com/kata/5993c1d917bc97d05d000068
# https://rpruim.github.io/m252/S19/from-class/models-of-computation/dfanfa-to-regular-expression.html
# https://stackoverflow.com/questions/21897554/design-dfa-accepting-binary-strings-divisible-by-a-number-n/52917910#52917910

from itertools import combinations, product


def regex_divisible_by(n):
    # Setup DFA
    delta: dict[tuple[str, str], str] = {("qs", ""): "q0", ("q0", ""): "qe"}
    for v, b in product(range(n), [0, 1]):
        target = (v * 2 + b) % n
        delta[(f"q{v}", str(b))] = f"q{target}"
    delta = unify_edges(delta)

    # Reduce DFA
    for r in (f"q{i}" for i in range(n - 1, -1, -1)):
        # Find loops on current node and remove
        loop = next((s for (q, s), qp in delta.items() if q == r and qp == r), None)
        loop_rgx = (f"(?:{loop})*" if len(loop) > 1 else f"{loop}*") if loop else ""

        if loop:
            del delta[(r, loop)]

        # Simplify A -> R -> B edges to a A -> B edge, including the loop
        incoming = [(q, s, qp) for (q, s), qp in delta.items() if qp == r]
        outgoing = [(q, s, qp) for (q, s), qp in delta.items() if q == r]

        for (qi, si, _), (_, so, qpo) in product(incoming, outgoing):
            delta[(qi, f"{si}{loop_rgx}{so}")] = qpo

        for q, s, _ in incoming + outgoing:
            del delta[(q, s)]

        # Unify edges with same source and target
        delta = unify_edges(delta)

    # Return remaining edge from qs to qe
    return f"^{delta.popitem()[0][1]}$"


def unify_edges(delta):
    merge = {
        (q1, s1, s2, qp1)
        for ((q1, s1), qp1), ((q2, s2), qp2) in combinations(delta.items(), 2)
        if q1 == q2 and qp1 == qp2 and s1 != s2
    }
    for q, s1, s2, qp in merge:
        del delta[(q, s1)]
        del delta[(q, s2)]
        delta[(q, f"(?:{s1}|{s2})")] = qp

    return delta


print(regex_divisible_by(4))
