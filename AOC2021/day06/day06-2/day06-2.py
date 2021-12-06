from functools import lru_cache
DAYS = 256


# def 
@lru_cache(maxsize=None)
def simulate_process(day, count_down):
    if day >= 256:
        return 1
    if count_down == 0:
        return simulate_process(day + 7, 0) + simulate_process(day + 9, 0)
    
    return simulate_process(day + 1, count_down - 1)


with open("../input.txt", "r") as input_:
    state = list(map(int, input_.read().split(",")))
    sm = 0
    for n in state:
        sm += simulate_process(0, n)
    print(sm)


