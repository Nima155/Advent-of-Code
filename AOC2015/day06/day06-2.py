def splitter(line):
    
    if line.startswith("toggle"):
        action, starting, _, ending = line.split()
    else:
        _, action, starting, _, ending = line.split()
    return [action, list(map(int, starting.split(","))), list(map(int, ending.split(",")))]

def perform_action(grid, action, starting, ending):
    s_y, s_x = starting
    e_y, e_x = ending

    for y in range(s_y, e_y + 1):
        for x in range(s_x, e_x + 1):
            if action == "toggle":
                grid[y][x] += 2
            elif action == "on":
                grid[y][x] += 1
            else:
                grid[y][x] = max(0, grid[y][x] - 1)



with open("input.txt", "r") as input_:
    instructions = list(map(splitter, input_.readlines()))
    imaginary_grid = [[0] * 1000 for _ in range(1000)]
    for action, starting, ending in instructions:
        perform_action(imaginary_grid, action, starting, ending)
    print(sum([sum(v) for v in imaginary_grid]))
