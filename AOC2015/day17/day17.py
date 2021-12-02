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


def total_num_combinations(taken, visited):
    if taken > 150:
        return 0
    if taken == 150:     
        return 1
    ans = 0
    for i, v in enumerate(INPUT):
        if i not in visited:
            visited += (i, )
            ans += total_num_combinations(taken + v, visited)
            
    return ans

print(total_num_combinations(0, ()))