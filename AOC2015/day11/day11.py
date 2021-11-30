import re
from string import ascii_lowercase as a_l

PATTERN = "|".join(a_l[i: i + 3] for i in range(len(a_l) - 2))
TWO_PATTERN = "|".join(f"{v}{v}" for v in a_l)

def is_valid(password):
    threes = re.search(PATTERN, password)
    prohibs = any(v in "iol" for v in password)
    two_twos = re.findall(TWO_PATTERN, password)
    
    return threes and not prohibs and len(two_twos) >= 2



password = "hxbxxzaa" # get output from part 1 and put it here for answer to part 2
# but remember to change the inital password so that its not valid so that the while loop can work
last_indx = len(password) - 1
cur_indx = last_indx
while not is_valid(password):
    to_list = list(password)
    if to_list[cur_indx] == "z":
        to_list[cur_indx] = 'a'
        cur_indx -= 1
    else:
        to_list[cur_indx] = chr(ord(to_list[cur_indx]) + 1)
        while to_list[cur_indx] in "iol":
            to_list[cur_indx] = chr(ord(to_list[cur_indx]) + 1)
        if cur_indx != last_indx:
            cur_indx = last_indx
    password = "".join(to_list)
    
print(password)
    
        

# cur_indx = 

