from copy import deepcopy
with open("input.txt", "r") as input_:
    ls = list(map(lambda x: list(x.strip()), input_.readlines()))
    # diagonals
    for i in range(100): # steps
        light_copy = deepcopy(ls)
        for y in range(100):
            for x in range(100):
                light_status = ls[y][x]
                ons = 0
                for j in range(-1, 2):
                    for v in range(-1, 2):
                        if j != 0 or v != 0:
                            ny, nx = y + j, x + v
                            if ny >= 0 and ny < 100 and nx >= 0 and nx < 100:
                                ons += ls[ny][nx] == "#"
                # if (y, x) not in { (0, 0), (0, 99), (99, 0), (99, 99) }:
                if light_status == "#" and ons not in [2, 3]:
                    light_copy[y][x] = "."
                elif light_status == "." and ons == 3:
                    light_copy[y][x] = "#"
        ls = light_copy
    print(sum(sum(map(lambda x: x == '#', v)) for v in ls ))
# quarantine plan.. vaccine card
# not staying with daughter 

                            