from functools import lru_cache
from collections import deque
from copy import deepcopy
INPUT = [1,3,5,11,13,17,19,23,29,31,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113]
arrange = []

TOTAL_SUM_DIV_3 = sum(INPUT) // 3



def find_optimal_arrangement():
    queue = deque([[[0, 0, 1], [0, 0, 1], [0, 0, 1]]])
    
    while queue and INPUT:
        groups = queue.popleft()
        
        if all(v[0] == TOTAL_SUM_DIV_3 for v in groups):
            print(groups)
            continue

        n = INPUT.pop()    
        
        for i in range(3):
            if groups[i][0] + n <= TOTAL_SUM_DIV_3:
                
                groupz = deepcopy(groups)
                groupz[i][0] += n
                groupz[i][1] += 1
                groupz[i][2] *= n
                queue.append(groupz)
    print(sorted(queue))




print(find_optimal_arrangement())
