"""
https://adventofcode.com/2023/day/4
"""

import numpy as np
from tqdm import tqdm

with open("./9/input.txt", "r") as f:
    input = f.readlines()
# input = ["10 13 16 21 30 45"]

def parse_line(line):
    return  [int(x) for x in line.split(" ")]
    
def extrapolate(sensor):
    lasts = []    
    while sensor.count(0) != len(sensor):
        lasts.append(sensor[-1])
        sensor = list(np.diff(sensor))
    return sum(lasts)

def backward_extrapolate(sensor):
    firsts = []    
    while sensor.count(0) != len(sensor):
        firsts.append(sensor[0])
        sensor = list(np.diff(sensor))

    firsts.reverse()
    extrapolated = firsts[0]
    for first in firsts[1:]:
        extrapolated = first - extrapolated
    return extrapolated

# extrapolated = [extrapolate(parse_line(line)) for line in tqdm(input)]
# print("extrapolated: ", extrapolated)
# print("part1: ", sum(extrapolated))

backward_extrapolate = [backward_extrapolate(parse_line(line)) for line in tqdm(input)]
print("backward_extrapolate: ", backward_extrapolate)
print("part2: ", sum(backward_extrapolate))