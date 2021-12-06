def parser(txt):
    txt = txt.strip()
    return txt.split(" ")



def inc(registers, register_name):
    registers[ord(register_name) - ord('a')] += 1

def tpl(registers, register_name):
    registers[ord(register_name) - ord('a')] *= 3

def hlf(registers, register_name):
    registers[ord(register_name) - ord('a')] /= 2


    
    

    

def jump_handler(instruction, registers):
    if instruction[0] == "jio":
        return registers[ord(instruction[1][:-1]) - ord('a')] == 1
    else:
        return registers[ord(instruction[1][:-1]) - ord('a')] % 2 == 0


INST = { "inc": inc, "tpl": tpl, "hlf": hlf}

with open("input.txt", "r") as input_:
    stripped_up = list(map(parser, input_.readlines()))
    registers, i = [1, 0], 0 # change 1 to 0 for answer to part 1 
    while i < len(stripped_up) and i >= 0:
        instruction = stripped_up[i]
        if instruction[0] in ["jie", "jmp", "jio"]:
            cond =  instruction[0] == "jmp" or jump_handler(instruction, registers)
            if cond: 
                i += int(instruction[-1])
                continue
            
        else:
            INST[instruction[0]](registers, instruction[1])
        i += 1
    print(registers)