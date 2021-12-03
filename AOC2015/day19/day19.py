from collections import defaultdict
from copy import deepcopy
def parser(ins):
    return ins.strip().split(" => ")

# 662 too high!
def simulate_the_process(allowables, text, transformations):
    visited = set()
    
    for l, transformations_ in transformations.items():
        for i in allowables[l]:
            for transform in transformations_:
                text[i] = transform
                tmp = ""
                if len(l) > 1:
                    tmp = text[i + 1]
                    text[i + 1] = ""
                    
                visited.add("".join(text))
                text[i] = l[0]
                if tmp:
                    text[i + 1] = tmp
    return len(visited)

with open("input.txt", "r") as input_:
    lines = input_.readlines()
    main_text = lines[-1]
    transformations = defaultdict(list)

    for from_, to in map(parser, lines[: -2]):
        transformations[from_].append(to)


    indices = defaultdict(list)

    for i, letter in enumerate(main_text):
        indices[letter].append(i)
        if i + 1 < len(main_text):
            indices[letter + main_text[i + 1]].append(i)

    print(simulate_the_process(indices, list(main_text), transformations))