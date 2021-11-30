from collections import defaultdict
from heapq import heappop, heappush
# "London", "Dublin", "Belfast"
CITIES = [ "Faerun", "Arbre", "Tambi", "Straylight", "Norrath", "AlphaCentauri", "Snowdin", "Tristram"]
TOTAL_TV = ~(~0 << 8)
MAX_T = 1000000000008

def parser(line, dd):
    line = line.strip()
    cities, distance = line.split(" = ")
    s, d = cities.split(" to ")
    dd[s].append((d, int(distance)))
    dd[d].append((s, int(distance)))
    
    return (s, d), int(distance)

def iterative_bfs(maps, starting_k):
    queue = [(0, 1 << CITIES.index(starting_k), starting_k)]
    visited = {(starting_k, 1 << CITIES.index(starting_k), 0)}
    
    
    while queue:
        total_traversed_distance, total_visited, s_city = heappop(queue)
        if total_visited == TOTAL_TV:
            return total_traversed_distance
        if s_city in maps:
            for city, dist in maps[s_city]:        
                t_v = total_visited | (1 << CITIES.index(city))
                if (city, t_v, dist + total_traversed_distance) not in visited and not total_visited & (1 << CITIES.index(city)):
                    visited.add((city, t_v, dist + total_traversed_distance))
                    heappush(queue, (dist + total_traversed_distance, t_v, city))
    return MAX_T

with open("input.txt", "r") as input_:
    dd = defaultdict(list)
    lines = dict(map(lambda x: parser(x, dd), input_.readlines()))
    mxt = MAX_T
    for k in dd.keys():
        mxt = min(mxt, iterative_bfs(dd, k))
    print(mxt)
    