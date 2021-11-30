# It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
# It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
# It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
import re
from collections import defaultdict
VOWELS = "aeiou"
def is_nice(text):
    text = text.strip("\n")
    counts = defaultdict(set)
    for i, l in enumerate(text):
        if i + 1 < len(text):
            if i not in counts[text[i: i + 2]]:
                counts[text[i: i + 2]].add((i + 1))

    cond_1 = any([len(v) >= 2 for v in counts.values()])
    cond_2 = re.search(r"([a-z])\w\1", text) 
    
    
    return cond_2 and cond_1
    
    

with open("input.txt", "r") as input_:
    hashes = len(list(filter(is_nice, input_.readlines())))
    print(hashes)