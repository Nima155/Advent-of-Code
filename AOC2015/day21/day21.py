# player goes first
# attacker dmg = dmg_score - def_armor_score ==> always at least 1 dmg
# we can buy items
# 100 hp
#  one gun at most 2 rings and armor (is optional)
# Hit Points: 100
# Damage: 8
# Armor: 2
WEAPONS = { "dagger": [8, 4], "shortsword": [10, 5], 
            "warhammer": [25, 6], "longsword": [40, 7],
            "greataxe": [74, 8] }
ARMOR = {
    "leather": [13, 1],
    "chainmail": [31, 2],
    "splintmail": [53, 3],
    "bandedmail": [75, 4],
    "platemail": [102, 5]
}
RINGS = {
    "dmg1": [25, 1, 0],
    "dmg2": [50, 2, 0],
    "dmg3": [100, 3, 0],
    "def1": [20, 0, 1],
    "def2": [40, 0, 2],
    "def3": [80, 0, 3],
}


def fight():
    boss = { "hp": 100, "dmg": 8, "armor": 2}
    




print(fight())