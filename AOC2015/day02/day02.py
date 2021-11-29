def needed_supplies(points):
    l, w, h = points
    ll, ww, hh = 2 * l * w, 2 * w * h, 2 * h *l
    return  ll + ww + hh + min(ll, ww, hh) // 2

with open("input.txt", "r") as input_:
    inp = input_.read().splitlines()
    print( sum(map(lambda x: needed_supplies(list(map(int, x.split("x")))), inp)))

    
    
    
