"""
https://adventofcode.com/2023/day/3

A bit dirty but string parsing ain't no fun
"""

import re
import numpy as np

with open("./3/input.txt", "r") as f:
    input = f.read().split("\n")[:-1];

print(len(input), len(input[0]))

# part1
if False:
    """
    Search if any character in contact with the number at the row row between the column col_start and col_end is a symbol
    """
    def is_near_symbol(number, input, row, col_start, col_end):
        for r in range(max(row-1, 0), min(row+2, len(input))):
            for c in range(max(col_start-1, 0), min(col_end+2, len(input[r]))):
                char = input[r][c]
                is_symbol = re.search("[^\\d\\.]", char)
                if is_symbol:
                    return True

    near_symbol_numbers = []
    number = ""
    col_start = None

    for row in range(len(input)):
        if len(input)!=140:
            raise Exception("Expect fixed input size")
        for col in range(len(input[row])):            
            if len(input[row])!=140:
                raise Exception("Expect fixed input size")
            char = input[row][col]
            is_digit = re.search("\\d", char)
            if is_digit:
                number += char
            if is_digit and col_start is None:
                col_start = col
            
            eol = col == len(input[row])-1
            if not is_digit or eol:
                if number != "" and is_near_symbol(number, input, row, col_start, col if eol else col-1):
                    near_symbol_numbers.append(int(number))
                number = ""
                col_start = None

    print("part1: ", sum(near_symbol_numbers))

# part2
def find_part(input, row, col):
    if row < 0 or row >= len(input) or col < 0 or col >= len(input[0]) :
        return None
    
    isdigit = lambda r,c: re.search("\\d",  input[r][c])
    
    if not isdigit(row, col):
        return None
    
    part = input[row][col]
    c = col
    while (c-1 >= 0 and isdigit(row, c-1)):
        part = input[row][c-1] + part
        c -= 1
    c = col
    while (c+1 < len(input[0]) and isdigit(row, c+1)):
        part = part + input[row][c+1]
        c += 1
    return part
    
def find_parts(input, row, col):    
    parts = []
    above = find_part(input, row-1, col)
    parts.append(above)

    if above is None:
        parts.append(find_part(input, row-1, col -1))
        parts.append(find_part(input, row-1, col +1))
   
    below = find_part(input, row+1, col)
    parts.append(below)

    if below is None:
        parts.append(find_part(input, row+1, col -1))
        parts.append(find_part(input, row+1, col +1))

    # sides 
    parts.append(find_part(input, row, col-1))
    parts.append(find_part(input, row, col+1))

    parts = [g for g in parts if not (g is None)]
    return parts

sum_gear = 0
for row in range(len(input)):
    for col in range(len(input[row])):
        char = input[row][col]
        if char != "*":
            continue

        parts = find_parts(input, row, col)
        if len(parts) == 2:
            gear = int(parts[0])*int(parts[1])
            sum_gear += gear
print("part2: ", sum_gear)

