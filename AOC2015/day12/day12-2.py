import re
with open("input.txt", "r") as input_:
    input_ = re.sub(r"red", "000000", input_.read())
    important_matches = re.findall(r"\{|[-]?\d+|\}|\[|\]", input_)
    bracks, squares, red_alert = [], [], False
    for mt in important_matches:
        if mt == "{":
            bracks.append("{")
        elif mt == "[":
            squares.append("[")
        elif mt == "}":
            bracks.pop()
        elif mt == "]":
            squares.pop()
        red_alert = mt.isdigit() and mt == "000000"
        

    # print(sum(map(int, re.findall("[-]?\d+", input_.read()))))
    