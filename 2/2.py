"""
https://adventofcode.com/2023/day/2
"""

import pandas as pd
import re

def parseInput():
    with open("./2/input.txt", "r") as f:
        input = f.readlines()

    data = []
    for game in input:
        gs = game.split(":")
        number = int(re.findall(r"\d+", gs[0])[0])

        sets = gs[-1].split(";")
        for set in sets:
            red = 0
            green = 0
            blue = 0
            for col in set.split(","):
                cnt = int(re.findall(r'\d+', col)[0])
                colname = re.findall(r'[a-z]+', col)[0]
                if colname == "red":
                    red = cnt
                if colname == "blue":
                    blue = cnt
                if colname == "green":
                    green = cnt
            
            data.append([number, red, green, blue])
    
    return pd.DataFrame(data= data, columns=['game','red','green','blue'])

sets = parseInput()

#part1
def is_valid(set):
    if (set.red > 12 or set.green > 13 or set.blue > 14) : 
        return False
    return True

sets["valid"] = sets.apply(is_valid, 1)
games = sets.groupby("game").min()["valid"].reset_index()
sum_valid = sum(games[games["valid"]]["game"])
print("part1: ", sum_valid)

#part2
def empower(game):
    return game.red * game.blue * game.green

min_cube = sets.groupby("game").max()
min_cube["power"] = min_cube.apply(empower, 1)
sum_min_cube = sum(min_cube["power"])
print("part2: ", sum_min_cube)