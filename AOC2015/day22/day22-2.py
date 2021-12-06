from collections import deque
from copy import deepcopy
from RPG import RPG
from heapq import heappop, heappush

MAGIC_STORE = {
    # cost dmg hp armor mana turn
    "missile": [53, 4, 0, 0, 0, 0],
    "drain": [73, 2, 2, 0, 0, 0],
    "shield": [113, 0, 0, 7, 0, 6],
    "poison": [173, 3, 0, 0, 0, 6],
    "recharge": [229, 0, 0, 0, 101, 5]

}

    

# 737 too low!
def fight():
    queue = [RPG({ "hp": 50, "armor": 0, "mana": 500 }, { "hp": 58, "dmg": 9 })]
    
    
    mino = float("inf")
    while queue:
        # pe, en, activez, turn, spent_mana =  queue.popleft()
        rpg_stats = heappop(queue)
        
        rpg_stats.player["hp"] -= 1
        if rpg_stats.player["hp"] > 0:
            tbd = []
            
            # print(pe, en, activez)
            for name, (_, dmg, _, armor, mana, _) in rpg_stats.actives.items():
                rpg_stats.enemy["hp"] -= dmg
                if name == "shield":
                    rpg_stats.player["armor"] = armor if rpg_stats.actives[name][5] > 1 else 0
                rpg_stats.player["mana"] += mana
                rpg_stats.actives[name][5] -= 1
                if  rpg_stats.actives[name][5] <= 0: 
                    tbd.append(name)


            for n in tbd:
                del rpg_stats.actives[n]
            
            if rpg_stats.enemy["hp"] <= 0:
                # print(spent_mana)
                return rpg_stats.spent_mana
                # continue 

            if not rpg_stats.turn:
                for name, effect in MAGIC_STORE.items():
                    # print(effect)
                    if rpg_stats.player["mana"] >= effect[0]:
                        # print(name, effect)
                        new_rpg = deepcopy(rpg_stats)
                        
                        
                        if name in ["missile", "drain"]:
                            new_rpg.player["hp"] += effect[2]
                            new_rpg.player["mana"] -= effect[0]
                            new_rpg.enemy["hp"] -= effect[1]
                            new_rpg.turn ^= 1
                            new_rpg.spent_mana += effect[0]
                            heappush(queue, new_rpg)
                        elif name not in new_rpg.actives:
                            new_rpg.player["mana"] -= effect[0]
                            new_rpg.turn ^= 1
                            new_rpg.spent_mana += effect[0]
                            new_rpg.actives[name] = deepcopy(effect)
                            heappush(queue, new_rpg)
                            
                
            else:
                surplus = rpg_stats.enemy["dmg"] - rpg_stats.player["armor"] if rpg_stats.enemy["dmg"] > rpg_stats.player["armor"] else 1
                rpg_stats.player["hp"] -= surplus
                rpg_stats.turn ^= 1
                if rpg_stats.player["hp"] > 0:
                    heappush(queue, rpg_stats)
                    

    return mino


print(fight())