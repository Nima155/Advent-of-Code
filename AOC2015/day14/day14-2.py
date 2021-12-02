import re

LAST_SECOND = 2503
def parser(text):

    return list(map(int, re.findall(r"(?<=fly )\d+|\d+(?= seconds)", text.strip())))

with open("input.txt", "r") as input_:
    reindeer = list(map(parser, input_))
    cur_states = {}
    resevoir = [0] * len(reindeer)
    for s in range(1, LAST_SECOND + 1):
        mx = -1
        for i, (speed, time, t_t_rest) in enumerate(reindeer):
            rem = min(LAST_SECOND - s, time)
            rein = (speed, time, t_t_rest)
            if rein in cur_states and s == cur_states[rein][1]:
                resevoir[i] += rem - 1
                if resevoir[i] > 0:
                    cur_states[rein][0] += speed
                cur_states[rein][1] = rem + s + t_t_rest
            elif rein not in cur_states: 
                resevoir[i] = rem - 1
                cur_states[rein] = [speed, rem + s + t_t_rest, 0]
            else:
                if resevoir[i] > 0:
                    cur_states[rein][0] += speed
                    resevoir[i] -= 1    

            mx = max(mx, cur_states[rein][0])
    
        cur_states = { k: (v if v[0] != mx else [v[0], v[1], v[2] + 1]) for k, v in cur_states.items() }
        # print(cur_states)
    # print(cur_states)
    print(max(map(lambda x: x[2], cur_states.values())))

    