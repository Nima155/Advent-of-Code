import re

def sue_parser(l):
    l = l.strip()
    el_dicto = {}
    for it, amount in list(map(lambda entry: entry.split(": "), re.sub(r"Sue \d+: ", "", l).split(", "))):
        el_dicto[it] = int(amount)
    return el_dicto

AUNT_SUES_STUFF = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1
}

"""
cats and trees greater than many
pomeranians and goldfish fewer than many
"""

def comparator(k, data, op):
    if op:
        if k in data and AUNT_SUES_STUFF[k] <= data[k]:
                return False
    else:
        if k in data and AUNT_SUES_STUFF[k] >= data[k]:
                return False
    return True



# 442 is too high!
with open("input.txt", "r") as input_:
    parsed_data = list(map(sue_parser, input_.readlines()))
    mx, mx_r = 0, 0
    for i, data in enumerate(parsed_data):
        c, t, p, g = [comparator("cats", data, 0), comparator("trees", data, 0), 
                      comparator("pomeranians", data, 1), comparator("goldfish", data, 1)]
        if c and t and p and g:
            mt = 0

            for k, v in data.items():
                if k in { "cats", "trees", "goldfish", "pomeranians" } or AUNT_SUES_STUFF[k] == v:
                    mt += 1

            if mt > mx:
                # print(data)
                mx = mt
                mx_r = i + 1
    print(mx_r)


            
        
    
