"""
https://adventofcode.com/2023/day/10
#@todo some dirty code, no time to clean
#@todo check if someone as found something more elegant
"""


import numpy as np
from math import ceil
import pandas as pd

TEST_INPUT = False

if TEST_INPUT:
    # test input for part 1 & 2 (both F start)
    # part1 input = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ" 
    # input = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ..."
    # last_dir_out = (+1, 0) # see mark_outer_side_of_loop
    
    input = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L"
    last_dir_out = (-1, 0) # see mark_outer_side_of_loop
else:
    with open("./10/input.txt", "r") as f:
        input = f.read()
    out_is_right = False # we don't know before testing
    last_dir_out = (0, 1 if out_is_right else -1) # see mark_outer_side_of_loop

input = [[char for char in line] for line in input.split("\n")]
input = np.array(input)

start =  np.where(input =='S')
start = (start[0][0], start[1][0]) # todo
# input[start] = '|' # no need to replace it with current input and test input
print("start: ", start)


mask = np.zeros(input.shape) # a mask of the input for part 2, 0 => inside the loop, 1=> the loop, 2 => outside the loop, 3 )> junk_pipe

"""
Mark in the mask, tiles which are cornering the loop (outside)
It solves this problem: "In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed!"

I am walking through the loop always remembering if the 2s are left or right of a '|' and up or down a '-'. When we turn direction changes accordingly.
At each step of the walkthrough I mark the 2

@note I wont try to know what side of the loop  are the 2 at start. It is either or. I have one chance out of two and as many guesses as I which. Therefore, I will just answer both. Smart engineering sometimes involves cheating through your problems ;)
"""
def mark_outside_of_loop(curr, input):
    # @todo cleanup those global, global are ugly
    global last_dir_out # last direction of 0: left, right, up or down. Init manually at start (see note)
    global mask

    # Turn in its outer position
    TURN_TYPE = {
        "F": (-1,-1),
        "7": (-1,+1),
        "L": (+1,-1),
        "J": (+1,+1)
    }

    out_tiles= [] # List of relativ adjacent position to curr that may be outside the loop if not the loop
    char = input[curr]
    if char == '-':
        out_tiles = [last_dir_out]
    elif char == '|':
        out_tiles = [last_dir_out] 
    else:
        turn = TURN_TYPE[char]
        if last_dir_out[0] == 0: # 2s are left or right, turn from vertical to horizontal
            is_outer = last_dir_out[1] == turn[1] # which side of the turn are the 2s
            if is_outer:
                out_tiles = [turn, (turn[0], 0), (0, turn[1])] 
                last_dir_out = (turn[0], 0)
            else: # inner turn
                out_tiles = [(-1* turn[0],-1* turn[1])]
                last_dir_out = (-1* turn[0], 0)
        else:
            is_outer = last_dir_out[0] == turn[0] # which side of the turn are the 2s
            if is_outer:
                out_tiles = [turn, (turn[0], 0), (0, turn[1])] 
                last_dir_out = (0, turn[1])
            else: # inner turn
                out_tiles = [(-1* turn[0],-1* turn[1])]
                last_dir_out = (0, -1* turn[1])

    # print("char: ", char, " out_tiles: ", out_tiles)

    # mark out tiles as 2s
    for out in out_tiles:
        out_index = (curr[0] + out[0], curr[1] + out[1])
        if out_index[0]<0 or out_index[0]>= len(input) or out_index[1]<0 or out_index[1]>= len(input[0]):
            continue 

        if input[out_index] == '.':
            mask[out_index] = 2
            input[out_index] = "2"  # for debug
    return

"""
Take the next step in the loop
"""
def walkthrough(prev, curr):
    global input

    if curr[0] < 0 or curr[0] >= len(input) and curr[1] < 0 and curr[1] >= len(input[0]):
        raise Exception("Out of bound") # last step went out of bound we stop

    con = []
    if input[curr] == "7":
        con = ((curr[0] + 1, curr[1] + 0), (curr[0] + 0, curr[1] - 1))
    if input[curr] == "F":
        con = ((curr[0] + 1, curr[1] + 0), (curr[0] + 0, curr[1] + 1))
    if input[curr] == "J":
        con = ((curr[0] - 1, curr[1] + 0), (curr[0] + 0, curr[1] - 1))
    if input[curr] == "L":
        con = ((curr[0] - 1, curr[1] + 0), (curr[0] + 0, curr[1] + 1))
    if input[curr] == "|":
        con = ((curr[0] - 1, curr[1] + 0), (curr[0] + 1, curr[1] + 0))
    if input[curr] == "-":
        con = ((curr[0] + 0, curr[1] - 1), (curr[0] + 0, curr[1] + 1))
    # print("prev: ", prev, "  curr: ", curr, " curinp: ", input[curr], "  con: ", con)
    

    if not prev in con:
        raise Exception("Invalid last step") # last step was blocked by current pipe

    if len(con) != 2:
        raise Exception("Current step ain't going anywhere")
    
    next_position = con[(not con.index(prev))] 
    return curr, next_position 


main_loop = []
main_loop_char = []
prev_position = start
# The first step is taken manually

if TEST_INPUT:
    # current_position = (prev_position[0], prev_position[1]+1) # both test input first step (F start)
    current_position = (prev_position[0], prev_position[1]-1) # third test input first step (7 start)
else: 
    current_position = (prev_position[0]-1, prev_position[1]) # real input first step

# Walkthrough the entire loop step by step starting from S until we reach S again. The first step is hardcoded for our input
while current_position != start:
    main_loop.append(current_position)
    main_loop_char.append(input[current_position])
    prev_position, current_position = walkthrough(prev_position, current_position)

# print("main loop: ", main_loop)
# print("main loop ch ar: ", main_loop_char)
print("part1: ", ceil(len(main_loop) /2)) 

# part2:
main_loop.append(start)

for r, c in main_loop:
    mask[r, c] = 1

junk_pipe = (mask != 1) & (input != '.')
input[junk_pipe] = '.' # remove junk_pipe

# re-walkthrough the loop while marking 2s outside
for curr in main_loop[:-1]:
    mark_outside_of_loop(curr, input)

for i in range(len(mask)):
    for j in range(len(mask[0])):
        if mask[i,j] == 1:
            continue

        is_cornered = i - 1 < 0 or j - 1 < 0 or mask[i-1, j] == 2 or mask[i, j-1] == 2 or mask[i-1, j-1] == 2
        if is_cornered:
            mask[i, j] = 2

for i in range(len(mask))[::-1]:
    for j in range(len(mask[0]))[::-1]:
        if mask[i,j] == 1:
            continue

        is_cornered = i + 1 >= len(mask) or j + 1 >= len(mask[0]) or mask[i+1, j] == 2 or mask[i, j+1] == 2 or mask[i+1, j+1] == 2
        if is_cornered:
            mask[i, j] = 2

print("part2: ", np.sum(mask == 0))