from collections import defaultdict
from copy import deepcopy
from heapq import heappop, heappush
import random
import re


def parser(ins):
    f, t = ins.strip().split(" => ")
    return [t, f]


# This algorithm will only work intermittently (infinite loops are imminent).. and really isn't much of a solution at all!
def simulate_the_process(text, transformations):
    queue = [(0, text)]
    transformations = list(transformations.items())
    visited = set()
    while queue:
        steps, up_to_text = queue.pop()
        
        if up_to_text == "e":
            return steps

        random.shuffle(transformations)

        for t, transforms in transformations:
            steps_addition = len(re.findall(t, up_to_text))
            txt_clone = deepcopy(up_to_text)
            txt_clone = txt_clone.replace(t, transforms[0])
            if txt_clone not in visited:
                visited.add(txt_clone)
                queue.append((steps + steps_addition, txt_clone))
    return 0

    

    

with open("input.txt", "r") as input_:
    lines = input_.readlines()
    main_text = lines[-1]
    transformations = defaultdict(list)

    for from_, to in map(parser, lines[: -2]):
        transformations[from_].append(to)

    print(simulate_the_process(main_text, transformations))