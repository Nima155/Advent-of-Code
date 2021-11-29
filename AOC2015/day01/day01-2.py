from collections import Counter

with open("input.txt", "r") as input:
    all_floors = input.read()
    b = 0
    for indx, v in enumerate(all_floors):
        b += -1 if v == ")" else 1
        if b < 0:
            print(indx + 1)
            break
    