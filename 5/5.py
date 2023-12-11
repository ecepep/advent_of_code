"""
https://adventofcode.com/2023/day/5
"""

import re
from tqdm import tqdm

with open("./5/input.txt", "r") as f:
    input = f.read().split("\n")

def parse_map(inputs):
    convertion_map = []
    for l in inputs:
        destination, source, range_len = re.findall("(\d+)", l) 
        convertion_map.append([int(destination), int(source), int(range_len)])
    return convertion_map

seeds = re.findall("(\d+)", input[0])
seeds = [int(s) for s in seeds]

seed_to_soil = parse_map(input[3:26])
soil_to_fertilizer = parse_map(input[28:59])
fertilizer_to_water = parse_map(input[61:87])
water_to_light = parse_map(input[89:129])
light_to_temp = parse_map(input[131:173])
temp_to_humidity = parse_map(input[175:210])
humidity_to_loc =  parse_map(input[212:249])

# part1
if False:
    """
    Convert a source to its destination
    """
    def convert(map, target_source):
        for destination, source, range_len in map:
            if target_source >= source and target_source < source + range_len:
                return destination + (target_source - source)
        return target_source

    locations = []
    for seed in seeds:
        soil = convert(seed_to_soil, seed)
        fertilizer = convert(soil_to_fertilizer, soil)
        water = convert(fertilizer_to_water, fertilizer)
        light = convert(water_to_light, water)
        temp = convert(light_to_temp, light)
        humidity = convert(temp_to_humidity, temp)
        loc = convert(humidity_to_loc, humidity)
        locations.append(loc)
    print("part1: ", min(locations))

# part2    
# This naiv approach is super poorly optimize. On my computer 25min to run.
# \Todo, optimize by not working in reverse but make an array of ranges. Each step concat for each current range in the array a set of ranges.

"""
convert a destination into a source
"""
def revert_convert(map, target_destination):
    for destination, source, range_len in map:
        if target_destination >= destination and target_destination < destination + range_len:
            return source + (target_destination - destination)
    return target_destination


def seed_exist(seed, seeds):
    for i in range(int(len(seeds)/2)):
        val = seeds[i*2]
        range_len =seeds[i*2+1]
        if seed >= val and seed < val+range_len:
            return True
    return False

# Test all location until we find one with a corresponding seed which do exist. If the first valid location has a too high value this loop won't return.
for i in tqdm(range(63179501)):
    humidity = revert_convert(humidity_to_loc, i)
    temp = revert_convert(temp_to_humidity, humidity)
    light = revert_convert(light_to_temp, temp)
    water = revert_convert(water_to_light, light)
    fertilizer = revert_convert(fertilizer_to_water, water)
    soil = revert_convert(soil_to_fertilizer, fertilizer)
    seed = revert_convert(seed_to_soil, soil)
    
    if seed_exist(seed,seeds):
        print("part2: ", i)
        break
