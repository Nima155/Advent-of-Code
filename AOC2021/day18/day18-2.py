import math
from copy import deepcopy
def parse_line(line):
    i = 1
    def secondary(depth):
        nonlocal i
        ans, cur_num = [], 0
        while i < len(line):
            cur = line[i]
            if cur == "[":
                i += 1
                ans.append(secondary(depth + 1))
            elif cur == "]":
                if line[i - 1].isdigit():
                    ans.append(cur_num)
                return ans
            elif cur == "," and line[i - 1].isdigit():
                ans.append(cur_num)
                cur_num = 0
            elif cur.isdigit():
                cur_num *= 10
                cur_num += int(cur)

            i += 1
    return secondary(1)

def incrementer(ls, indx, v, reserve_indx):
    if isinstance(ls[indx], list):
        incrementer(ls[indx], reserve_indx, v, reserve_indx)
    else:
        ls[indx] += v
        

lsd = []
def cycle_through(ls, dpth, layer_cake, mode):
    i = 0
    while i < len(ls):

        if isinstance(ls[i], list) and not mode:
            if dpth + 1 == 5:
                l, r = ls[i] 
                look_for = deepcopy(ls)
                
                for listo in layer_cake:
                    indx = listo.index(look_for)
                    if l != -1 and i:
                        incrementer(ls, i - 1, l, -1)
                        l = -1
                    elif l != -1 and indx:
                        incrementer(listo, indx - 1, l, -1)
                        l = -1

                    if r != -1 and i + 1 < len(ls):
                        incrementer(ls, i + 1, r, 0)
                        r = -1
                    elif r != - 1 and indx + 1 < len(listo):
                        incrementer(listo, indx + 1, r, 0)
                        r = -1
                    look_for = listo

                del ls[i]
                ls.insert(i, 0)
                i = 0
                    
            else:
                new_cake = [ls, *layer_cake]
                bef = deepcopy(ls[i])
                cycle_through(ls[i], dpth + 1, new_cake, mode)
                if bef != ls[i]:
                    i = 0
                    continue
                
                    
            
        elif isinstance(ls[i], list) and mode:
            bef = deepcopy(ls[i])
            cycle_through(ls[i], dpth + 1, layer_cake, mode)
            if ls[i] != bef:
                return 

        
        if mode and isinstance(ls[i], int) and ls[i] >= 10:
            ls[i] = [ls[i] // 2, math.ceil(ls[i] / 2)]
            i = 0
            return 
            
            
            
            
        
        i += 1
            
    return []

   
def calculate_magnitude(ls):
    ans = 0
    for i in range(0, len(ls), 2):
        
        if isinstance(ls[i], list):
            ans = calculate_magnitude(ls[i]) * 3 + (ls[i + 1] if 
            isinstance(ls[i+1], int) else calculate_magnitude(ls[i + 1])) * 2
        elif i + 1 < len(ls):
            ans = 3 * ls[i] + (ls[i + 1] if isinstance(ls[i+1], int) else calculate_magnitude(ls[i + 1])) * 2
    return ans

# To check whether it's the right answer, the snailfish teacher only checks the magnitude of the final sum. The magnitude of a
#  pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element. The magnitude
#   of a regular number is just that number.

with open("input.txt", "r") as f:
    inp = f.read().splitlines()
    lss = [parse_line(p) for p in inp]
    biggest_mag = float("-inf")
    for i, v in enumerate(lss):
        for j in range(0, len(lss)):
            if j != i:
                ls = [deepcopy(v), deepcopy(lss[j])]
                ls_bef = deepcopy(ls)
                while 1:
                    cycle_through(ls, 1, [], 0)
                    cycle_through(ls, 1, [], 1)
                    if ls == ls_bef:
                        break
                    ls_bef = deepcopy(ls)

                biggest_mag = max(biggest_mag, calculate_magnitude(ls))
    
    print(biggest_mag)

    
    
    