# z3 is pretty cool!
import z3 

def abs(x):
    return z3.If(x >= 0,x,-x)

with open("../input.txt", "r") as f:
    points = []
    lines = f.readlines()
    for l in lines:
       coords, radius = l.split(", ")
       points.append(list(map(int, coords[5:len(coords) - 1].split(","))) + [int(radius[2:])])

    optimizer = z3.Optimize()

    x, y, z = z3.Ints("x y z")
    
    assertions = z3.Sum([z3.If(abs(x - xx) +  abs(y - yy) +  abs(z - zz) <= r, 1, 0) for xx, yy, zz, r in points])
    
    v = optimizer.maximize(assertions)
    
    minox = optimizer.minimize(x)
    minoy = optimizer.minimize(y)
    minoz = optimizer.minimize(z)
    # Gives the right answer after about a minute... just add up the values returned by the 2nd call to print and voile!
    if optimizer.check() == z3.sat:
        print(v.value())
        print(minox.value(), minoz.value(), minoy.value())
        
    

    
        
        
    


