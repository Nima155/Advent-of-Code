import re
# 787 too low!
def fn(matches):
    for j in range(4):
        mt = matches.group(j)
        if mt and mt.startswith("\\x"):
            return "zzzzz"
        elif mt and ( mt.startswith("\\\"") or mt.startswith("\\\\")):
            return "xxxx"
        else:
            return "."
        
            
    pass

with open("input.txt", "r") as input_:
    sm = 0
    for l in input_.readlines():
        l = l.strip()
        in_code = len(l) 
        in_code_encoded = 4 + len(re.sub(r'(\\[x][0-9a-fA-F]{2})|(\\")|(\\\\)|(.)', fn, l))
        sm += in_code_encoded - in_code
    print(sm)
