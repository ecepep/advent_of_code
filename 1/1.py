"""
https://adventofcode.com/2023/day/1
"""

import os
import re

with open("./1/input.txt", "r") as f:
    input = f.readlines()

def to_digit(l):
    l = re.sub("one", "one1one", l);
    l = re.sub("two", "two2two", l);
    l = re.sub("three", "three3three", l);
    l = re.sub("four", "four4four", l);
    l = re.sub("five", "five5five", l);
    l = re.sub("six", "six6six", l);
    l = re.sub("seven", "seven7seven", l);
    l = re.sub("eight", "eight8eight", l);
    l = re.sub("nine", "nine9nine", l);
    return l

sum1 = 0
sum2 = 0
for l in input:
    l2 = to_digit(l)

    d1 = re.sub("[^\d]", "", l);
    sum1 += int(d1[0]+d1[-1])
    d2 = re.sub("[^\d]", "", l2);
    sum2 += int(d2[0]+d2[-1])

print("part1: ", sum1)
print("part2: ", sum2)