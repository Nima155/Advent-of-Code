from collections import Counter

with open("input.txt", "r") as input:
    counter = Counter(input.read())
    print(counter.get("(", 0) - counter.get(")", 0))