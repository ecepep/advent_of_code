"""
https://adventofcode.com/2023/day/4
"""

import re

with open("./4/input.txt", "r") as f:
    input = f.readlines()

own_cnt = [] # for each card, how many number we own
for card in input:
    card_nb, winning_str, owned_str = re.split("[:|]", card)
    winning = re.findall("(\d+)", winning_str)
    owned = re.findall("(\d+)", owned_str)
    
    # if len(owned) > len(set(owned)):
    #     raise Exception("not unique %s" % owned_str)
    
    owned = set(owned)
    winning = set(winning)
    intersect = owned.intersection(winning)

    own_cnt.append(len(intersect))

# part1
points = [0 if x == 0 else pow(2, x-1) for x in own_cnt]
print("part1: ", sum(points))

# part2
card_cnt = [1]*len(own_cnt) # for each card, how many was drawn
for i in range(len(own_cnt)-1):
    owned = own_cnt[i]
    drawn = card_cnt[i]
    
    for j in range(i+1, min(len(own_cnt), i+1+owned)):
        card_cnt[j] += drawn
print(own_cnt)
print(card_cnt)
print("part2: ", sum(card_cnt))
