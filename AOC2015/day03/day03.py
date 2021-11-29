with open("input.txt", "r") as input_:
    directions = input_.read()
    visited = {(0, 0)}
    
    cur_x, cur_y = 0, 0
    moves = { ">": (0, 1), "<": (0, -1), "v": (1, 0), "^": (-1, 0) }
    for dir in directions:
        ny, nx = moves[dir]
        cur_x += nx
        cur_y += ny
        visited.add((cur_x, cur_y))
    print(len(visited))
        