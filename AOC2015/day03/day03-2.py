with open("input.txt", "r") as input_:
    directions = input_.read()
    visited = {(0, 0)}
    santa_x, santa_y, robot_x, robot_y = 0, 0, 0, 0
    turn = 0
    moves = { ">": (0, 1), "<": (0, -1), "v": (1, 0), "^": (-1, 0) }
    for dir in directions:
        ny, nx = moves[dir]
        if not turn:
            santa_x += nx
            santa_y += ny
            visited.add((santa_x, santa_y))
        else:
            robot_x += nx
            robot_y += ny
            visited.add((robot_x, robot_y))
        turn ^= 1
    print(len(visited))
        