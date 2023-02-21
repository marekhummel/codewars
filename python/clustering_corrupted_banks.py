# https://www.codewars.com/kata/63e9a2ef7774010017975438


def get_reward(corrupted_banks: list[tuple[int, int]]):
    lend_dict = dict(corrupted_banks)

    reward = 0
    while lend_dict:
        start, current = lend_dict.popitem()
        cluster_sz = 1
        while current != start:
            cluster_sz += 1
            current = lend_dict.pop(current)

        reward += (1 << cluster_sz) * cluster_sz

    return reward


x = [(1, 3), (4, 6), (3, 1), (6, 7), (7, 4), (8, 8)]
print(get_reward(x))
