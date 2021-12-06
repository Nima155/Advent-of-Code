class RPG:
    def __init__(self, player_stats, enemy_stats):
        self.player = player_stats
        self.enemy = enemy_stats
        self.actives = {}
        self.turn = 0
        self.spent_mana = 0
    
    def __lt__(self, other):
        return self.spent_mana < other.spent_mana

    def __gt__(self, other):
        return self.spent_mana > other.spent_mana

    