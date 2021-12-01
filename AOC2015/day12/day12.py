import re
with open("input.txt", "r") as input_:
    print(sum(map(int, re.findall("[-]?\d+",input_.read()))))