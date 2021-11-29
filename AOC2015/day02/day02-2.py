from functools import reduce
from operator import mul
def needed_supplies(points):
    f_m, s_m, _ = sorted(points)
    
    return  f_m * 2 + s_m * 2 + reduce(mul, points)

with open("input.txt", "r") as input_:
    inp = input_.read().splitlines()
    print(sum(map(lambda x: needed_supplies(list(map(int, x.split("x")))), inp)))

    
    
    