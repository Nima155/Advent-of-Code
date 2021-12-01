import json

def recursive_dive(maps, is_list):
    tots = 0
    
    for v in maps:
        if v == "red" and not is_list:
            return 0
        if isinstance(v, dict):           
            tots += recursive_dive(v.values(), False)
        elif isinstance(v, list):
            tots += recursive_dive(v, True)
        elif isinstance(v, int):
            tots += v
    return tots

with open("input.txt", "r") as input_:
    important_matches = json.load(input_)  
    tots = 0
    if isinstance(important_matches, list):
        tots = recursive_dive(important_matches, True)
    else:
        tots = recursive_dive(important_matches.values(), False)

    print(tots)