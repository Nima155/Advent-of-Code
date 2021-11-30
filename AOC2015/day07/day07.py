from operator import and_, or_, lshift, rshift
from collections import defaultdict
ops = {"OR": or_, "AND": and_, "LSHIFT": lshift, "RSHIFT": rshift}

def parser(line, gates):
    gate, destination = line.split(" -> ")
    gate = gate.split()
    if len(gate) == 3:
        try:
            arg_1 =  int(gate[0]) if gate[0].isdigit() else gates[gate[0]]
            arg_2 = int(gate[2]) if gate[2].isdigit() else gates[gate[2]]
            # print(gate)
            gates[destination] = min(65535, ops[gate[1]](arg_1, arg_2))
        except: 
            pass
    elif len(gate) == 2:
        try:
            arg_1 =  int(gate[1]) if gate[1].isdigit() else gates[gate[1]]
            gates[destination] = max(65536 + ~arg_1, 0)
        except:
            pass
    else:
        try:
            arg_1 = int(gate[0]) if gate[0].isdigit()  else gates[gate[0]]
            gates[destination] = min(arg_1, 65535)
        except:
            pass

with open("input.txt", "r") as input_:
    gates = {}
    input_ = sorted(input_.readlines(), key=lambda x:len(x))
    
    while "a" not in gates: 
        for l in input_:
                parser(l.strip("\n"), gates)
        
    print(gates["a"])