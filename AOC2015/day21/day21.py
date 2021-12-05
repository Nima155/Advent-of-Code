from itertools import combinations
# player goes first
# attacker dmg = dmg_score - def_armor_score ==> always at least 1 dmg
# we can buy items
# 100 hp
#  one gun at most 2 rings and armor (is optional)
# Hit Points: 100
# Damage: 8
# Armor: 2
WEAPONS = { "dagger": [8, 4], "shortsword": [10, 5],  # cost and dmg must 1
            "warhammer": [25, 6], "longsword": [40, 7],
            "greataxe": [74, 8] }
ARMOR = {
    "leather": [13, 1], # cost and armor just one or zero
    "chainmail": [31, 2],
    "splintmail": [53, 3],
    "bandedmail": [75, 4],
    "platemail": [102, 5]
}
RINGS = {
    "dmg1": [25, 1, 0], # cost dmg armor up to 2
    "dmg2": [50, 2, 0],
    "dmg3": [100, 3, 0],
    "def1": [20, 0, 1],
    "def2": [40, 0, 2],
    "def3": [80, 0, 3],
}
combb = []
def customization_combs(inventory):
    if "weapon" not in inventory:
        for w in WEAPONS:
            customization_combs({**inventory, "weapon": WEAPONS[w]})
    if "armor" not in inventory:
        for a in ARMOR:
            customization_combs({**inventory, "armor": ARMOR[a]})
    if "rings" not in inventory:
        for i in range(1, 3):
            combs = list(combinations(RINGS.items(), i))
            for comb in combs:
                customization_combs({**inventory, "rings": comb})
    if "weapon" in inventory:
        combb.append(inventory)

    
            


def fight(player):
    boss = { "hp": 100, "dmg": 8, "armor": 2}
    # print("ring: ",player["rings"])
    ring_def, ring_dmg = 0, 0
    if "rings" in player:
        ring_def = sum(map(lambda x: x[1][2], filter(lambda x: x[0].startswith("def"), player["rings"])))
        ring_dmg = sum(map(lambda x: x[1][1], filter(lambda x: x[0].startswith("dmg"), player["rings"])))

    playerz = { "hp": 100, "armor":  (player["armor"][1] if "armor" in player else 0) + ring_def , "dmg": ring_dmg + player["weapon"][1] }
    turn = 0
    
    while playerz["hp"] > 0 and boss["hp"] > 0:
        if not turn:
            surplus = playerz["dmg"] - boss["armor"] if playerz["dmg"] > boss["armor"] else 1
            boss["hp"] -= surplus
        else:
            surplus = boss["dmg"] - playerz["armor"] if boss["dmg"] > playerz["armor"] else 1
            playerz["hp"] -= surplus
        turn ^= 1
    
    armor_cost = player["armor"][0] if "armor" in player else 0
    weapon_cost = player["weapon"][0]
    rings_cost = sum(map(lambda x: x[1][0], player["rings"])) if "rings" in player else 0
    
    return armor_cost + weapon_cost + rings_cost if playerz["hp"] > 0 else 10000000



def prepare():
    customization_combs({})
    
    cost = float("inf")
    for comb in combb:
        cost = min(fight(comb), cost)
    return cost

    




print(prepare())