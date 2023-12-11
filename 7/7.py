"""
https://adventofcode.com/2023/day/7
"""

import re
from tqdm import tqdm
import pandas as pd

CARD_ORDER_1 = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
CARD_ORDER_2 = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A']
# CARD_ORDER.reverse()
# print(len(CARD_ORDER)) ==> base 13

"""
Convert chars into a base13 encoding of the hand
"""
def to_base13(hand, CARD_ORDER):
    hand_13 = []
    for card in hand:
        card_13 = CARD_ORDER.index(card)
        hand_13.append(card_13)
    return hand_13


def count_to_type(count):
    max_label = max(count)
    if max_label == 5:
        return 6 # five of a kind
    if max_label == 4:
        return 5 # four of a kind
    if max_label == 1:
        return 0 # high card
    
    count = [c if c != 1 else 0 for c in count]
    total = sum(count)
    if total == 5:
        return 4 # full house
    if total == 3:
        return 3 # 3 of a kind
    if total == 4:
        return 2 # 2 pairs
    if total == 2:
        return 1 # 1 pair
    raise Exception("unexpected")

"""
Count occurence of each card in hand
"""
def get_count(hand_13):
    count = [0]*13
    for card in hand_13:
        count[card] +=1
    return count

"""
Figure out hand type for part2
note: the best strategy is always to have the highest number of similar card (aqa J becomes the most common card)
"""
def type_rank_part2(hand_13):
    count = get_count(hand_13)

    J_cnt = count[0]
    count[0] = 0

    i_max = count.index(max(count))
    count[i_max] += J_cnt

    return count_to_type(count)

"""
Figure out hand type
"""
def type_rank_part1(hand_13):
    count = get_count(hand_13)    
    return count_to_type(count)
    
"""
Return the value of the hand in base 13
"""
def hand13_to_int(hand13):
    hand13.reverse() # first card tell the most
    score = 0
    for i in range(len(hand13)):
        score += pow(13, i) * hand13[i]
    return score

"""
We assign to each hand a score. Higher score means better hand.
To do so we convert all card to it's order in base 13. The type become the first and most important card.
The hand is then converted from base13 to an int (displayed in base 10)
"""
def score_hand(hand, is_part2):
    global CARD_ORDER_1
    global CARD_ORDER_2
    CARD_ORDER = CARD_ORDER_2 if is_part2 else CARD_ORDER_1

    hand_13 = to_base13(hand.hand, CARD_ORDER)

    if is_part2:
        type = type_rank_part2(hand_13)
    else:
        type = type_rank_part1(hand_13)

    # We add as a "first card" the type because it is what tells the most
    full_hand = [type] + hand_13
    score = hand13_to_int(full_hand)
    
    return score

def get_total_winning(df, is_part2):
    score_hand_part = lambda x : score_hand(x, is_part2)

    df["score"] = df.loc[:,["hand"]].apply(score_hand_part, axis=1)
    df = df. sort_values(by=["score"])
    ranked_bid = list(df["bid"])

    total_wining = 0
    for i in range(len(ranked_bid)):
        total_wining += ranked_bid[i]*(i+1)

    return total_wining

df = pd.read_csv("./7/input.csv")
df.columns = ["hand", "bid"]

print("part1: ", get_total_winning(df, False))
print("part2: ", get_total_winning(df, True))
