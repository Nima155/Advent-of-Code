import re
# 787 too low!
with open("input.txt", "r") as input_:
    sm = 0
    for l in input_.readlines():
        l = l.strip()
        in_memory = len(re.findall(r'\\[x][0-9a-fA-F]{2}|\\"|\\\\|.', l)) - 2
        in_code = len(l)
        sm += in_code - in_memory 
    print(sm)
