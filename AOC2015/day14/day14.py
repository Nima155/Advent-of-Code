import re

LAST_SECOND = 2503
def parser(text):

    return list(map(int, re.findall(r"(?<=fly )\d+|\d+(?= seconds)", text.strip())))

with open("input.txt", "r") as input_:
    reindeer = list(map(parser, input_))
    cur_states = {}
    for s in range(1, LAST_SECOND + 1):
        for speed, time, t_t_rest in reindeer:
            
            rem = min(LAST_SECOND - s, time)
            
            rein = (speed, time, t_t_rest)
            if rein in cur_states and s == cur_states[rein][1]:
                cur_states[rein][0] += speed * rem
                cur_states[rein][1] = rem + s + t_t_rest
            elif rein not in cur_states: 
                cur_states[rein] = [speed * rem, rem + s + t_t_rest]
                
        
    print(max(map(lambda x: x[0], cur_states.values())))

    