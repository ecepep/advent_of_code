"""
https://adventofcode.com/2023/day/6
"""

# Time:        63     78     94     68
# Distance:   411   1274   2047   1035
input1 = [[63, 411],[78, 1274],[94, 2047],[68, 1035]]

def distance(hold, time):
    return hold * (time - hold)

# Part1
def get_win_factor(input):
    win_factor = 1
    for time, current_best in input:
        distances = [distance(hold, time) for hold in range(time)]
        win_factor *= sum([d > current_best for d in distances])
    return win_factor

print("part1: ", get_win_factor(input1))


# Part2
# The function associating holding time to distance in the race is monotically increasing and then decreasing with
# a maximum at the index in the center. We can then just do a [binary] search in both half of the indexes for the
# first index in the lower half with distances value above the current_best and the first index of the second half 
# with a value lower than the current best. I guess math could solve this problem trivially but math ain't as fun
# as programming.

time2 =  63789468
current_best2 = 411127420471035

def binary_search(lower_bound, upper_bound, increasing, func):
    if increasing:
        compare = lambda a, b: a > b
    else:
        compare =  lambda a, b: a < b

    while (True):
        index = lower_bound + (upper_bound - lower_bound)//2
        d = func(index)
        d_prev = func(index-1)
        
        if compare(d, current_best2) and not compare(d_prev, current_best2):
            break

        if compare(d, current_best2):
            upper_bound = index
        else:
            lower_bound = index
    return index

distance2 = lambda i : distance(i, time2)
start_index = binary_search(0, time2//2, True, distance2)
stop_index = binary_search(time2//2, time2, False, distance2)
print("part2: ", stop_index-start_index)
