from operator import mul
from functools import reduce
from itertools import combinations
INPUT = [1,3,5,11,13,17,19,23,29,31,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113]
arrange = []

TOTAL_SUM_DIV_3 = sum(INPUT) // 4



def find_optimal_arrangement():
    i = 1
    qe = float("inf")
    seen = False
    while i <= len(INPUT) and not seen:
        for group in combinations(INPUT, i):
            if sum(group) == TOTAL_SUM_DIV_3:
                for j in range(1, (len(INPUT) - len(group)) + 1):
                    for group_sec in combinations([v for v in INPUT if v not in group], j):
                        if sum(group_sec) == TOTAL_SUM_DIV_3:
                            for x in range(1, len(INPUT) - len(group) - len(group_sec) + 1):
                                for group_third in combinations([v for v in INPUT if v 
                                                                not in group_sec and v not in group], x):
                                    if sum([v for v in INPUT if v not in group and 
                                                                v not in group_sec and v not in group_third]) == TOTAL_SUM_DIV_3:
                                        return reduce(mul, group)
                                
                
        i += 1
    



print(find_optimal_arrangement())
