# It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
# It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
# It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
import re
from itertools import groupby
VOWELS = "aeiou"
def is_nice(text):
    cond_3 = len(re.findall(r"ab|cd|pq|xy", text)) == 0
    cond_1 = len(re.findall(f"[{VOWELS}]", text)) >= 3
    cond_2 = any([len(list(i)) >= 2 for _, i in groupby(text)])
    
    return cond_1 and cond_2 and cond_3

with open("input.txt", "r") as input_:
    hashes = len(list(filter(is_nice, input_.readlines())))
    print(hashes)