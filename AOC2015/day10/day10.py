from itertools import groupby
initial_input = "1113222113"

cycle = 0
while cycle < 50: # change 50 to 40 for part 1 -- at least in my case
    initial_input = "".join([f"{len(list(v))}{k}"for k, v in groupby(initial_input)])
    cycle += 1

print(len(initial_input))