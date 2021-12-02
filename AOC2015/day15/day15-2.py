from itertools import permutations
from copy import deepcopy
def parser(ingredient):
    ingredient = ingredient.strip()
    _, values = ingredient.split(": ")
    return list(map(lambda x: int(x.split()[1]), values.split(", ")))

hundos = []
def up_to_100(rem, number, up_to = []):
    if number == 0 and rem == 0:
        hundos.append(up_to)
        return 
    elif number == 0:
        return 
    
    for i in range(1, rem + 1):
        up_to_100(rem - i, number - 1, [*up_to, i])
        
        
def scorer(items):
    mx = 1
    
    for item in items:
        for comb in hundos:
            tmp, cal_is_500 = 1, False
            ls = list(map(lambda x: [v * x[1] for v in x[0]], zip(item, comb)))
            for m in range(4, -1, -1):
                if m == 4 and sum(v[m] for v in ls) == 500:
                    cal_is_500 = True
                elif m == 4:
                    break
                tmp *= max(0, sum(v[m] for v in ls) if m != 4 else 1)
            mx =  max(tmp, mx) if cal_is_500 else mx
    return mx
            
            


with open("input.txt", "r") as input_:
    parsed_data = list(map(parser, input_.readlines()))
    mx = 0
    for j in range(1, len(parsed_data) + 1):
        up_to_100(100, j)
        perms = list(permutations(parsed_data, j))
        mx = max(mx, scorer(perms))
        hundos = []
    print(mx)
                

    
    
            
        
