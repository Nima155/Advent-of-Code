from math import sqrt
# 33100000
GOAL = 33100000


def divisors(n):
    divisors = set()
    for i in range(1, int(sqrt(n)) + 1):
        if n % i == 0:
            divisors.add(n // i)
            divisors.add(i)
    
    return divisors

def minimum_to_goal():
    i = 0
    while 1:
        if sum(map(lambda x:  x * 11 if x * 50 >= i else 0, divisors(i))) >= GOAL:
            return i
        i += 1
    
    
        



print(minimum_to_goal())
    
        



