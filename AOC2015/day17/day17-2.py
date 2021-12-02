from copy import deepcopy
from functools import lru_cache

INPUT = [50,
44,
11,
49,
42,
46,
18,
32,
26,
40,
21,
7,
18,
43,
10,
47,
36,
24,
22,
40]

def total_num_combinations(taken, visited, rem_turns):
    if taken == 150:     
        return 1
    if taken > 150 or rem_turns == 0:
        return 0
    ans = 0
    for i, v in enumerate(INPUT):
        if i not in visited:
            visited += (i, )
            ans += total_num_combinations(taken + v, visited, rem_turns - 1)     
    return ans


def min_turns(taken, visited, turn):
    if taken > 150:
        return float("inf")
    if taken == 150:     
        return turn
    ans = float("inf")
    for i, v in enumerate(INPUT):
        if i not in visited:
            visited += (i, )
            ans = min(ans, min_turns(taken + v, visited, turn + 1))
    return ans


print(total_num_combinations(0, (), min_turns(0, (), 0)))