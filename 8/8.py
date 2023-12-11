"""
https://adventofcode.com/2023/day/8
"""

import json
import re
import numpy as np
import copy
import math

with open("./8/input.txt", "r") as f:
    input = f.readlines()

def get_graph(input):
    graph = {}
    for node in input:
        node = re.sub("[\\s\\(\\)]", "", node)
        root, leaves = node.split("=")
        leaves = leaves.split(",")
        graph[root]=leaves
    return graph

instructions = re.sub("[^LR]", "", input[0])
# print("instructions", instructions)
graph = get_graph(input[2:])
# print("lengraph ", len(graph))

def step(turn, graph, cur_root):    
    if turn == "L":
        cur_root = graph[cur_root][0]
    elif turn =="R":
        cur_root = graph[cur_root][1]
    else:
        raise Exception("unexpeted char: "+turn)
    return cur_root

# part1
if False:
    cur_root = "AAA"
    steps = 0
    while cur_root != "ZZZ":
        for turn in instructions:
            steps+=1
            
            cur_root = step(turn, graph, cur_root)

            if cur_root =="ZZZ":
                break

    print("part1: ", steps)

# #part 2
# We need to search for cycles XXA goes to some XXZ and at some point it will fall back on some previously met XXZ or XXA

def next_instruction(step):
    global instructions
    # if steps%10000 == 0:
    #     print("step: ", step)
    return instructions[steps%len(instructions)]

# search cycles 
nodes = [{"node": [k], "reach_at_step": [0]} for k in graph.keys() if k[-1] == 'A']
print("root at start ", nodes)

for node in nodes:
    steps = 0
    cur_root = node["node"][0] # start node XXA

    while (True):
        turn = next_instruction(steps)

        steps+=1
        
        cur_root = step(turn, graph, cur_root)
        # print("cur_root ", cur_root, "  is end ", is_end(cur_root))

        if cur_root.endswith("Z"):
            cycle = False
            for existing_root in node["node"]:
                if cur_root ==  existing_root:
                    cycle = True
            node["node"].append(cur_root)
            node["reach_at_step"].append(steps)

            if cycle:
                break

# Unlike my expectations all the cycle are direct (the first XXZ cycle with itself without any other XXZ in between) and XXA and XXZ have the same value in graph
print(json.dumps(nodes, indent=4))

steps_for_cycle = [node["reach_at_step"][1] for node in nodes]
part2 = math.lcm(*steps_for_cycle)
print("part2: ", part2)

