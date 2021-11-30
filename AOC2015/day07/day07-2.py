from operator import and_, or_, lshift, rshift
from collections import defaultdict
ops = {"OR": or_, "AND": and_, "LSHIFT": lshift, "RSHIFT": rshift}

def parser(line, gates):
    gate, destination = line.split(" -> ")
    gate, expr = gate.split(), -1
    try:
        if len(gate) == 3:
                arg_1 =  int(gate[0]) if gate[0].isdigit() else gates[gate[0]]
                arg_2 = int(gate[2]) if gate[2].isdigit() else gates[gate[2]]
                expr = min(65535, ops[gate[1]](arg_1, arg_2))

        elif len(gate) == 2:
            arg_1 =  int(gate[1]) if gate[1].isdigit() else gates[gate[1]]
            expr = max(65536 + ~arg_1, 0)
        else:
            arg_1 = int(gate[0]) if gate[0].isdigit()  else gates[gate[0]]
            expr = min(arg_1, 65535)        
    except:
        pass
    if destination not in gates and expr != -1:
            gates[destination] = expr


with open("input.txt", "r") as input_:
    gates = {}
    input_ = sorted(input_.readlines(), key=lambda x:len(x))
    turn = 0
    while 1: 
        for l in input_:
                parser(l.strip("\n"), gates)
        if "a" in gates:
            if turn == 1:
                break
            gates = { "b": gates["a"]}
            turn += 1
        
    print(gates["a"])