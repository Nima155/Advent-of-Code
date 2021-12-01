from collections import defaultdict
from itertools import permutations
# lets do a ring buffer!
def parser(arrangement, rels):
    split_up = arrangement.split()
    is_nega = split_up[2] == "lose"
    name, amount, dest_name = [split_up[0], int(split_up[3]) if not is_nega else -int(split_up[3]), split_up[-1][:-1]]
    rels[name][dest_name] = amount

def find_optimal_arrangement(rels):
    names = list(rels.keys())
    perms = list(permutations(rels, len(names)))
    optimal_happiness = 0
    for perm in perms:
        cost = 0
        for i, person in enumerate(perm):
            next_ = (i + 1) % len(perm)
            cost += rels[person][perm[next_]] + rels[perm[next_]][person]
        optimal_happiness = max(cost, optimal_happiness)
    return optimal_happiness




with open("input.txt", "r") as input_:
    lines = map(str.strip, input_.readlines())
    rels = defaultdict(dict)

    for l in lines:
        parser(l, rels)

    print(find_optimal_arrangement(rels))
    
    